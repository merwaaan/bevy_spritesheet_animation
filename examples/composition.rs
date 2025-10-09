// This example shows how to create composite animations made of multiple clips.

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
        .run();
}

fn spawn_character(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
) {
    commands.spawn(Camera2d);

    // Compose an animation from multiple clips
    //
    // - idle 3 times
    // - run 5 times
    // - shoot once
    //
    // The whole animation will repeat 2 times

    let spritesheet = Spritesheet::new(8, 8);

    let idle_clip = Clip::from_frames(spritesheet.horizontal_strip(0, 0, 5))
        .with_duration(AnimationDuration::PerRepetition(700))
        .with_repetitions(3);

    let run_clip = Clip::from_frames(spritesheet.row(3))
        .with_duration(AnimationDuration::PerRepetition(600))
        .with_repetitions(5);

    let shoot_clip = Clip::from_frames(spritesheet.horizontal_strip(0, 5, 5))
        .with_duration(AnimationDuration::PerRepetition(600))
        .with_repetitions(1);

    let animation = Animation::from_clips([idle_clip, run_clip, shoot_clip])
        .with_repetitions(AnimationRepeat::Times(2));

    let animation_handle = animations.add(animation);

    // Spawn a sprite that uses the animation

    let image = assets.load("character.png");

    let atlas = TextureAtlas {
        layout: atlas_layouts.add(spritesheet.atlas_layout(96, 96)),
        ..default()
    };

    commands.spawn((
        Sprite::from_atlas_image(image, atlas),
        SpritesheetAnimation::new(animation_handle),
    ));
}
