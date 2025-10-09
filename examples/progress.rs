// This example shows how to query and control the progress of an animation.

use bevy::prelude::*;
use bevy_spritesheet_animation::{animation::Animation, prelude::*};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin,
        ))
        .add_systems(Startup, spawn_character)
        .add_systems(Update, (control_animation, update_current_frame_text))
        .run();
}

#[derive(Component)]
struct CurrentFrameText;

fn spawn_character(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // Create an animation

    let image = assets.load("character.png");

    let spritesheet = Spritesheet::new(&image, 8, 8);

    let animation = spritesheet
        .create_animation()
        .add_row(3)
        .set_duration(AnimationDuration::PerFrame(1000))
        .build();

    let animation_handle = animations.add(animation);

    // Spawn an animated sprite

    let sprite = spritesheet
        .with_size_hint(768, 768)
        .sprite(&mut atlas_layouts);

    commands.spawn((sprite, SpritesheetAnimation::new(animation_handle)));

    // Text

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent                    .spawn(Text::new(
        "P: play/pause\nR: reset animation\n0/1/2/3/4/5: switch to frame x\nS: switch animation\n"));


            parent.spawn((CurrentFrameText,Text::new("")));
        });
}

fn control_animation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut sprite: Single<&mut SpritesheetAnimation>,
) {
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

fn update_current_frame_text(
    sprite: Single<&SpritesheetAnimation>,
    mut text: Single<&mut Text, With<CurrentFrameText>>,
) {
    text.0 = format!("Current frame: {}", sprite.progress.frame);
}
