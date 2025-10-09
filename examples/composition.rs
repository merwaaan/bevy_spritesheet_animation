// This example shows how to create composite animations made of multiple clips.

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin,
        ))
        .add_systems(Startup, create_animated_sprite)
        .run();
}

fn create_animated_sprite(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // Compose an animation with multiple clips
    //
    // - idle 3 times
    // - run 5 times
    // - shoot once
    //
    // The whole animation will repeat 2 times

    let image = assets.load("character.png");

    let spritesheet = Spritesheet::new(&image, 8, 8);

    let animation = spritesheet
        .create_animation()
        .set_repetitions(AnimationRepeat::Times(2))
        // Clip 1
        .add_horizontal_strip(0, 0, 5)
        .set_clip_duration(AnimationDuration::PerRepetition(700))
        .set_clip_repetitions(3)
        // Clip 2
        .start_clip()
        .add_row(3)
        .set_clip_duration(AnimationDuration::PerRepetition(600))
        .set_clip_repetitions(5)
        // Clip 3
        .start_clip()
        .add_horizontal_strip(0, 5, 5)
        .set_clip_duration(AnimationDuration::PerRepetition(600))
        .set_clip_repetitions(1)
        .build();

    let animation_handle = animations.add(animation);

    // Spawn a sprite that uses the animation

    let sprite = spritesheet
        .with_size_hint(768, 768)
        .sprite(&mut atlas_layouts);

    commands.spawn((sprite, SpritesheetAnimation::new(animation_handle)));
}
