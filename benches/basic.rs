use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn basic(c: &mut Criterion) {
    c.bench_function("basic", |b| {
        b.iter(|| {
            let mut app = App::new();

            app.add_plugins(DefaultPlugins)
                .add_plugins(SpritesheetAnimationPlugin)
                .finish();

            let assets = app.world().get_resource::<AssetServer>().unwrap();

            let texture = assets.load("character.png");

            let mut atlas_layouts = app
                .world_mut()
                .get_resource_mut::<Assets<TextureAtlasLayout>>()
                .unwrap();

            let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(96, 96),
                8,
                8,
                None,
                None,
            ));

            let mut library = app
                .world_mut()
                .get_resource_mut::<SpritesheetLibrary>()
                .unwrap();

            let clip_id = library.new_clip(|clip| {
                clip.push_frame_indices([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
            });

            let animation_id = library.new_animation(|animation| {
                animation.add_stage(clip_id.into());
            });

            for _ in 0..1000 {
                app.world_mut().spawn((
                    SpriteBundle {
                        texture: texture.clone(),
                        ..default()
                    },
                    TextureAtlas {
                        layout: layout.clone(),
                        ..default()
                    },
                    SpritesheetAnimation::from_id(animation_id),
                ));
            }

            for _ in 0..1000 {
                app.update();
            }
        })
    });

    // TODO single sprite
    // TODO many sprites
    // TODO many animations
}

criterion_group!(benches, basic);
criterion_main!(benches);
