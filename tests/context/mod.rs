use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    time::TimeUpdateStrategy,
    winit::WinitPlugin,
};
use bevy_spritesheet_animation::prelude::*;
use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

pub struct Context {
    pub app: App,
    pub sprite: Entity,
}

impl Context {
    pub fn new() -> Self {
        // Initialize the app

        let mut app = App::new();

        // #[cfg(not(windows))]
        // app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>());

        // #[cfg(windows)]
        // app.add_plugins(
        //     DefaultPlugins
        //         .build()
        //         // Headless mode
        //         .disable::<WinitPlugin>()
        //         .disable::<LogPlugin>()
        //         .set(RenderPlugin {
        //             render_creation: WgpuSettings {
        //                 backends: None,
        //                 ..default()
        //             }
        //             .into(),
        //             ..default()
        //         }),
        // );

        //app.add_plugins(MinimalPlugins);

        app.add_plugins(
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
        );

        // Add our plugin

        app.add_plugins(SpritesheetAnimationPlugin);

        // Insert a manual update strategy to control time
        app.insert_resource(TimeUpdateStrategy::ManualInstant(Instant::now()));

        // Update the app once so that Time's delta is not zero in the tests

        app.update();

        // Add a sprite

        let assets = app.world.get_resource::<AssetServer>().unwrap();

        let texture = assets.load("character.png");

        let mut atlas_layouts = app
            .world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();

        let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
            Vec2::new(96.0, 96.0),
            8,
            8,
            None,
            None,
        ));

        let sprite = app
            .world
            .spawn(SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout,
                    ..default()
                },
                ..default()
            })
            .id();

        Self { app, sprite }
    }

    pub fn library(&mut self) -> Mut<'_, SpritesheetLibrary> {
        self.app
            .world
            .get_resource_mut::<SpritesheetLibrary>()
            .unwrap()
    }

    pub fn add_animation_to_sprite(&mut self, animation_id: AnimationId) {
        self.app
            .world
            .entity_mut(self.sprite)
            .insert(SpritesheetAnimation::from_id(animation_id));
    }

    pub fn run(&mut self, ms: u32) {
        // Clear the events from the previous frame

        let mut events_resources = self
            .app
            .world
            .get_resource_mut::<Events<AnimationEvent>>()
            .unwrap();

        events_resources.clear();

        // Move time forwards

        let mut time_strategy = self.app.world.get_resource_mut::<TimeUpdateStrategy>();

        if let Some(TimeUpdateStrategy::ManualInstant(ref mut last_instant)) =
            time_strategy.as_deref_mut()
        {
            *last_instant += Duration::from_millis(ms as u64);
        }

        self.app.update();
    }

    pub fn check(&mut self, expected_atlas_index: usize, expected_events: &[AnimationEvent]) {
        println!("check");
        // Check the current atlas index of the sprite

        let atlas = self.get_sprite_atlas(self.sprite);

        assert_eq!(atlas.index, expected_atlas_index);

        // Check the emitted events

        let events_resources = self
            .app
            .world
            .get_resource_mut::<Events<AnimationEvent>>()
            .unwrap();

        let mut events: HashSet<AnimationEvent> = HashSet::new();

        for event in events_resources.get_reader().read(&events_resources) {
            events.insert(*event);
        }

        assert_eq!(events, HashSet::from_iter(expected_events.iter().cloned()));
    }

    fn get_sprite_atlas(&self, entity: Entity) -> TextureAtlas {
        self.app
            .world
            .entity(entity)
            .get::<TextureAtlas>()
            .unwrap()
            .clone()
    }

    pub fn update_sprite_animation<F: Fn(&mut SpritesheetAnimation) -> ()>(&mut self, builder: F) {
        let mut sprite_animation = self
            .app
            .world
            .get_mut::<SpritesheetAnimation>(self.sprite)
            .unwrap();

        builder(&mut sprite_animation);
    }

    pub fn stage_cycle_end(&self, stage_index: usize, animation_id: AnimationId) -> AnimationEvent {
        AnimationEvent::ClipCycleEnd {
            entity: self.sprite,
            stage_index,
            animation_id,
        }
    }

    pub fn stage_end(&self, stage_index: usize, animation_id: AnimationId) -> AnimationEvent {
        AnimationEvent::ClipEnd {
            entity: self.sprite,
            stage_index,
            animation_id,
        }
    }

    pub fn anim_cycle_end(&self, animation_id: AnimationId) -> AnimationEvent {
        AnimationEvent::AnimationCycleEnd {
            entity: self.sprite,
            animation_id,
        }
    }

    pub fn anim_end(&self, animation_id: AnimationId) -> AnimationEvent {
        AnimationEvent::AnimationEnd {
            entity: self.sprite,
            animation_id,
        }
    }
}
