// This example shows how to react to animations reaching points of interest with events.
//
// - We'll create a few animations for our character (idle, run, shoot) in a setup system
//
// - We'll add markers on interesting frames of our animations:
//      - when a character's foot touches the ground
//      - when the character shoots their gun
//
// - We'll setup a UI that shows all the animation events that exist.
//   Events received at each update will be highlighted.
//
// - We'll spawn special effects when a marker is hit:
//      - A bullet when the character shoots their gun
//      - A shockwave when their feet hit the ground

#[path = "./common/mod.rs"]
pub mod common;

use std::collections::HashSet;

use bevy::{
    color::palettes::css::{DEEP_PINK, GRAY, YELLOW},
    prelude::*,
};
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin,
        ))
        .add_systems(Startup, (spawn_character, create_ui))
        .add_systems(
            Update,
            (
                show_triggered_events,
                spawn_visual_effects,
                animate_bullets,
                animate_footsteps,
            ),
        )
        .run();
}

fn spawn_character(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
    assets: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // Create a running clip

    let spritesheet = Spritesheet::new(8, 8);

    let foot_touches_ground_marker = library.new_marker();

    library
        .name_marker(foot_touches_ground_marker, "foot touches ground")
        .unwrap();

    let run_clip = Clip::from_frames(spritesheet.row(3))
        .with_repetitions(4)
        // The character's foot touches the ground on frame 1...
        .with_marker(foot_touches_ground_marker, 1)
        // ... and then again on frame 5
        .with_marker(foot_touches_ground_marker, 5);

    let run_clip_id = library.register_clip(run_clip);

    // Create a shooting clip

    let bullet_out_marker = library.new_marker();

    library
        .name_marker(bullet_out_marker, "bullet goes out")
        .unwrap();

    let shoot_clip = Clip::from_frames(spritesheet.horizontal_strip(0, 5, 5))
        // The character shoots their gun on frame 1
        .with_marker(bullet_out_marker, 1);

    let shoot_clip_id = library.register_clip(shoot_clip);

    // Create the final animation

    let animation = Animation::from_clips([run_clip_id, shoot_clip_id]);

    let animation_id = library.register_animation(animation);

    // Spawn a sprite using the animation

    let image = assets.load("character.png");

    let atlas = TextureAtlas {
        layout: atlas_layouts.add(Spritesheet::new(8, 8).atlas_layout(96, 96)),
        ..default()
    };

    commands.spawn((
        Sprite::from_atlas_image(image, atlas),
        SpritesheetAnimation::from_id(animation_id),
    ));
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
                            event_type,
                        ));

                        // Event name

                        parent.spawn((
                            Text(format!("{event_type:?}")),
                            TextFont::from_font_size(30.0),
                            Label,
                        ));
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
    mut events: EventReader<AnimationEvent>,
    mut squares: Query<(&mut BackgroundColor, &EventType)>,
) {
    // Collect the events that were just received

    let mut triggered_events: HashSet<EventType> = HashSet::new();

    for event in events.read() {
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

    // Color the squares for the events that were just received

    for (mut color, event_type) in &mut squares {
        if triggered_events.contains(event_type) {
            color.0 = Color::from(DEEP_PINK);
        } else {
            color.0 = Color::from(GRAY);
        }
    }
}

// Spawns footsteps & bullets when the marked frames are played
fn spawn_visual_effects(
    mut commands: Commands,
    library: Res<AnimationLibrary>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<AnimationEvent>,
) {
    for event in events.read() {
        if let AnimationEvent::MarkerHit { marker_id, .. } = event {
            // Spawn a shockwave at each footstep

            if library.is_marker_name(*marker_id, "foot touches ground") {
                commands.spawn((
                    Mesh2d(meshes.add(Circle { radius: 1.0 })),
                    MeshMaterial2d(materials.add(ColorMaterial::default())),
                    Transform::from_xyz(0.0, -30.0, -1.0),
                    Footstep,
                ));
            }

            // Spawn a bullet when firing

            if library.is_marker_name(*marker_id, "bullet goes out") {
                commands.spawn((
                    Mesh2d(meshes.add(Circle { radius: 3.0 })),
                    MeshMaterial2d(materials.add(Color::from(YELLOW))),
                    Transform::from_xyz(20.0, 15.0, 0.0),
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
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform), With<Bullet>>,
) {
    for (entity, mut transform) in &mut bullets {
        // Move horizontally

        transform.translation.x += time.delta_secs() * 400.0;

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
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut footsteps: Query<(Entity, &mut Transform, &MeshMaterial2d<ColorMaterial>), With<Footstep>>,
) {
    for (entity, mut transform, material_handle) in &mut footsteps {
        // Grow

        transform.scale += time.delta_secs() * Vec3::splat(100.0);

        // Fade away

        if let Some(material) = materials.get_mut(material_handle) {
            material
                .color
                .set_alpha(material.color.alpha() - time.delta_secs() * 4.0);

            // Despawn when transparent

            if material.color.alpha() <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}
