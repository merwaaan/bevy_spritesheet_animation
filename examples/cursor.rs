// This example shows how to animate a cursor.
//
// The 'custom_cursor' feature must be enabled for cursors to be animated (enabled by default).

#![cfg(feature = "custom_cursor")]

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, create_cursor)
        .add_systems(Update, trigger_clicks)
        .run();
}

fn create_cursor(
    window: Single<Entity, With<Window>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // Create an animation

    let image = assets.load("cursor.png");

    let spritesheet = Spritesheet::new(&image, 4, 1);

    let animation = spritesheet
        .create_animation()
        .add_row(0)
        // Repeat the animation once
        .set_repetitions(AnimationRepeat::Times(1))
        .build();

    let animation_handle = animations.add(animation);

    // Create a custom cursor using that animation

    let cursor_icon = spritesheet
        .with_size_hint(32 * 4, 32)
        .cursor_icon(&mut atlas_layouts);

    commands.entity(*window).insert((
        cursor_icon,
        SpritesheetAnimation::new(animation_handle)
            // Pause the animation on the last frame (idle frame)
            .with_playing(false)
            .with_progress(AnimationProgress::with_frame(3)),
    ));

    // Help text

    commands.spawn((Text::new("Click to animate the cursor"),));
}

fn trigger_clicks(
    buttons: Res<ButtonInput<MouseButton>>,
    mut sprite: Single<&mut SpritesheetAnimation>,
) {
    // Reset the click animation

    if buttons.just_pressed(MouseButton::Left) {
        sprite.reset();
        sprite.play();
    }
}
