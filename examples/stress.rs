// A stress test with thousands of animated 3D sprites
//
// CLI:
//
// Pass "2d" for 2D sprites (default)
// Pass "3d" for 3D sprites
//
// Pass --sprites X to render X sprites (default is 100 000)
//
// Best executed in --release mode!

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use clap::{Parser, ValueEnum};
use common::random_position;
use iyes_perf_ui::prelude::*;
use rand::{seq::IndexedRandom as _, Rng};

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
            bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
            PerfUiPlugin,
        ))
        .insert_resource(cli)
        .add_systems(Startup, spawn_sprites)
        .run();
}

fn spawn_sprites(
    mut commands: Commands,
    mut library: ResMut<AnimationLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    cli: Res<Cli>,
    assets: Res<AssetServer>,
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

    let clip_ids = clip_frames.map(|frames| {
        let clip = Clip::from_frames(frames);
        library.register_clip(clip)
    });

    // Create 100 animations from those clips, each with random parameters

    let mut rng = rand::rng();

    let animation_directions = [
        AnimationDirection::Forwards,
        AnimationDirection::Backwards,
        AnimationDirection::PingPong,
    ];

    let animation_ids: Vec<AnimationId> = (0..100)
        .map(|_| {
            let clip_id = *clip_ids.choose(&mut rng).unwrap();

            let animation = Animation::from_clip(clip_id)
                .with_duration(AnimationDuration::PerFrame(rng.random_range(100..1000)))
                .with_direction(*animation_directions.choose(&mut rng).unwrap());

            library.register_animation(animation)
        })
        .collect();

    // Spawn a lot of sprites, each with a random animation assigned

    let image = assets.load("character.png");

    let atlas = TextureAtlas {
        layout: atlas_layouts.add(Spritesheet::new(8, 8).atlas_layout(96, 96)),
        ..default()
    };

    for _ in 0..cli.sprites {
        let animation = animation_ids.choose(&mut rng).unwrap();

        let transform = Transform::from_translation(random_position());

        match cli.mode {
            Mode::TwoD => commands.spawn((
                Sprite::from_atlas_image(image.clone(), atlas.clone()),
                SpritesheetAnimation::from_id(*animation),
                transform,
            )),
            Mode::ThreeD => commands.spawn((
                Sprite3d::from_atlas_image(image.clone(), atlas.clone()),
                SpritesheetAnimation::from_id(*animation),
                transform,
            )),
        };
    }

    // UI

    commands.spawn((
        PerfUiRoot {
            // Set a fixed width to make all the bars line up
            values_col_width: 160.0,
            ..Default::default()
        },
        PerfUiWidgetBar::new(PerfUiEntryFPS::default()),
    ));
}
