use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    time::TimeUpdateStrategy,
    winit::WinitPlugin,
};
use bevy_spritesheet_animation::prelude::*;

pub struct Context {
    pub app: App,
    pub sprite_entity: Entity,
}

impl Context {
    pub fn new() -> Self {
        // Initialize the app

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
        // Insert a manual update strategy to control time
        .insert_resource(TimeUpdateStrategy::ManualInstant(Instant::now()));

        // Increase the max delta for each frame

        app.world_mut()
            .get_resource_mut::<Time<Virtual>>()
            .unwrap()
            .set_max_delta(Duration::from_millis(10000));

        // Update the app once so that Time's delta is not zero in the tests

        app.update();

        // Add a sprite

        let assets = app.world().get_resource::<AssetServer>().unwrap();

        let image = assets.load("character.png");

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

        let atlas = TextureAtlas {
            layout,
            ..default()
        };

        let sprite = app
            .world_mut()
            .spawn(Sprite::from_atlas_image(image, atlas))
            .id();

        Self {
            app,
            sprite_entity: sprite,
        }
    }

    pub fn library(&mut self) -> Mut<'_, AnimationLibrary> {
        self.app
            .world_mut()
            .get_resource_mut::<AnimationLibrary>()
            .unwrap()
    }

    pub fn add_animation_to_sprite(&mut self, animation_id: AnimationId) {
        self.app
            .world_mut()
            .entity_mut(self.sprite_entity)
            .insert(SpritesheetAnimation::from_id(animation_id));
    }

    pub fn run(&mut self, ms: u32) {
        // Clear the events from the previous frame

        let mut events_resources = self
            .app
            .world_mut()
            .get_resource_mut::<Events<AnimationEvent>>()
            .unwrap();

        events_resources.clear();

        // Move time forwards

        let mut time_strategy = self
            .app
            .world_mut()
            .get_resource_mut::<TimeUpdateStrategy>();

        if let Some(TimeUpdateStrategy::ManualInstant(ref mut last_instant)) =
            time_strategy.as_deref_mut()
        {
            *last_instant += Duration::from_millis(ms as u64);
        }

        self.app.update();
    }

    pub fn check(
        &mut self,
        expected_atlas_index: usize,
        expected_events: impl IntoIterator<Item = AnimationEvent>,
    ) {
        // Check the current atlas index of the sprite

        let entity_ref = self.app.world().entity(self.sprite_entity);

        let atlas = entity_ref
            .get::<Sprite>()
            .and_then(|sprite| sprite.texture_atlas.as_ref())
            .or(entity_ref
                .get::<Sprite3d>()
                .and_then(|sprite| sprite.texture_atlas.as_ref()))
            .unwrap();

        assert_eq!(atlas.index, expected_atlas_index);

        // Check the emitted events

        let events_resources = self
            .app
            .world_mut()
            .get_resource_mut::<Events<AnimationEvent>>()
            .unwrap();

        let mut events: HashSet<AnimationEvent> = HashSet::new();

        for event in events_resources.get_cursor().read(&events_resources) {
            events.insert(*event);
        }

        assert_eq!(events, HashSet::from_iter(expected_events));
    }

    pub fn get_sprite<F: FnMut(&mut SpritesheetAnimation) -> ()>(&mut self, mut f: F) {
        let mut sprite_animation = self
            .app
            .world_mut()
            .get_mut::<SpritesheetAnimation>(self.sprite_entity)
            .unwrap();

        f(&mut sprite_animation);
    }

    pub fn update_sprite_animation<F: FnMut(&mut SpritesheetAnimation) -> ()>(
        &mut self,
        mut builder: F,
    ) {
        let mut sprite_animation = self
            .app
            .world_mut()
            .get_mut::<SpritesheetAnimation>(self.sprite_entity)
            .unwrap();

        builder(&mut sprite_animation);
    }

    pub fn marker_hit(
        &self,
        marker_id: AnimationMarkerId,
        animation_id: AnimationId,
        animation_repetition: usize,
        clip_id: ClipId,
        clip_repetition: usize,
    ) -> AnimationEvent {
        AnimationEvent::MarkerHit {
            entity: self.sprite_entity,
            marker_id,
            animation_id,
            animation_repetition,
            clip_id,
            clip_repetition,
        }
    }

    pub fn clip_rep_end(
        &self,
        animation_id: AnimationId,
        clip_id: ClipId,
        clip_repetition: usize,
    ) -> AnimationEvent {
        AnimationEvent::ClipRepetitionEnd {
            entity: self.sprite_entity,
            animation_id,
            clip_id,
            clip_repetition,
        }
    }

    pub fn clip_end(&self, animation_id: AnimationId, clip_id: ClipId) -> AnimationEvent {
        AnimationEvent::ClipEnd {
            entity: self.sprite_entity,
            animation_id,
            clip_id,
        }
    }

    pub fn anim_rep_end(
        &self,
        animation_id: AnimationId,
        animation_repetition: usize,
    ) -> AnimationEvent {
        AnimationEvent::AnimationRepetitionEnd {
            entity: self.sprite_entity,
            animation_id,
            animation_repetition,
        }
    }

    pub fn anim_end(&self, animation_id: AnimationId) -> AnimationEvent {
        AnimationEvent::AnimationEnd {
            entity: self.sprite_entity,
            animation_id,
        }
    }
}
