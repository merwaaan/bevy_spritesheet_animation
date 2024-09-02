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
use rand::{seq::SliceRandom, Rng};

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
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            PerfUiPlugin,
        ))
        .insert_resource(cli)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut library: ResMut<AnimationLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    cli: Res<Cli>,
    assets: Res<AssetServer>,
) {
    // Spawn a camera

    match cli.mode {
        Mode::TwoD => commands.spawn(Camera2dBundle::default()),
        Mode::ThreeD => commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1000.0, 4000.0).looking_at(Vec3::ZERO, Dir3::Y),
            ..default()
        }),
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

    let mut rng = rand::thread_rng();

    let animation_directions = [
        AnimationDirection::Forwards,
        AnimationDirection::Backwards,
        AnimationDirection::PingPong,
    ];

    let animation_ids: Vec<AnimationId> = (0..100)
        .map(|_| {
            let clip_id = clip_ids.choose(&mut rng).unwrap().clone();

            let animation = Animation::from_clip(clip_id)
                .with_duration(AnimationDuration::PerFrame(rng.gen_range(100..1000)))
                .with_direction(animation_directions.choose(&mut rng).unwrap().clone());

            library.register_animation(animation)
        })
        .collect();

    // Spawn a lot of sprites, each with a random animation assigned

    let texture = assets.load("character.png");

    let atlas_layout = atlas_layouts.add(spritesheet.atlas_layout(96, 96));

    for _ in 0..cli.sprites {
        let animation = animation_ids.choose(&mut rng).unwrap();

        let transform = Transform::from_translation(random_position());

        match cli.mode {
            Mode::TwoD => commands.spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    transform,
                    ..default()
                },
                TextureAtlas {
                    layout: atlas_layout.clone(),
                    ..default()
                },
                SpritesheetAnimation::from_id(*animation),
            )),
            Mode::ThreeD => commands.spawn((
                Sprite3dBuilder::from_image(texture.clone())
                    .with_atlas(atlas_layout.clone())
                    .with_transform(transform)
                    .build(),
                SpritesheetAnimation::from_id(*animation),
            )),
        };
    }

    // UI

    commands.spawn((
        PerfUiRoot {
            // set a fixed width to make all the bars line up
            values_col_width: Some(160.0),
            ..Default::default()
        },
        PerfUiWidgetBar::new(PerfUiEntryFPS::default()),
    ));
}
