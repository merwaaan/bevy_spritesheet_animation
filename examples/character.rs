// This example shows how to create controllable character with multiple animations.
//
// - We'll create a few animations for our character (idle, run, shoot) in a setup system
// - We'll move the character with the keyboard and switch to the appropriate animation in another system

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, control_character)
        .run();
}

fn setup(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
    assets: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    // Create the animations

    let spritesheet = Spritesheet::new(8, 8);

    // Idle

    let idle_clip = Clip::from_frames(spritesheet.horizontal_strip(0, 0, 5));

    let idle_clip_id = library.register_clip(idle_clip);

    let idle_animation = Animation::from_clip(idle_clip_id);

    let idle_animation_id = library.register_animation(idle_animation);

    library.name_animation(idle_animation_id, "idle").unwrap();

    // Run

    let run_clip = Clip::from_frames(spritesheet.row(3));

    let run_clip_id = library.register_clip(run_clip);

    let run_animation = Animation::from_clip(run_clip_id);

    let run_animation_id = library.register_animation(run_animation);

    library.name_animation(run_animation_id, "run").unwrap();

    // Shoot

    let shoot_clip = Clip::from_frames(spritesheet.horizontal_strip(0, 5, 5));

    let shoot_clip_id = library.register_clip(shoot_clip);

    let shoot_animation = Animation::from_clip(shoot_clip_id);

    let shoot_animation_id = library.register_animation(shoot_animation);

    library.name_animation(shoot_animation_id, "shoot").unwrap();

    // Spawn the character

    let texture = assets.load("character.png");

    let layout = atlas_layouts.add(spritesheet.atlas_layout(96, 96));

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        TextureAtlas {
            layout,
            ..default()
        },
        SpritesheetAnimation::from_id(idle_animation_id),
    ));
}

// Component to check if a character is currently shooting
#[derive(Component)]
struct Shooting;

fn control_character(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    library: Res<AnimationLibrary>,
    mut events: EventReader<AnimationEvent>,
    mut characters: Query<(
        Entity,
        &mut Transform,
        &mut Sprite,
        &mut SpritesheetAnimation,
        Option<&Shooting>,
    )>,
) {
    // Control the character with the keyboard

    const CHARACTER_SPEED: f32 = 150.0;

    for (entity, mut transform, mut sprite, mut animation, shooting) in &mut characters {
        // Except if they're shooting, in which case we wait for the animation to end

        if shooting.is_some() {
            continue;
        }

        // Shoot
        if keyboard.pressed(KeyCode::Space) {
            // Set the animation

            if let Some(id) = library.animation_with_name("shoot") {
                animation.animation_id = id;
            }

            // Add a Shooting component

            commands.entity(entity).insert(Shooting);
        }
        // Move left
        else if keyboard.pressed(KeyCode::ArrowLeft) {
            // Set the animation

            if let Some(id) = library.animation_with_name("run") {
                animation.animation_id = id;
            }

            // Move

            transform.translation -= Vec3::X * time.delta_seconds() * CHARACTER_SPEED;
            sprite.flip_x = true;
        }
        // Move right
        else if keyboard.pressed(KeyCode::ArrowRight) {
            // Set the animation

            if let Some(id) = library.animation_with_name("run") {
                animation.animation_id = id;
            }

            // Move

            transform.translation += Vec3::X * time.delta_seconds() * CHARACTER_SPEED;
            sprite.flip_x = false;
        }
        // Idle
        else {
            // Set the animation

            if let Some(id) = library.animation_with_name("idle") {
                animation.animation_id = id;
            }
        }
    }

    // Remove the Shooting component when the shooting animation ends
    //
    // We use animation events to detect when this happens.
    // Check out the `events` examples for more details.

    for event in events.read() {
        match event {
            AnimationEvent::AnimationRepetitionEnd {
                entity,
                animation_id,
            } => {
                if library.is_animation_name(*animation_id, "shoot") {
                    commands.entity(*entity).remove::<Shooting>();
                }
            }
            _ => (),
        }
    }
}
