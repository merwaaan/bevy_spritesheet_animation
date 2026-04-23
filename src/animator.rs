pub(crate) mod cache;
mod iterator;

use std::{sync::Arc, time::Duration};

use bevy::prelude::*;

#[cfg(feature = "custom_cursor")]
use bevy::window::{CursorIcon, CustomCursor, CustomCursorImage};
use bevy::{
    asset::{AssetId, Assets, Handle},
    ecs::{
        entity::Entity,
        message::MessageWriter,
        query::QueryData,
        resource::Resource,
        system::{Query, ResMut},
    },
    image::TextureAtlas,
    platform::collections::HashMap,
    reflect::Reflect,
    sprite::Sprite,
    time::Time,
    ui::widget::ImageNode,
};

#[cfg(feature = "3d")]
use crate::components::sprite3d::Sprite3d;
use crate::{
    CRATE_NAME,
    animation::Animation,
    animator::{
        cache::AnimationCache,
        iterator::{AnimationIterator, IteratorFrame},
    },
    components::spritesheet_animation::{AnimationProgress, SpritesheetAnimation},
    events::AnimationEvent,
};
use iterator::AnimationIteratorEvent;

#[derive(Debug, Reflect)]
#[reflect(Debug)]
/// An instance of an animation that is currently being played
struct AnimationInstance {
    animation: Handle<Animation>,
    iterator: AnimationIterator,

    /// Current frame
    current_frame: Option<(IteratorFrame, AnimationProgress)>,

    /// Time accumulated since the last frame
    accumulated_time: Duration,
}

/// The animator is responsible for playing animations as time advances.
#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource, Debug, Default)]
pub(crate) struct Animator {
    /// Animation caches, one for each animation
    ///
    /// They contain all the data required to play an animation.
    animation_caches: HashMap<AssetId<Animation>, Arc<AnimationCache>>,

    /// Instances of animations currently being played
    ///
    /// Each animation instance is associated to an entity with a [SpritesheetAnimation] component.
    animation_instances: HashMap<Entity, AnimationInstance>,
}

/// A query data type for the [`Animator::animate`] system.
#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub(crate) struct SpritesheetAnimationQuery {
    entity: Entity,
    spritesheet_animation: &'static mut SpritesheetAnimation,
    sprite: Option<&'static mut Sprite>,
    #[cfg(feature = "3d")]
    sprite3d: Option<&'static mut Sprite3d>,
    image_node: Option<&'static mut ImageNode>,
    #[cfg(feature = "custom_cursor")]
    cursor_icon: Option<&'static mut CursorIcon>,
}

impl Animator {
    /// Advances animations and updates their current frame.
    pub fn animate(
        &mut self,
        time: &Time,
        message_writer: &mut MessageWriter<AnimationEvent>,
        query: &mut Query<SpritesheetAnimationQuery>,
        animations: &mut ResMut<Assets<Animation>>,
    ) {
        // Clear outdated animation instances associated to entities that do not have the component anymore

        self.animation_instances
            .retain(|entity, _state| query.contains(*entity));

        // Run animations for all the entities

        for mut item in query.iter_mut() {
            if !self.sync_instance(&mut item, animations, message_writer) {
                continue;
            }

            let animation_instance = self.animation_instances.get_mut(&item.entity).unwrap();

            // Skip the update if the animation is paused
            //
            // (skipped AFTER the setup above so that the first frame is assigned, even if paused)

            if !item.spritesheet_animation.playing {
                continue;
            }

            // Update the animation

            animation_instance.accumulated_time += Duration::from_secs_f32(
                time.delta_secs() * item.spritesheet_animation.speed_factor,
            );

            while let Some(current_frame) = animation_instance
                .current_frame
                .as_ref()
                .filter(|frame| animation_instance.accumulated_time > frame.0.duration)
            {
                // Consume the elapsed time

                animation_instance.accumulated_time -= current_frame.0.duration;

                // Fetch the next frame

                animation_instance.current_frame =
                    Self::play_frame(&mut animation_instance.iterator, &mut item, message_writer)
                        .or_else(|| {
                            // The animation is over

                            // Emit the end events if the animation just ended

                            message_writer.write(AnimationEvent::ClipRepetitionEnd {
                                entity: item.entity,
                                clip_id: current_frame.0.clip_id,
                                clip_repetition: current_frame.0.clip_repetition,
                                animation: animation_instance.animation.clone(),
                            });

                            message_writer.write(AnimationEvent::ClipEnd {
                                entity: item.entity,
                                clip_id: current_frame.0.clip_id,
                                animation: animation_instance.animation.clone(),
                            });

                            message_writer.write(AnimationEvent::AnimationRepetitionEnd {
                                entity: item.entity,
                                animation: animation_instance.animation.clone(),
                                animation_repetition: current_frame.0.animation_repetition,
                            });

                            message_writer.write(AnimationEvent::AnimationEnd {
                                entity: item.entity,
                                animation: animation_instance.animation.clone(),
                            });

                            None
                        });
            }
        }
    }

    /// Reconciles animation component changes applied later in the frame.
    pub fn sync(
        &mut self,
        message_writer: &mut MessageWriter<AnimationEvent>,
        query: &mut Query<SpritesheetAnimationQuery>,
        animations: &mut ResMut<Assets<Animation>>,
    ) {
        for mut item in query.iter_mut() {
            self.sync_instance(&mut item, animations, message_writer);
        }
    }

