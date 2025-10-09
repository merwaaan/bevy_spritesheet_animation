// This example shows how to query and control the progress of an animation.

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::{animation::Animation, prelude::*};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin,
        ))
        .add_systems(Startup, spawn_character)
        .add_systems(Update, control_animation)
        .run();
}

fn spawn_character(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
) {
    commands.spawn(Camera2d);

    // Create an animation

    let spritesheet = Spritesheet::new(8, 8);

    let clip =
        Clip::from_frames(spritesheet.row(3)).with_duration(AnimationDuration::PerFrame(2000));

    let animation = Animation::from_clip(clip);

    let animation_handle = animations.add(animation);

    // Spawn an animated sprite

    let image = assets.load("character.png");

    let atlas = TextureAtlas {
        layout: atlas_layouts.add(spritesheet.atlas_layout(96, 96)),
        ..default()
    };

    commands.spawn((
        Sprite::from_atlas_image(image, atlas),
        SpritesheetAnimation::new(animation_handle),
    ));

    // Help text

    commands.spawn((Text(
        "P: play/pause\nR: reset animation\n0/1/2/3/4/5: switch to frame x\nS: switch animation".to_owned()),
        TextFont::from_font_size(30.0)
    ));
}

fn control_animation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut sprites: Query<&mut SpritesheetAnimation>,
) {
    for mut sprite in &mut sprites {
        // Play/Pause the animation

        if keyboard.just_pressed(KeyCode::KeyP) {
            sprite.playing = !sprite.playing;
        }

        // Reset the animation

        if keyboard.just_pressed(KeyCode::KeyR) {
            sprite.reset();
        }

        // Go to a specific frame of the animation

        let keys = [
            KeyCode::Numpad0,
            KeyCode::Numpad1,
            KeyCode::Numpad2,
            KeyCode::Numpad3,
            KeyCode::Numpad4,
            KeyCode::Numpad5,
        ];

        for (frame, key) in keys.iter().enumerate() {
            if keyboard.just_pressed(*key) {
                sprite.progress.frame = frame;
            }
        }
    }
}

// TODO print frame index
