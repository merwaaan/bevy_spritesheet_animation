// This example shows how to use events to be notified when animations reach points of interest.
//
// - We'll create a few animations for our character (run, shoot) in a setup system
//
// - We'll add markers on specific frames of our animations:
//      - when a character's foot touches the ground
//      - when the character shoots their gun
//
// - We'll setup a UI that shows all the animation events that exist.
//   Events received at each update will be highlighted.
//
// - We'll spawn special effects when a marker is hit:
//      - A bullet when the character shoots their gun
//      - A shockwave when their feet hit the ground

use std::collections::HashSet;

use bevy::{color, prelude::*};
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin,
        ))
        .insert_resource(TimeScale(1.0))
        .add_systems(Startup, (spawn_character, create_ui))
        .add_systems(
            Update,
            (
                show_triggered_events,
                fade_triggered_events,
                spawn_visual_effects,
                animate_bullets,
                animate_footsteps,
                update_on_keypress,
            ),
        )
        .run();
}

// Let's use a custom resource to store our markers and access them across systems
#[derive(Resource)]
struct MyMarkers {
    bullet_out: Marker,
    foot_touches_ground: Marker,
}

fn spawn_character(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // Create the animation

    let image = assets.load("character.png");

    let spritesheet = Spritesheet::new(&image, 8, 8);

    let foot_touches_ground_marker = Marker::new();
    let bullet_out_marker = Marker::new();

    let animation = spritesheet
        .create_animation()
        .set_repetitions(AnimationRepeat::Times(3))
        // Clip 1: run
        //
        // The character's foot touches the ground on frame 1 and then again on frame 5
        .add_row(3)
        .set_clip_repetitions(4)
        .add_clip_marker(foot_touches_ground_marker, 1)
        .add_clip_marker(foot_touches_ground_marker, 5)
        // Clip 2: shoot
        //
        // The character shoots their gun on frame 1
        .start_clip()
        .add_horizontal_strip(0, 5, 5)
        .add_clip_marker(bullet_out_marker, 1)
        .build();

    let animation_handle = animations.add(animation);

    // Store the markers as a resource

    commands.insert_resource(MyMarkers {
        bullet_out: bullet_out_marker,
        foot_touches_ground: foot_touches_ground_marker,
    });

    // Spawn the character

    let sprite = spritesheet
        .with_size_hint(768, 768)
        .sprite(&mut atlas_layouts);

    commands.spawn((sprite, SpritesheetAnimation::new(animation_handle)));
}

fn create_ui(mut commands: Commands) {
    commands
        // Full-screen container
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            // Help text

            parent
                .spawn(Node {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                })
                .with_child((Text::new("SPACE: toggle slow motion\nR: reset animation"),));

            // Events

            let mut add_event = |event_type: EventType| {
                parent
                    // Row
                    .spawn(Node {
                        margin: UiRect::all(Val::Px(10.0)),
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|parent| {
                        // Colored square

                        parent.spawn((
                            Node {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                margin: UiRect::right(Val::Px(10.0)),
                                ..default()
                            },
                            BorderColor::all(color::palettes::css::GRAY),
                            BackgroundColor(color::palettes::css::DEEP_PINK.with_alpha(0.0).into()),
                            event_type,
                        ));

                        // Event name

                        parent.spawn((Text(format!("{event_type:?}")), Label));
                    });
            };

            add_event(EventType::MarkerHit);
            add_event(EventType::ClipRepetitionEnd);
            add_event(EventType::ClipEnd);
            add_event(EventType::RepetitionEnd);
            add_event(EventType::End);
        });
}

// Component attached to a UI square to be highlighted when the given event type is received
#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
enum EventType {
    MarkerHit,
    ClipRepetitionEnd,
    ClipEnd,
    RepetitionEnd,
    End,
}

