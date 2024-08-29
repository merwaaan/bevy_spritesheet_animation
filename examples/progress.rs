// This example shows how to control the progress of an animation.

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard)
        .run();
}

#[derive(Component)]
struct AllAnimations {
    animation1_id: AnimationId,
    animation2_id: AnimationId,
}

fn setup(
    mut commands: Commands,
    mut library: ResMut<AnimationLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    // Create two animations

    let spritesheet = Spritesheet::new(8, 8);

    let mut create_animation = |frames| {
        let clip = Clip::from_frames(frames).with_duration(AnimationDuration::PerFrame(2000));
        let clip_id = library.register_clip(clip);

        let animation = Animation::from_clip(clip_id);
        library.register_animation(animation)
    };

    let animation1_id = create_animation(spritesheet.row(3));
    let animation2_id = create_animation(spritesheet.horizontal_strip(0, 5, 5));

    // Spawn an animated sprite

    let texture = assets.load("character.png");

    let layout = atlas_layouts.add(Spritesheet::new(8, 8).atlas_layout(96, 96));

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        TextureAtlas {
            layout,
            ..default()
        },
        SpritesheetAnimation::from_id(animation1_id),
        // Store the two animation IDs in a component for convenience
        AllAnimations {
            animation1_id,
            animation2_id,
        },
    ));

    // Help text

    commands.spawn(TextBundle::from_section(
        "P: play/pause\nR: reset animation\n0/1/2/3/4/5: switch to frame x\nS: switch animation",
        TextStyle {
            font_size: 30.0,
            ..default()
        },
    ));
}

fn keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut sprites: Query<(&mut SpritesheetAnimation, &AllAnimations)>,
) {
    for (mut sprite, all_animations) in &mut sprites {
        // Pause the current animation

        if keyboard.just_pressed(KeyCode::KeyP) {
            sprite.playing = !sprite.playing;
        }

        // Reset the current animation

        if keyboard.just_pressed(KeyCode::KeyR) {
            sprite.reset();
        }

        // Go to a specific frame of the current animation

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

        // Switch to the other animation

        if keyboard.just_pressed(KeyCode::KeyS) {
            let next_animation = if sprite.animation_id == all_animations.animation1_id {
                all_animations.animation2_id
            } else {
                all_animations.animation1_id
            };

            sprite.switch(next_animation);
        }
    }
}
