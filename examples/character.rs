// This example shows how to create a controllable character with multiple animations.
//
// - We create a few animations for our character (idle, run, shoot) in a Startup system
// - We move the character with the keyboard and switch between animations in an Update system

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

animation_set!(MyAnimations [
    anim idle,
    anim run,
    anim shoot
]);

fn spawn_character(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
) {
    commands.spawn(Camera2d);

    // Create the animations

    let spritesheet = Spritesheet::new(8, 8);

    // Idle

    let idle_clip = Clip::from_frames(spritesheet.horizontal_strip(0, 0, 5));

    let idle_animation = Animation::from_clip(idle_clip);

    let idle_animation_handle = animations.add(idle_animation);

    // Run

    let run_clip = Clip::from_frames(spritesheet.row(3));

    let run_animation = Animation::from_clip(run_clip);

    let run_animation_handle = animations.add(run_animation);

    // Shoot

    let shoot_clip = Clip::from_frames(spritesheet.horizontal_strip(0, 5, 5));

    let shoot_animation = Animation::from_clip(shoot_clip);

    let shoot_animation_handle = animations.add(shoot_animation);

    // Store the animation set as a resource

    commands.insert_resource(MyAnimations {
        idle: idle_animation_handle.clone(),
        run: run_animation_handle.clone(),
        shoot: shoot_animation_handle.clone(),
    });

    // Spawn the character

    let image = assets.load("character.png");

    let atlas = TextureAtlas {
        layout: atlas_layouts.add(spritesheet.atlas_layout(96, 96)),
        ..default()
    };

    commands.spawn((
        Sprite::from_atlas_image(image, atlas),
        SpritesheetAnimation::new(idle_animation_handle),
    ));
}

// Component to mark that a character is currently shooting
#[derive(Component)]
struct Shooting;

fn control_character(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut characters: Query<(
        Entity,
        &mut Sprite,
        &mut SpritesheetAnimation,
        &mut Transform,
        Option<&Shooting>,
    )>,
    my_animations: Res<MyAnimations>,
    mut messages: MessageReader<AnimationEvent>,
) {
    // Control the character with the keyboard

    const CHARACTER_SPEED: f32 = 150.0;

    for (entity, mut sprite, mut animation, mut transform, shooting) in &mut characters {
        // If they're shooting, do nothing and wait for the animation to end

        if shooting.is_some() {
            continue;
        }

        // Shoot with the spacebar
        if keyboard.pressed(KeyCode::Space) {
            // Set the animation

            animation.switch(my_animations.shoot.clone());

            // Add a Shooting component

            commands.entity(entity).insert(Shooting);
        }
        // Run with the arrows
        else if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::ArrowRight) {
            // Set the animation
            //
            // Only if not already running as we don't want to reset the animation in that case

            if animation.animation != my_animations.run {
                animation.switch(my_animations.run.clone());
            }

            // Move the entity and flip it horizontally depending on the direction

            let translation = Vec3::X * time.delta_secs() * CHARACTER_SPEED;

            if keyboard.pressed(KeyCode::ArrowLeft) {
                transform.translation -= translation;
                sprite.flip_x = true;
            } else {
                transform.translation += translation;
                sprite.flip_x = false;
            }
        }
        // Idle
        else {
            // Set the animation
            //
            // Only if not already idle as we don't want to reset the animation in that case

            if animation.animation != my_animations.idle {
                animation.switch(my_animations.idle.clone());
            }
        }
    }

    // Remove the Shooting component when the shooting animation ends
    //
    // We use animation events to detect when this happens.
    // Check out the `events` examples for more details.

    for event in messages.read() {
        if let AnimationEvent::AnimationRepetitionEnd {
            entity, animation, ..
        } = event
            && animation == &my_animations.shoot
        {
            commands.entity(*entity).remove::<Shooting>();
        }
    }
}
