// This example shows how to create a simple animated sprite.

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin to enable animations.
        // This makes the Assets<Animation> resource available to your systems.
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(
            Startup,
            (create_animation, spawn_sprite.after(create_animation)),
        )
        .run();
}

#[derive(Resource)]
struct MyAnimation {
    handle: Handle<Animation>,
}

fn create_animation(mut commands: Commands, mut animations: ResMut<Assets<Animation>>) {
    commands.spawn(Camera2d);

    // Create a clip that references some frames from a spritesheet

    let spritesheet = Spritesheet::new(8, 8);

    let clip = Clip::from_frames(spritesheet.row(3));

    // Create an animation that uses the clip
    //
    // This is a simple animation with a single clip but we can create more sophisticated
    // animations with multiple clips, each one having different parameters.
    //
    // See the `composition` example for more details.

    let animation = Animation::from_clip(clip);

    // Name the animation to retrieve it from other systems

    let animation_handle = animations.add(animation);

    commands.insert_resource(MyAnimation {
        handle: animation_handle,
    });
}

// We split the setup in two separate systems to show how to retrieve animations from their name

fn spawn_sprite(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    my_animation: Res<MyAnimation>,
) {
    // Retrieve our animation from the library

    // Create an image and a texture atlas like you would for any Bevy sprite
    //
    // However, here we use the Spritesheet helper to easily generate the atlas.
    // This is optional and you may prefer to build the atlas manually.

    let image = assets.load("character.png");

    let spritesheet = Spritesheet::new(8, 8);

    let atlas = TextureAtlas {
        layout: atlas_layouts.add(spritesheet.atlas_layout(96, 96)),
        ..default()
    };

    // Spawn a sprite with a SpritesheetAnimation component that references our animation

    commands.spawn((
        Sprite::from_atlas_image(image, atlas),
        SpritesheetAnimation::new(my_animation.handle.clone()),
    ));
}
