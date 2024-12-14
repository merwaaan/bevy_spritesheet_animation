// This example shows how to create a simple animated sprite.

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin to enable animations.
        // This makes the AnimationLibrary resource available to your systems.
        .add_plugins(SpritesheetAnimationPlugin::default())
        .add_systems(
            Startup,
            (create_animation, spawn_sprite.after(create_animation)),
        )
        .run();
}

fn create_animation(mut commands: Commands, mut library: ResMut<AnimationLibrary>) {
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

    // This is a simple animation with a single clip but we can create more sophisticated
    // animations with multiple clips, each one having different parameters.
    //
    // See the `composition` example for more details.
}

// We split the setup in two separate systems to show how to retrieve animations from their name

fn spawn_sprite(
    mut commands: Commands,
    library: Res<AnimationLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
) {
    // Retrieve our animation from the library

    if let Some(animation_id) = library.animation_with_name("walk") {
        // Create an image and a texture atlas like you would for any Bevy sprite

        let image = assets.load("character.png");

        let atlas = TextureAtlas {
            layout: atlas_layouts.add(Spritesheet::new(8, 8).atlas_layout(96, 96)),
            ..default()
        };

        // Spawn a sprite with a SpritesheetAnimation component that references our animation

        commands.spawn((
            Sprite::from_atlas_image(image, atlas),
            SpritesheetAnimation::from_id(animation_id),
        ));
    }
}
