// This example showcases different animation parameters.

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_spritesheet_animation::prelude::*;

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
    windows: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let window = windows.single().expect("no primary window");

    commands.spawn(Camera2d);

    // Create an animated sprite for each parameter set

    let image = assets.load("ball.png");

    let spritesheet = Spritesheet::new(&image, 1, 30);

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
        let mut animation = spritesheet.create_animation().add_column(0);

        if let &Some(duration) = duration {
            animation = animation.set_duration(duration);
        }

        if let &Some(repetitions) = repetitions {
            animation = animation.set_repetitions(repetitions);
        }

        if let &Some(direction) = direction {
            animation = animation.set_direction(direction);
        }

        if let &Some(easing) = easing {
            animation = animation.set_easing(easing);
        }

        let animation_handle = animations.add(animation.build());

        let sprite = spritesheet
            .with_size_hint(100, 600)
            .sprite(&mut atlas_layouts);

        commands
            .spawn((
                sprite,
                SpritesheetAnimation::new(animation_handle),
                Transform::from_translation(grid_position(window, 6, 6, index)),
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
                    Transform::from_xyz(0.0, -50.0, 0.0).with_scale(Vec3::splat(0.7)),
                ));
            });
    }
}

/// Returns the screen-space position of the nth item in a grid
fn grid_position(window: &Window, columns: u32, rows: u32, n: usize) -> Vec3 {
    const MARGIN: f32 = 100.0;

    let width = window.width() - MARGIN * 2.0;
    let height = window.height() - MARGIN * 2.0;

    let xgap = width / columns.saturating_sub(1) as f32;
    let ygap = height / rows.saturating_sub(1) as f32;

    let x = (n as u32 % columns) as f32;
    let y = (n as u32 / columns) as f32;

    Vec3::new(
        x * xgap - width / 2.0,
        -y * ygap + height / 2.0, // flip Y
        0.0,
    )
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
