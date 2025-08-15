// This example shows the effect of each animation parameter.

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::{
    easing::{Easing, EasingVariety},
    prelude::*,
};
use common::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin,
        ))
        .add_systems(Startup, spawn_animations)
        .run();
}

fn spawn_animations(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
    assets: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // Create a clip

    let spritesheet = Spritesheet::new(1, 30);

    let clip = Clip::from_frames(spritesheet.column(0));

    let clip_id = library.register_clip(clip);

    // Load assets for the sprites

    let image = assets.load("ball.png");

    let atlas = TextureAtlas {
        layout: atlas_layouts.add(spritesheet.atlas_layout(100, 20)),
        ..default()
    };

    // Create an animated sprite for each parameter set

    let mut parameters = vec![
        // Duration
        (Some(AnimationDuration::PerFrame(50)), None, None, None),
        (Some(AnimationDuration::PerFrame(500)), None, None, None),
        (Some(AnimationDuration::PerFrame(2000)), None, None, None),
        (
            Some(AnimationDuration::PerRepetition(2000)),
            None,
            None,
            None,
        ),
        (
            Some(AnimationDuration::PerRepetition(100)),
            None,
            None,
            None,
        ),
        // Repeat
        (None, Some(AnimationRepeat::Times(10)), None, None),
        (None, Some(AnimationRepeat::Times(100)), None, None),
        (None, Some(AnimationRepeat::Loop), None, None),
        // Direction
        (None, None, Some(AnimationDirection::Forwards), None),
        (None, None, Some(AnimationDirection::Backwards), None),
        (None, None, Some(AnimationDirection::PingPong), None),
    ];

    // Easing
    for variety in [
        EasingVariety::Quadratic,
        EasingVariety::Cubic,
        EasingVariety::Quartic,
        EasingVariety::Quintic,
        EasingVariety::Sin,
        EasingVariety::Exponential,
        EasingVariety::Circular,
    ] {
        parameters.push((None, None, None, Some(Easing::In(variety))));
        parameters.push((None, None, None, Some(Easing::Out(variety))));
        parameters.push((None, None, None, Some(Easing::InOut(variety))));
    }

    for (index, (duration, repetitions, direction, easing)) in parameters.iter().enumerate() {
        let mut animation = Animation::from_clip(clip_id);

        if let &Some(duration) = duration {
            animation.set_duration(duration);
        }

        if let &Some(repetitions) = repetitions {
            animation.set_repetitions(repetitions);
        }

        if let &Some(direction) = direction {
            animation.set_direction(direction);
        }

        if let &Some(easing) = easing {
            animation.set_easing(easing);
        }

        let animation_id = library.register_animation(animation);

        commands
            .spawn((
                Sprite::from_atlas_image(image.clone(), atlas.clone()),
                SpritesheetAnimation::from_id(animation_id),
                Transform::from_translation(grid_position(6, 6, index)),
            ))
            // Add a label describing the parameters
            .with_children(|builder| {
                let mut description = String::new();

                duration.inspect(|x| description.push_str(&format!("duration: {:?}\n", x)));
                repetitions.inspect(|x| description.push_str(&format!("repetitions: {:?}\n", x)));
                direction.inspect(|x| description.push_str(&format!("direction: {:?}\n", x)));
                easing.inspect(|x| description.push_str(&format!("easing: {:?}\n", x)));

                builder.spawn((
                    Text2d(description),
                    TextColor(Color::WHITE),
                    TextFont::from_font_size(15.0),
                    Transform::from_xyz(0.0, -50.0, 0.0),
                ));
            });
    }
}

// The spritesheet has been generated with this Javascript code:
//
// const frames = 30;
// const radius = 10;
// const distance = 100;
//
// const canvas = document.getElementById('canvas');
// canvas.width = distance + radius * 2;
// canvas.height = frames * radius * 2;
// canvas.style.width = canvas.width + "px";
// canvas.style.height = canvas.height + "px";
// canvas.style.imageRendering = "pixelated";
//
// const ctx = canvas.getContext('2d');
// ctx.fillStyle = 'red';
//
// for (var frame = 0; frame < frames; ++frame) {
//   var x = radius + (distance - 2 * radius) * frame / (frames - 1);
//   var y = radius + frame * 2 * radius;
//
//   ctx.beginPath();
//   ctx.arc(x, y, radius, 0, Math.PI * 2, true);
//   ctx.fill();
// }
