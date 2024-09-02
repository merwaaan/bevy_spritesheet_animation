use std::time::Duration;

use bevy::{
    ecs::system::SystemState,
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    winit::WinitPlugin,
};
use bevy_spritesheet_animation::{animator::Animator, prelude::*};
use divan::Bencher;
use rand::seq::IteratorRandom;

fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .build()
            // Headless mode
            .disable::<WinitPlugin>()
            .set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: None,
                    ..default()
                }
                .into(),
                ..default()
            }),
        SpritesheetAnimationPlugin,
    ))
    .finish();
    app
}

fn create_animation(app: &mut App) -> AnimationId {
    let mut library = app
        .world_mut()
        .get_resource_mut::<AnimationLibrary>()
        .unwrap();

    let clip = Clip::from_frames([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let clip_id = library.register_clip(clip);

    let animation = Animation::from_clip(clip_id);
    library.register_animation(animation)
}

fn create_sprite(app: &mut App, animation_id: AnimationId) {
    let mut atlas_layouts = app
        .world_mut()
        .get_resource_mut::<Assets<TextureAtlasLayout>>()
        .unwrap();

    let layout = atlas_layouts.add(Spritesheet::new(8, 8).atlas_layout(96, 96));

    app.world_mut().spawn((
        TextureAtlas {
            layout: layout.clone(),
            ..default()
        },
        SpritesheetAnimation::from_id(animation_id),
    ));
}

#[divan::bench(args=[(1, 1000), (1000, 1000)])]
fn playback(bencher: Bencher, (animation_count, sprite_count): (usize, usize)) {
    let mut app = create_app();

    let animation_ids: Vec<_> = (0..animation_count)
        .map(|_| create_animation(&mut app))
        .collect();

    let mut rng = rand::thread_rng();
    for _ in 0..sprite_count {
        create_sprite(
            &mut app,
            animation_ids.iter().choose(&mut rng).unwrap().clone(),
        );
    }

    let mut time = Time::new_with(());

    let mut system_state: SystemState<(
        Res<AnimationLibrary>,
        ResMut<Animator>,
        EventWriter<AnimationEvent>,
        Query<(Entity, &mut SpritesheetAnimation, &mut TextureAtlas)>,
    )> = SystemState::new(app.world_mut());

    bencher.bench_local(|| {
        for _ in 0..1000 {
            let (library, mut animator, mut event_writer, mut query) =
                system_state.get_mut(app.world_mut());

            animator.update(&time, &library, &mut event_writer, &mut query);

            time.advance_by(Duration::from_millis(100));
        }
    });
}

fn main() {
    divan::main();
}
