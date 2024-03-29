// This example illustrates how to react to animations reaching points of interest with events.
//
// - We'll create a few animations for our character in a setup system (idle, run, shoot)
//
// - We'll add markers on interesting frames of our animations:
//      - when a character's foot touches the ground
//      - when the character shoots their gun
//
// - We'll setup a UI that shows all the animation events that exist.
//   The events received at each update will be highlighted.
//
// - We'll spawn special effects when a marker is hit:
//      - A bullet when the character shoots their gun
//      - A shockwave when their feet hit the ground

#[path = "./common/mod.rs"]
pub mod common;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_spritesheet_animation::prelude::*;
use std::collections::HashSet;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, show_triggered_events)
        .add_systems(Update, spawn_visual_effects)
        .add_systems(Update, animate_bullets)
        .add_systems(Update, animate_footsteps)
        .run();
}

fn setup(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<SpritesheetLibrary>,
    assets: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load assets for the sprite

    let texture = assets.load("character.png");

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(96.0, 96.0),
        8,
        8,
        None,
        None,
    ));

    // Create a running clip

    let foot_touches_ground_marker = library.new_marker();

    library
        .name_marker(foot_touches_ground_marker, "foot touches ground")
        .unwrap();

    let run_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).row(3))
            // The character's foot touches the ground on frame 1...
            .add_marker(foot_touches_ground_marker, 1)
            // ... and then on frame 5
            .add_marker(foot_touches_ground_marker, 5);
    });

    // Create a shooting clip

    let bullet_out_marker = library.new_marker();

    library
        .name_marker(bullet_out_marker, "bullet goes out")
        .unwrap();

    let shoot_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).horizontal_strip(0, 5, 5))
            // The character shoots their gun on frame 1
            .add_marker(bullet_out_marker, 1);
    });

    // Assemble the two clips into an animation

    let animation_id = library.new_animation(|animation| {
        let mut run_stage = AnimationStage::from_clip(run_clip_id);
        run_stage.set_repeat(4);

        animation
            .add_stage(run_stage)
            .add_stage(shoot_clip_id.into());
    });

    // Spawn a sprite using the animation

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout,
                ..default()
            },
            ..default()
        },
        SpritesheetAnimation::from_id(animation_id),
    ));

    // Setup the UI

    commands
        // Full-screen container
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let mut add_event = |event_type: EventType| {
                parent
                    // Row
                    .spawn(NodeBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        // Colored square

                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                ..default()
                            },
                            event_type,
                        ));

                        // Event name

                        parent.spawn((
                            TextBundle::from_section(
                                format!("{event_type:?}"),
                                TextStyle {
                                    font_size: 30.0,
                                    ..default()
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::left(Val::Px(10.0)),
                                ..default()
                            }),
                            Label,
                        ));
                    });
            };

            add_event(EventType::MarkerHit);
            add_event(EventType::ClipCycleEnd);
            add_event(EventType::ClipEnd);
            add_event(EventType::CycleEnd);
            add_event(EventType::End);
        });
}

// Component attached to the square to be highlighted when the same event is received
#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
enum EventType {
    MarkerHit,
    ClipCycleEnd,
    ClipEnd,
    CycleEnd,
    End,
}

// Updates the colored squares
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
            AnimationEvent::ClipCycleEnd { .. } => {
                triggered_events.insert(EventType::ClipCycleEnd);
            }
            AnimationEvent::ClipEnd { .. } => {
                triggered_events.insert(EventType::ClipEnd);
            }
            AnimationEvent::AnimationCycleEnd { .. } => {
                triggered_events.insert(EventType::CycleEnd);
            }
            AnimationEvent::AnimationEnd { .. } => {
                triggered_events.insert(EventType::End);
            }
        }
    }

    // Color the squares for the events that were just received

    for (mut color, event_type) in &mut squares {
        if triggered_events.contains(event_type) {
            color.0 = Color::PINK;
        } else {
            color.0 = Color::GRAY;
        }
    }
}

// Spawns footsteps & bullets when the marked frames are played
fn spawn_visual_effects(
    mut commands: Commands,
    library: Res<SpritesheetLibrary>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<AnimationEvent>,
) {
    for event in events.read() {
        match event {
            AnimationEvent::MarkerHit { marker_id, .. } => {
                // Spawn a shockwave at each footstep

                if library.is_marker_name(*marker_id, "foot touches ground") {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: Mesh2dHandle(meshes.add(Circle { radius: 1.0 })),
                            material: materials.add(Color::WHITE),
                            transform: Transform::from_xyz(0.0, -30.0, -1.0),
                            ..default()
                        },
                        Footstep,
                    ));
                }

                // Spawn a bullet when firing

                if library.is_marker_name(*marker_id, "bullet goes out") {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: Mesh2dHandle(meshes.add(Circle { radius: 3.0 })),
                            material: materials.add(Color::YELLOW),
                            transform: Transform::from_xyz(20.0, 15.0, 0.0),
                            ..default()
                        },
                        Bullet,
                    ));
                }
            }
            _ => (),
        }
    }
}

#[derive(Component)]
struct Bullet;

// Updates the bullets
fn animate_bullets(
    time: Res<Time>,
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform), With<Bullet>>,
) {
    for (entity, mut transform) in &mut bullets {
        // Move horizontally

        transform.translation.x += time.delta_seconds() * 400.0;

        // Despawn when far away

        if transform.translation.x > 5000.0 {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
struct Footstep;

// Updates the footsteps
fn animate_footsteps(
    time: Res<Time>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut footsteps: Query<(Entity, &mut Transform, &Handle<ColorMaterial>), With<Footstep>>,
) {
    for (entity, mut transform, material_handle) in &mut footsteps {
        // Grow

        transform.scale += time.delta_seconds() * Vec3::splat(100.0);

        // Fade away

        if let Some(material) = materials.get_mut(material_handle) {
            material
                .color
                .set_a(material.color.a() - time.delta_seconds() * 4.0);

            // Despawn when transparent

            if material.color.a() <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}
