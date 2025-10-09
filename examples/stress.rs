// A stress test with an unreasonable amount of animated 3D sprites
//
// CLI options:
//
// Pass "2d" for 2D sprites (default)
// Pass "3d" for 3D sprites
//
// Pass --sprites X to render X sprites (default is 100 000)
//
// Best executed in --release mode!

use bevy::{dev_tools::fps_overlay::FpsOverlayPlugin, prelude::*, window::PrimaryWindow};
use bevy_spritesheet_animation::prelude::*;
use clap::{Parser, ValueEnum};
use rand::{Rng, seq::IndexedRandom as _};

#[derive(ValueEnum, Clone)]
enum Mode {
    #[clap(name = "2d")]
    TwoD,
    #[clap(name = "3d")]
    ThreeD,
}

#[derive(Parser, Resource)]
struct Cli {
    #[arg(value_enum, default_value_t=Mode::TwoD)]
    mode: Mode,

    #[arg(long, default_value_t = 100_000)]
    sprites: usize,
}

fn main() {
    let cli = Cli::parse();

    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin,
            FpsOverlayPlugin::default(),
        ))
        .insert_resource(cli)
        .add_systems(Startup, spawn_sprites)
        .run();
}

fn spawn_sprites(
    cli: Res<Cli>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn a camera

    match cli.mode {
        Mode::TwoD => commands.spawn(Camera2d),
        Mode::ThreeD => commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 1000.0, 4000.0).looking_at(Vec3::ZERO, Dir3::Y),
        )),
    };

    // Create base animations from the rows of a spritesheet

    let image = assets.load("character.png");

    let spritesheet = Spritesheet::new(&image, 8, 8);

    let base_animations = [
        spritesheet.create_animation().add_partial_row(0, ..5),
        spritesheet.create_animation().add_partial_row(1, ..5),
        spritesheet.create_animation().add_row(2),
        spritesheet.create_animation().add_row(3),
        spritesheet.create_animation().add_partial_row(4, ..5),
        spritesheet.create_animation().add_partial_row(5, ..5),
        spritesheet.create_animation().add_row(6),
        spritesheet.create_animation().add_row(7),
    ];

    // Create 100 derived animations, each with random parameters

    let mut rng = rand::rng();

    let animation_directions = [
        AnimationDirection::Forwards,
        AnimationDirection::Backwards,
        AnimationDirection::PingPong,
    ];

    let animation_handles: Vec<Handle<Animation>> = (0..100)
        .map(|_| {
            let base_animation = base_animations.choose(&mut rng).unwrap();

            let animation = base_animation
                .clone()
                .set_duration(AnimationDuration::PerFrame(rng.random_range(100..1000)))
                .set_direction(*animation_directions.choose(&mut rng).unwrap())
                .build();

            animations.add(animation)
        })
        .collect();

    // Spawn A LOT of sprites, each with a random animation assigned

    let component_generator = spritesheet.with_size_hint(768, 768);

    for _ in 0..cli.sprites {
        let animation_handle = animation_handles.choose(&mut rng).unwrap();

        let transform = Transform::from_translation(random_position(&window));

        match cli.mode {
            Mode::TwoD => commands.spawn((
                component_generator.sprite(&mut atlas_layouts),
                SpritesheetAnimation::new(animation_handle.clone()),
                transform,
            )),
            Mode::ThreeD => commands.spawn((
                component_generator.sprite3d(&mut atlas_layouts),
                SpritesheetAnimation::new(animation_handle.clone()),
                transform,
            )),
        };
    }
}

pub fn random_position(window: &Window) -> Vec3 {
    let mut rng = rand::rng();

    Vec3::new(
        rng.random_range(-window.width() / 2.0..window.width() / 2.0),
        rng.random_range(-window.height() / 2.0..window.height() / 2.0),
        0.0,
    )
}
