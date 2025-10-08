// This example shows how to use this plugin in a bevy app without bevy_render.
//
// For example for a headless server with MinimalPlugins, while still using the animations events.

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

fn spawn_animation(mut commands: Commands, mut library: ResMut<AnimationLibrary>) {
    commands.spawn(Camera2d);

    // Create a clip that references some frames from a spritesheet

    let spritesheet = Spritesheet::new(8, 8);

    let clip = Clip::from_frames(spritesheet.row(3));

    let clip_id = library.register_clip(clip);

    // Create an animation that uses the clip

    let animation = Animation::from_clip(clip_id);

    let animation_id = library.register_animation(animation);

    // Name the animation to retrieve it from other systems

    library.name_animation(animation_id, "walk").unwrap();

    // Spawn an entity with a SpritesheetAnimation component that references our animation
    //
    // We dont even need a Sprite since its only used for bevy_render (and we aren't rendering anything)

    commands.spawn(SpritesheetAnimation::from_id(animation_id));
}

fn log_animations_events(mut events: MessageReader<AnimationEvent>) {
    for event in events.read() {
        println!("{:?}", event);
    }
}
