// A stress test with thousands of animated sprites
// (best executed in --release mode!)

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use common::*;
use rand::{seq::SliceRandom, Rng};

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
    mut library: ResMut<AnimationLibrary>,
    assets: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

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

            let duration = AnimationDuration::PerRepetition(rng.gen_range(100..1000));

            let direction = animation_directions.choose(&mut rng).unwrap().clone();

            let animation = Animation::from_clip(clip_id)
                .with_duration(duration)
                .with_direction(direction);

            library.register_animation(animation)
        })
        .collect();

    // Spawn a lot of sprites, each with a random animation assigned

    let texture = assets.load("character.png");

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(96, 96),
        8,
        8,
        None,
        None,
    ));

    for _ in 0..100_000 {
        let animation = animation_ids.choose(&mut rng).unwrap();

        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_translation(random_position()),
                ..default()
            },
            TextureAtlas {
                layout: layout.clone(),
                ..default()
            },
            SpritesheetAnimation::from_id(*animation),
        ));
    }
}