    fn sync_instance(
        &mut self,
        item: &mut SpritesheetAnimationQueryItem<'_, '_>,
        animations: &mut ResMut<Assets<Animation>>,
        message_writer: &mut MessageWriter<AnimationEvent>,
    ) -> bool {
        let animation = item.spritesheet_animation.animation.clone();
        let animation_id = animation.id();
        let cache = if let Some(cache) = self.animation_caches.get(&animation_id).cloned() {
            cache
        } else if let Some(animation_asset) = animations.get(animation_id) {
            let cache = Arc::new(AnimationCache::from_animation(animation_asset));
            self.animation_caches.insert(animation_id, cache.clone());
            cache
        } else {
            error!(
                "{CRATE_NAME}: missing animation asset for entity {entity:?}, skipping update",
                entity = item.entity,
            );
            self.animation_instances.remove(&item.entity);
            return false;
        };

        let needs_new_animation_instance = match self.animation_instances.get(&item.entity) {
            None => true,
            Some(instance) => {
                instance.animation != animation
                    || instance.current_frame.is_none()
                        && item.spritesheet_animation.progress.frame == 0
            }
        };

        if needs_new_animation_instance {
            let mut iterator = AnimationIterator::new(cache.clone());

            if item.spritesheet_animation.progress != AnimationProgress::default() {
                if !iterator.to(item.spritesheet_animation.progress) {
                    item.spritesheet_animation.progress = AnimationProgress::default();
                }
            }

            let first_frame = Self::play_frame(&mut iterator, item, message_writer);

            self.animation_instances.insert(
                item.entity,
                AnimationInstance {
                    animation,
                    iterator,
                    current_frame: first_frame,
                    accumulated_time: Duration::ZERO,
                },
            );
        }

        let Some(animation_instance) = self.animation_instances.get_mut(&item.entity) else {
            error!(
                "{CRATE_NAME}: missing animation instance for entity {entity:?}, skipping update",
                entity = item.entity,
            );
            return false;
        };

        if animation_instance
            .current_frame
            .as_ref()
            .is_some_and(|frame| item.spritesheet_animation.progress != frame.1)
        {
            if animation_instance
                .iterator
                .to(item.spritesheet_animation.progress)
            {
                Self::play_frame(&mut animation_instance.iterator, item, message_writer).inspect(
                    |new_frame| {
                        animation_instance.current_frame = Some(new_frame.clone());
                        animation_instance.accumulated_time = Duration::ZERO;
                    },
                );
            } else {
                item.spritesheet_animation.progress = animation_instance
                    .current_frame
                    .as_ref()
                    .map(|(_, progress)| *progress)
                    .unwrap_or_default()
            }
        }

        true
    }

    fn play_frame(
        iterator: &mut AnimationIterator,
        item: &mut SpritesheetAnimationQueryItem<'_, '_>,
        message_writer: &mut MessageWriter<AnimationEvent>,
    ) -> Option<(IteratorFrame, AnimationProgress)> {
        let maybe_frame = iterator.next();

        if let Some((frame, progress)) = &maybe_frame {
            Self::sync_atlas(
                item.sprite
                    .as_deref_mut()
                    .and_then(|sprite| sprite.texture_atlas.as_mut()),
                frame.atlas_index,
            );

            #[cfg(feature = "3d")]
            Self::sync_atlas(
                item.sprite3d
                    .as_deref_mut()
                    .and_then(|sprite| sprite.texture_atlas.as_mut()),
                frame.atlas_index,
            );

            Self::sync_atlas(
                item.image_node
                    .as_deref_mut()
                    .and_then(|image| image.texture_atlas.as_mut()),
                frame.atlas_index,
            );

            #[cfg(feature = "custom_cursor")]
            Self::sync_atlas(
                item.cursor_icon
                    .as_deref_mut()
                    .and_then(|cursor_icon| match cursor_icon {
                        CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
                            texture_atlas,
                            ..
                        })) => texture_atlas.as_mut(),
                        _ => None,
                    }),
                frame.atlas_index,
            );

            item.spritesheet_animation.progress = *progress;

            Self::emit_events(
                &frame.events,
                &item.spritesheet_animation.animation,
                item.entity,
                message_writer,
            );
        }

        maybe_frame
    }

    fn sync_atlas(atlas: Option<&mut TextureAtlas>, atlas_index: usize) {
        if let Some(atlas) = atlas
            && atlas.index != atlas_index
        {
            atlas.index = atlas_index;
        }
    }

    fn emit_events(
        animation_events: &[AnimationIteratorEvent],
        animation: &Handle<Animation>,
        entity: Entity,
        message_writer: &mut MessageWriter<AnimationEvent>,
    ) {
        for event in animation_events {
            message_writer.write(match event {
                AnimationIteratorEvent::MarkerHit {
                    marker,
                    clip_id,
                    clip_repetition,
                    animation_repetition,
                } => AnimationEvent::MarkerHit {
                    entity,
                    marker: *marker,
                    clip_id: *clip_id,
                    clip_repetition: *clip_repetition,
                    animation: animation.clone(),
                    animation_repetition: *animation_repetition,
                },
                AnimationIteratorEvent::ClipRepetitionEnd {
                    clip_id,
                    clip_repetition,
                } => AnimationEvent::ClipRepetitionEnd {
                    entity,
                    clip_id: *clip_id,
                    clip_repetition: *clip_repetition,
                    animation: animation.clone(),
                },
                AnimationIteratorEvent::ClipEnd { clip_id } => AnimationEvent::ClipEnd {
                    entity,
                    clip_id: *clip_id,
                    animation: animation.clone(),
                },
                AnimationIteratorEvent::AnimationRepetitionEnd {
                    animation_repetition,
                } => AnimationEvent::AnimationRepetitionEnd {
                    entity,
                    animation: animation.clone(),
                    animation_repetition: *animation_repetition,
                },
            });
        }
    }
}
