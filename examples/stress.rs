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

#[path = "./common/mod.rs"]
pub mod common;

use bevy::{dev_tools::fps_overlay::FpsOverlayPlugin, prelude::*};
use bevy_spritesheet_animation::prelude::*;
use clap::{Parser, ValueEnum};
use common::random_position;
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
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
) {
    // Spawn a camera

    match cli.mode {
        Mode::TwoD => commands.spawn(Camera2d),
        Mode::ThreeD => commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 1000.0, 4000.0).looking_at(Vec3::ZERO, Dir3::Y),
        )),
    };

    // Create clips from a spritesheet

    let spritesheet = Spritesheet::new(8, 8);

    let clip_frames = [
        spritesheet.row_partial(0, ..5),
        spritesheet.row_partial(1, ..5),
        spritesheet.row(2),
        spritesheet.row(3),
        spritesheet.row_partial(4, ..5),
        spritesheet.row_partial(5, ..5),
        spritesheet.row(6),
        spritesheet.row(7),
    ];

    let clips = clip_frames.map(Clip::from_frames);

    // Create 100 animations from those clips, each with random parameters

    let mut rng = rand::rng();

    let animation_directions = [
        AnimationDirection::Forwards,
        AnimationDirection::Backwards,
        AnimationDirection::PingPong,
    ];

    let animation_handles: Vec<Handle<Animation>> = (0..100)
        .map(|_| {
            let clip = clips.choose(&mut rng).unwrap().clone();

            let animation = Animation::from_clip(clip)
                .with_duration(AnimationDuration::PerFrame(rng.random_range(100..1000)))
                .with_direction(*animation_directions.choose(&mut rng).unwrap());

            animations.add(animation)
        })
        .collect();

    // Spawn a lot of sprites, each with a random animation assigned

    let image = assets.load("character.png");

    let atlas = TextureAtlas {
        layout: atlas_layouts.add(spritesheet.atlas_layout(96, 96)),
        ..default()
    };

    for _ in 0..cli.sprites {
        let animation_handle = animation_handles.choose(&mut rng).unwrap();

        let transform = Transform::from_translation(random_position());

        match cli.mode {
            Mode::TwoD => commands.spawn((
                Sprite::from_atlas_image(image.clone(), atlas.clone()),
                SpritesheetAnimation::new(animation_handle.clone()),
                transform,
            )),
            Mode::ThreeD => commands.spawn((
                Sprite3d::from_atlas_image(image.clone(), atlas.clone()),
                SpritesheetAnimation::new(animation_handle.clone()),
                transform,
            )),
        };
    }
}
