// This example shows how to use this plugin in a headless bevy app without bevy_render.
//
// For example this could run on a game server with MinimalPlugins, while still using the animations events.

#[path = "./common/mod.rs"]
pub mod common;

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
    commands.spawn(Camera2d);

    // Create an animation

    let spritesheet = Spritesheet::new(8, 8);

    let clip = Clip::from_frames(spritesheet.row(3));

    let animation = Animation::from_clip(clip);

    let animation_handle = animations.add(animation);

    // Spawn an entity with a SpritesheetAnimation component that references our animation
    //
    // We don't even need a Sprite since we aren't rendering anything

    commands.spawn(SpritesheetAnimation::new(animation_handle));
}

fn log_animations_events(mut events: MessageReader<AnimationEvent>) {
    for event in events.read() {
        println!("{:?}", event);
    }
}