fn show_triggered_events(
    mut messages: MessageReader<AnimationEvent>,
    mut squares: Query<(&mut BackgroundColor, &EventType)>,
) {
    // Collect the events that were just received

    let mut triggered_events: HashSet<EventType> = HashSet::new();

    for event in messages.read() {
        match event {
            AnimationEvent::MarkerHit { .. } => {
                triggered_events.insert(EventType::MarkerHit);
            }
            AnimationEvent::ClipRepetitionEnd { .. } => {
                triggered_events.insert(EventType::ClipRepetitionEnd);
            }
            AnimationEvent::ClipEnd { .. } => {
                triggered_events.insert(EventType::ClipEnd);
            }
            AnimationEvent::AnimationRepetitionEnd { .. } => {
                triggered_events.insert(EventType::RepetitionEnd);
            }
            AnimationEvent::AnimationEnd { .. } => {
                triggered_events.insert(EventType::End);
            }
        }
    }

    // Set full alpha for the squares which events were just received

    for (mut color, event_type) in &mut squares {
        if triggered_events.contains(event_type) {
            color.0.set_alpha(1.0);
        }
    }
}

fn fade_triggered_events(time: Res<Time>, mut squares: Query<&mut BackgroundColor>) {
    const FADE_SPEED: f32 = 3.0;

    for mut color in &mut squares {
        let new_alpha = color.0.alpha() - time.delta_secs() * FADE_SPEED;
        color.0.set_alpha(new_alpha);
    }
}

// Spawns footsteps & bullets when the marked frames are played
fn spawn_visual_effects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut messages: MessageReader<AnimationEvent>,
    my_markers: Res<MyMarkers>,
) {
    for event in messages.read() {
        if let AnimationEvent::MarkerHit { marker, .. } = event {
            // Spawn a shockwave at each footstep
            if marker == &my_markers.foot_touches_ground {
                commands.spawn((
                    Mesh2d(meshes.add(Circle { radius: 1.0 })),
                    MeshMaterial2d(materials.add(ColorMaterial::default())),
                    Transform::from_xyz(0.0, -30.0, -1.0),
                    Footstep,
                ));
            }
            // Spawn a bullet when firing
            else if marker == &my_markers.bullet_out {
                commands.spawn((
                    Mesh2d(meshes.add(Circle { radius: 3.0 })),
                    MeshMaterial2d(materials.add(Color::from(color::palettes::css::YELLOW))),
                    Transform::from_xyz(50.0, 15.0, 0.0),
                    Bullet,
                ));
            }
        }
    }
}

#[derive(Component)]
struct Bullet;

fn animate_bullets(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform), With<Bullet>>,
) {
    const MOVE_SPEED: f32 = 800.0;

    for (entity, mut transform) in &mut bullets {
        // Move horizontally

        transform.translation.x += time.delta_secs() * time_scale.0 * MOVE_SPEED;

        // Despawn when far away

        if transform.translation.x > 5000.0 {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
struct Footstep;

fn animate_footsteps(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut footsteps: Query<(Entity, &mut Transform, &MeshMaterial2d<ColorMaterial>), With<Footstep>>,
) {
    const GROW_SPEED: f32 = 100.0;
    const FADE_SPEED: f32 = 4.0;

    for (entity, mut transform, material_handle) in &mut footsteps {
        // Grow

        transform.scale += time.delta_secs() * time_scale.0 * Vec3::splat(GROW_SPEED);

        // Fade away

        if let Some(material) = materials.get_mut(material_handle) {
            material
                .color
                .set_alpha(material.color.alpha() - time.delta_secs() * time_scale.0 * FADE_SPEED);

            // Despawn when transparent

            if material.color.alpha() <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

#[derive(Resource)]
struct TimeScale(f32);

fn update_on_keypress(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut time_scale: ResMut<TimeScale>,
    mut sprite: Single<&mut SpritesheetAnimation>,
) {
    // Space: toggle slomo

    if keyboard.just_pressed(KeyCode::Space) {
        // Update our global time scale (used by bullets and footsteps)

        if time_scale.0 >= 1.0 {
            time_scale.0 = 0.1;
        } else {
            time_scale.0 = 1.0;
        }

        // Update the sprite's playback speed

        sprite.speed_factor = time_scale.0;
    }

    // R: reset the animation

    if keyboard.just_pressed(KeyCode::KeyR) {
        sprite.reset();
    }
}
