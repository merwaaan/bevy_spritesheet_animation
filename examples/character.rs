// This example shows how to create controllable character with multiple animations.
//
// - We'll create a few animations for our character (idle, run, shoot) in a setup system
// - We'll move the character with the keyboard and switch between animations in another system

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin,
        ))
        .add_systems(Startup, spawn_character)
        .add_systems(Update, control_character)
        .run();
}

fn spawn_character(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
    assets: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

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

    let image = assets.load("character.png");

    let atlas = TextureAtlas {
        layout: atlas_layouts.add(Spritesheet::new(8, 8).atlas_layout(96, 96)),
        ..default()
    };

    commands.spawn((
        Sprite::from_atlas_image(image, atlas),
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
    mut messages: MessageReader<AnimationMessage>,
    mut characters: Query<(
        Entity,
        &mut Sprite,
        &mut SpritesheetAnimation,
        &mut Transform,
        Option<&Shooting>,
    )>,
) {
    // Control the character with the keyboard

    const CHARACTER_SPEED: f32 = 150.0;

    for (entity, mut sprite, mut animation, mut transform, shooting) in &mut characters {
        // Except if they're shooting, in which case we wait for the animation to end

        if shooting.is_some() {
            continue;
        }

        // Shoot
        if keyboard.pressed(KeyCode::Space) {
            // Set the animation

            if let Some(shoot_animation_id) = library.animation_with_name("shoot") {
                animation.switch(shoot_animation_id);
            }

            // Add a Shooting component

            commands.entity(entity).insert(Shooting);
        }
        // Move left
        else if keyboard.pressed(KeyCode::ArrowLeft) {
            // Set the animation

            if let Some(run_animation_id) = library.animation_with_name("run")
                && animation.animation_id != run_animation_id
            {
                animation.switch(run_animation_id);
            }

            // Move

            transform.translation -= Vec3::X * time.delta_secs() * CHARACTER_SPEED;
            sprite.flip_x = true;
        }
        // Move right
        else if keyboard.pressed(KeyCode::ArrowRight) {
            // Set the animation

            if let Some(run_animation_id) = library.animation_with_name("run")
                && animation.animation_id != run_animation_id
            {
                animation.switch(run_animation_id);
            }

            // Move

            transform.translation += Vec3::X * time.delta_secs() * CHARACTER_SPEED;
            sprite.flip_x = false;
        }
        // Idle
        else {
            // Set the animation

            if let Some(idle_animation_id) = library.animation_with_name("idle")
                && animation.animation_id != idle_animation_id
            {
                animation.switch(idle_animation_id);
            }
        }
    }

    // Remove the Shooting component when the shooting animation ends
    //
    // We use animation messages to detect when this happens.
    // Check out the `messages` examples for more details.

    for message in messages.read() {
        if let AnimationMessage::AnimationRepetitionEnd {
            entity,
            animation_id,
            ..
        } = message
            && library.is_animation_name(*animation_id, "shoot")
        {
            commands.entity(*entity).remove::<Shooting>();
        }
    }
}
