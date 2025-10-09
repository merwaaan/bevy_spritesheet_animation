// This example shows how to use this plugin in a headless bevy app without bevy_render.
//
// For example this could run on a game server with MinimalPlugins, while still using the animations events.

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, SpritesheetAnimationPlugin))
        .add_systems(Startup, spawn_animation)
        .add_systems(Update, log_animations_events)
        .run();
}

fn spawn_animation(mut commands: Commands, mut animations: ResMut<Assets<Animation>>) {
    // Create an animation
    //
    // We can pass a dummy image as we aren't rendering anything

    let dummy_image = Handle::default();

    let spritesheet = Spritesheet::new(&dummy_image, 8, 8);

    let animation = spritesheet
        .create_animation()
        .add_indices([0, 1, 2])
        .build();

    let animation_handle = animations.add(animation);

    // Spawn an entity with a SpritesheetAnimation component that references our animation
    //
    // We don't even need a Sprite component as we aren't rendering anything

    commands.spawn(SpritesheetAnimation::new(animation_handle));
}

fn log_animations_events(mut events: MessageReader<AnimationEvent>) {
    for event in events.read() {
        println!("{:?}", event);
    }
}
