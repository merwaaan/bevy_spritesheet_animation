// This example illustrates the effect of each clip/stage/animation parameter.

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
        .add_plugins(DefaultPlugins)
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<SpritesheetLibrary>,
    assets: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load assets for the sprite

    let texture = assets.load("ball.png");

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(100, 20),
        1,
        30,
        None,
        None,
    ));

    // Create an animation clip

    let clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(1, 30).column(0))
            .set_default_duration(AnimationDuration::PerCycle(1000));
    });

    // Create an animated sprite for each parameter set

    let mut parameters = vec![
        // Duration
        (Some(AnimationDuration::PerFrame(50)), None, None, None),
        (Some(AnimationDuration::PerFrame(500)), None, None, None),
        (Some(AnimationDuration::PerFrame(2000)), None, None, None),
        (Some(AnimationDuration::PerCycle(2000)), None, None, None),
        (Some(AnimationDuration::PerCycle(100)), None, None, None),
        // Repeat
        (None, Some(AnimationRepeat::Cycles(10)), None, None),
        (None, Some(AnimationRepeat::Cycles(100)), None, None),
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

    for (index, (duration, repeat, direction, easing)) in parameters.iter().enumerate() {
        let animation_id = library.new_animation(|animation| {
            animation.add_stage(clip_id.into());

            if let &Some(duration) = duration {
                animation.set_duration(duration);
            }

            if let &Some(repeat) = repeat {
                animation.set_repeat(repeat);
            }

            if let &Some(direction) = direction {
                animation.set_direction(direction);
            }

            if let &Some(easing) = easing {
                animation.set_easing(easing);
            }
        });

        commands
            .spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform::from_translation(grid_position(6, 6, index as u32)),
                    ..default()
                },
                TextureAtlas {
                    layout: layout.clone(),
                    ..default()
                },
                SpritesheetAnimation::from_id(animation_id),
            ))
            // Add a label describing the parameters
            .with_children(|builder| {
                let mut description = String::new();

                duration.inspect(|x| description.push_str(&format!("duration: {:?}\n", x)));
                repeat.inspect(|x| description.push_str(&format!("repeat: {:?}\n", x)));
                direction.inspect(|x| description.push_str(&format!("direction: {:?}\n", x)));
                easing.inspect(|x| description.push_str(&format!("easing: {:?}\n", x)));

                builder.spawn(Text2dBundle {
                    text: Text::from_section(
                        &description,
                        TextStyle {
                            font_size: 15.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0.0, -50.0, 0.0),
                    ..default()
                });
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
