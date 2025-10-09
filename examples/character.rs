// This example shows how to create a controllable character with multiple animations.
//
// - We create a few animations for our character (idle, run, shoot) in a Startup system
// - We move the character with the keyboard and switch between animations in an Update system

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

// Let's use a custom resource to store our animations and access them across systems
#[derive(Resource)]
struct MyAnimations {
    idle: Handle<Animation>,
    run: Handle<Animation>,
    shoot: Handle<Animation>,
}

fn spawn_character(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // Create the animations

    let image = assets.load("character.png");

    let spritesheet = Spritesheet::new(&image, 8, 8);

    // Idle

    let idle_animation = spritesheet
        .create_animation()
        .add_horizontal_strip(0, 0, 5)
        .build();

    let idle_animation_handle = animations.add(idle_animation);

    // Run

    let run_animation = spritesheet.create_animation().add_row(3).build();

    let run_animation_handle = animations.add(run_animation);

    // Shoot

    let shoot_animation = spritesheet
        .create_animation()
        .add_horizontal_strip(0, 5, 5)
        .build();

    let shoot_animation_handle = animations.add(shoot_animation);

    // Store the animations as a resource

    commands.insert_resource(MyAnimations {
        idle: idle_animation_handle.clone(),
        run: run_animation_handle,
        shoot: shoot_animation_handle,
    });

    // Spawn the character

    let sprite = spritesheet
        .with_size_hint(768, 768)
        .sprite(&mut atlas_layouts);

    commands.spawn((sprite, SpritesheetAnimation::new(idle_animation_handle)));
}

// Component to mark that a character is currently shooting
#[derive(Component)]
struct Shooting;

fn control_character(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    character: Single<(
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

    let (entity, mut sprite, mut animation, mut transform, shooting) = character.into_inner();

    // If they're shooting, do nothing and wait for the animation to end

    if shooting.is_none() {
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
