// This example shows how to create a animated cursor.
//
// The 'custom_cursor' feature must be enabled for custom cursors to be animated (enabled by default).

#[path = "./common/mod.rs"]
pub mod common;

use bevy::{
    prelude::*,
    window::{CursorIcon, CustomCursor, CustomCursorImage},
};
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, create_cursor)
        .run();
}

fn create_cursor(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    mut library: ResMut<AnimationLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // Create an animation

    let spritesheet = Spritesheet::new(8, 8);

    let clip = Clip::from_frames(spritesheet.row(3));
    let clip_id = library.register_clip(clip);

    let animation = Animation::from_clip(clip_id);
    let animation_id = library.register_animation(animation);

    // Create a custom cursor using that animation

    let image = assets.load("character.png");

    let atlas = TextureAtlas {
        layout: atlas_layouts.add(spritesheet.atlas_layout(96, 96)),
        ..default()
    };

    commands.entity(*window).insert((
        CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            handle: image,
            texture_atlas: Some(atlas),
            ..default()
        })),
        SpritesheetAnimation::from_id(animation_id),
    ));
}
