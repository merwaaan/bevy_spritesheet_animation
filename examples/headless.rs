// This example shows how to use this plugin in a bevy app without bevy_render.
//
// For example for a headless server with MinimalPlugins, while still using the animations events.

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            SpritesheetAnimationPlugin { enable_3d: false },
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_animations_events)
        .run();
}

fn setup(mut commands: Commands, mut library: ResMut<AnimationLibrary>) {
    commands.spawn(Camera2dBundle::default());

    // Create a clip that references some frames from a spritesheet

    let spritesheet = Spritesheet::new(8, 8);

    let clip = Clip::from_frames(spritesheet.row(3));

    let clip_id = library.register_clip(clip);

    // Create an animation that uses the clip

    let animation = Animation::from_clip(clip_id);

    let animation_id = library.register_animation(animation);

    // Name the animation to retrieve it from other systems

    library.name_animation(animation_id, "walk").unwrap();

    // Spawn a sprite with Bevy's built-in SpriteBundle

    commands.spawn((
        // We dont even need a TextureAtlas since its only used for bevy_render (and we aren't rendering anything)

        // Add a SpritesheetAnimation component that references our animation
        SpritesheetAnimation::from_id(animation_id),
    ));
}

fn handle_animations_events(mut events: EventReader<AnimationEvent>) {
    for event in events.read() {
        dbg!(event);
    }
}
