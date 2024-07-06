// This example illustrates how to create a simple animated sprite.

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin to enable animations.
        // This makes the SpritesheetLibrary resource available to your systems.
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, (setup, spawn_sprite))
        .run();
}

fn setup(mut commands: Commands, mut library: ResMut<SpritesheetLibrary>) {
    commands.spawn(Camera2dBundle::default());

    // Create a simple animation
    //
    // - create a new clip that references some frames from a spritesheet
    // - create a new animation that uses that clip

    let clip_id = library.new_clip(|clip| {
        // You can configure this clip here (duration, number of repetitions, etc...)

        // This clip will use all the frames in row 3 of the spritesheet
        clip.push_frame_indices(Spritesheet::new(8, 8).row(3));
    });

    let animation_id = library.new_animation(|animation| {
        // You can configure this animation here (duration, number of repetitions, etc...)

        animation.add_stage(clip_id.into());

        // This is a simple animation with a single clip but we can create more sophisticated
        // animations with multiple clips, each one having different parameters.
        //
        // See the `composition` example for more details.
    });

    // Attach a name to the animation in order to be able to retrieve it from the library in other systems
    //
    // If you prefer a less error-prone approach, you may keep the animation ID around instead
    // (for instance in a Bevy Resource that contains the IDs of all your animations)

    library.name_animation(animation_id, "walk").unwrap();
}

fn spawn_sprite(
    mut commands: Commands,
    library: Res<SpritesheetLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
) {
    // Create an image and an atlas layout like you would for any Bevy sprite

    let texture = assets.load("character.png");

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(96, 96),
        8,
        8,
        None,
        None,
    ));

    // Retrieve our animation from the library

    let animation_id = library.animation_with_name("walk");

    if let Some(id) = animation_id {
        // Spawn a sprite with Bevy's built-in SpriteSheetBundle

        commands.spawn((
            SpriteBundle {
                texture,
                ..default()
            },
            TextureAtlas {
                layout,
                ..default()
            },
            //  Add a SpritesheetAnimation component that references our animation
            SpritesheetAnimation::from_id(id),
        ));
    }
}
