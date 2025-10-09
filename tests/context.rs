use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use bevy::{
    log::LogPlugin,
    prelude::*,
    render::{RenderPlugin, settings::WgpuSettings},
    time::TimeUpdateStrategy,
    winit::WinitPlugin,
};
use bevy_spritesheet_animation::prelude::*;

pub struct Context {
    pub app: App,
    pub sprite_entity: Entity,
}

impl Context {
    // Creates the test context
    pub fn new() -> Self {
        // Create the app

        let mut app = App::new();

        app.add_plugins((
            DefaultPlugins
                .build()
                // Headless mode
                .disable::<WinitPlugin>()
                .disable::<LogPlugin>()
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

        // Increase the max delta for each frame as we'll increment time manually by various amounts

        app.world_mut()
            .get_resource_mut::<Time<Virtual>>()
            .unwrap()
            .set_max_delta(Duration::from_millis(10000));

        // Update the app once so that Time's delta is not zero in the tests

        app.update();

        // Add a sprite

        let mut atlas_layouts = app
            .world_mut()
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();

        let sprite = Spritesheet::new(&Handle::default(), 8, 8)
            .with_size_hint(768, 768)
            .sprite(&mut atlas_layouts);

        let entity = app.world_mut().spawn(sprite).id();

        Self {
            app,
            sprite_entity: entity,
        }
    }

    // Registers an animation and returns a handle
    pub fn create_animation<F>(&mut self, mut f: F) -> Handle<Animation>
    where
        F: FnMut(AnimationBuilder) -> AnimationBuilder,
    {
        let builder = AnimationBuilder::new(Spritesheet::new(&Handle::default(), 8, 8));

        let animation = f(builder).build();

        let mut animations = self
            .app
            .world_mut()
            .get_resource_mut::<Assets<Animation>>()
            .unwrap();

        animations.add(animation.clone())
    }

    // Adds an animation to the test sprite
    pub fn attach_animation<F>(&mut self, f: F) -> Handle<Animation>
    where
        F: FnMut(AnimationBuilder) -> AnimationBuilder,
    {
        let animation_handle = self.create_animation(f);

        self.app
            .world_mut()
            .entity_mut(self.sprite_entity)
            .insert(SpritesheetAnimation::new(animation_handle.clone()));

        animation_handle
    }

    // Runs the app for some time
    pub fn run(&mut self, ms: u32) {
        // Clear the events from the previous frame

        let mut events_resources = self
            .app
            .world_mut()
            .get_resource_mut::<Messages<AnimationEvent>>()
            .unwrap();

        events_resources.clear();

        // Move time forwards

        let mut time_strategy = self
            .app
            .world_mut()
            .get_resource_mut::<TimeUpdateStrategy>();

        if let Some(TimeUpdateStrategy::ManualInstant(last_instant)) = time_strategy.as_deref_mut()
        {
            *last_instant += Duration::from_millis(ms as u64);
        }

        self.app.update();
    }

    // Tests the current state of the sprite
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
            .get_resource_mut::<Messages<AnimationEvent>>()
            .unwrap();

        let mut events: HashSet<AnimationEvent> = HashSet::new();

        for event in events_resources.get_cursor().read(&events_resources) {
            events.insert(event.clone());
        }

        assert_eq!(events, HashSet::from_iter(expected_events));
    }

    // Gets the sprites to inspect or update it
    pub fn get_sprite<F: FnMut(&mut SpritesheetAnimation)>(&mut self, mut f: F) {
        let mut sprite = self
            .app
            .world_mut()
            .get_mut::<SpritesheetAnimation>(self.sprite_entity)
            .unwrap();

        f(&mut sprite);
    }

    // Helpers that create animation events

    pub fn marker_hit(
        &self,
        marker: Marker,
        animation: &Handle<Animation>,
        animation_repetition: usize,
        clip_id: ClipId,
        clip_repetition: usize,
    ) -> AnimationEvent {
        AnimationEvent::MarkerHit {
            entity: self.sprite_entity,
            marker,
            clip_id,
            clip_repetition,
            animation: animation.clone(),
            animation_repetition,
        }
    }

    pub fn clip_rep_end(
        &self,
        animation: &Handle<Animation>,
        clip_id: ClipId,
        clip_repetition: usize,
    ) -> AnimationEvent {
        AnimationEvent::ClipRepetitionEnd {
            entity: self.sprite_entity,
            clip_id,
            clip_repetition,
            animation: animation.clone(),
        }
    }

    pub fn clip_end(&self, animation: &Handle<Animation>, clip_id: ClipId) -> AnimationEvent {
        AnimationEvent::ClipEnd {
            entity: self.sprite_entity,
            clip_id,
            animation: animation.clone(),
        }
    }

    pub fn anim_rep_end(
        &self,
        animation: &Handle<Animation>,
        animation_repetition: usize,
    ) -> AnimationEvent {
        AnimationEvent::AnimationRepetitionEnd {
            entity: self.sprite_entity,
            animation: animation.clone(),
            animation_repetition,
        }
    }

    pub fn anim_end(&self, animation: &Handle<Animation>) -> AnimationEvent {
        AnimationEvent::AnimationEnd {
            entity: self.sprite_entity,
            animation: animation.clone(),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
