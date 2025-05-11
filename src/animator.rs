pub mod cache;
mod iterator;

use crate::{
    animation::AnimationId,
    animator::iterator::{AnimationIterator, IteratorFrame},
    components::{
        sprite3d::Sprite3d,
        spritesheet_animation::{AnimationProgress, SpritesheetAnimation},
    },
    events::AnimationEvent,
    library::AnimationLibrary,
};
#[cfg(feature = "custom_cursor")]
use bevy::winit::cursor::{CursorIcon, CustomCursor};
use bevy::{
    ecs::{
        entity::Entity, event::EventWriter, query::QueryData, reflect::*, resource::Resource,
        system::Query,
    },
    reflect::prelude::*,
    sprite::Sprite,
    time::Time,
    ui::widget::ImageNode,
};
use iterator::AnimationIteratorEvent;
use std::{collections::HashMap, time::Duration};

#[derive(Debug, Reflect)]
#[reflect(Debug)]
/// An instance of an animation that is currently being played
struct AnimationInstance {
    animation_id: AnimationId,
    iterator: AnimationIterator,

    /// Current frame
    current_frame: Option<(IteratorFrame, AnimationProgress)>,

    /// Time accumulated since the last frame
    accumulated_time: Duration,
}

/// The animator is responsible for playing animations as time advances.
#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource, Debug, Default)]
pub struct Animator {
    /// Instances of animations currently being played.
    /// Each animation instance is associated to an entity with a [SpritesheetAnimation] component.
    animation_instances: HashMap<Entity, AnimationInstance>,
}

/// A query data type for the [`Animator::update`] system.
#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct SpritesheetAnimationQuery {
    entity: Entity,
    spritesheet_animation: &'static mut SpritesheetAnimation,
    sprite: Option<&'static mut Sprite>,
    sprite3d: Option<&'static mut Sprite3d>,
    image_node: Option<&'static mut ImageNode>,
    #[cfg(feature = "custom_cursor")]
    cursor_icon: Option<&'static mut CursorIcon>,
}

impl Animator {
    /// Plays the animations
    pub fn update(
        &mut self,
        time: &Time,
        library: &AnimationLibrary,
        event_writer: &mut EventWriter<AnimationEvent>,
        query: &mut Query<SpritesheetAnimationQuery>,
    ) {
        // Clear outdated animation instances associated to entities that do not have the component anymore

        self.animation_instances
            .retain(|entity, _state| query.contains(*entity));

        // Run animations for all the entities

        for mut item in query.iter_mut() {
            let current_component_animation_id = item.spritesheet_animation.animation_id;
            let mut previous_accumulated_time = Duration::ZERO;
            let mut is_directional_change_of_same_type = false;

            // Check if this is a change of AnimationId for an existing instance
            if let Some(existing_instance) = self.animation_instances.get(&item.entity) {
                if existing_instance.animation_id != current_component_animation_id {
                    // AnimationId has changed. Let's see if it's a grouped change.
                    let old_animation_data = library.get_animation(existing_instance.animation_id);
                    let new_animation_data = library.get_animation(current_component_animation_id);

                    if old_animation_data.animation_group_key.is_some() && // Both must have a key
                       old_animation_data.animation_group_key == new_animation_data.animation_group_key
                    {
                        // It's a change within the same animation group (e.g., different direction)
                        is_directional_change_of_same_type = true;
                        previous_accumulated_time = existing_instance.accumulated_time;
                    }
                }
            }

            // Create a new animation instance if:
            let needs_new_animation_instance = match self.animation_instances.get(&item.entity) {
                // The entity has an animation instance already but it switched animation
                Some(instance) => instance.animation_id != current_component_animation_id,
                // The entity has no animation instance yet
                None => true,
            };

            if needs_new_animation_instance {
                // Create a new iterator for this animation

                let cache = library.get_animation_cache(current_component_animation_id);

                let mut iterator = AnimationIterator::new(cache.clone());

                // Move to the starting progress if specified

                if item.spritesheet_animation.progress != AnimationProgress::default() {
                    // Start from the beginning if the progress is invalid
                    if !iterator.to(item.spritesheet_animation.progress) {
                        item.spritesheet_animation.progress = AnimationProgress::default();
                    }
                }

                // Create the instance and immediately play the first frame

                let first_frame_data = Self::play_frame(&mut iterator, &mut item, event_writer);

                let accumulated_time_for_new_instance = if is_directional_change_of_same_type {
                    previous_accumulated_time
                } else {
                    Duration::ZERO
                };

                self.animation_instances.insert(
                    item.entity,
                    AnimationInstance {
                        animation_id: current_component_animation_id,
                        iterator,
                        current_frame: first_frame_data,
                        accumulated_time: accumulated_time_for_new_instance,
                    },
                );
            }

            let animation_instance = self.animation_instances.get_mut(&item.entity).unwrap();

            // Apply manual progress updates

            if let Some((ref _current_iterator_frame, current_iterator_actual_progress)) =
                animation_instance.current_frame
            {
                if item.spritesheet_animation.progress != current_iterator_actual_progress {
                    // User manually changed item.spritesheet_animation.progress
                    if animation_instance
                        .iterator
                        .to(item.spritesheet_animation.progress)
                    {
                        Self::play_frame(&mut animation_instance.iterator, &mut item, event_writer)
                            .inspect(|new_frame_data| {
                                animation_instance.current_frame = Some(new_frame_data.clone());
                                animation_instance.accumulated_time = Duration::ZERO;
                            });
                    } else {
                        // Restore to the last valid progress if user set an invalid one
                        item.spritesheet_animation.progress = current_iterator_actual_progress;
                    }
                }
            }

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

            while let Some(cf_data_tuple) = animation_instance.current_frame.as_ref() {
                let frame_duration = cf_data_tuple.0.duration;

                if animation_instance.accumulated_time >= frame_duration {
                    animation_instance.accumulated_time -= frame_duration;

                    // Store current frame info before advancing, for end events
                    let last_played_frame_data = cf_data_tuple.0.clone();

                    // Fetch the next frame

                    animation_instance.current_frame =
                        Self::play_frame(&mut animation_instance.iterator, &mut item, event_writer)
                            .or_else(|| {
                                event_writer.write(AnimationEvent::ClipRepetitionEnd {
                                    entity: item.entity,
                                    animation_id: animation_instance.animation_id,
                                    clip_id: last_played_frame_data.clip_id,
                                    clip_repetition: last_played_frame_data.clip_repetition,
                                });

                                event_writer.write(AnimationEvent::ClipEnd {
                                    entity: item.entity,
                                    animation_id: animation_instance.animation_id,
                                    clip_id: last_played_frame_data.clip_id,
                                });

                                event_writer.write(AnimationEvent::AnimationRepetitionEnd {
                                    entity: item.entity,
                                    animation_id: animation_instance.animation_id,
                                    animation_repetition: last_played_frame_data
                                        .animation_repetition,
                                });

                                event_writer.write(AnimationEvent::AnimationEnd {
                                    entity: item.entity,
                                    animation_id: animation_instance.animation_id,
                                });

                                None
                            });

                    if animation_instance.current_frame.is_none() {
                        break; // Animation finished, exit while loop
                    }
                } else {
                    break; // Not enough accumulated_time for the current frame
                }
            }
        }
    }

    fn play_frame(
        iterator: &mut AnimationIterator,
        item: &mut SpritesheetAnimationQueryItem<'_>,
        event_writer: &mut EventWriter<AnimationEvent>,
    ) -> Option<(IteratorFrame, AnimationProgress)> {
        let maybe_frame = iterator.next();

        if let Some((frame, progress)) = &maybe_frame {
            // Update the sprite
            // (we compare the indices to prevent needless "Changed" events)

            if let Some(atlas) = item
                .sprite
                .as_deref_mut()
                .and_then(|sprite| sprite.texture_atlas.as_mut())
            {
                if atlas.index != frame.atlas_index {
                    atlas.index = frame.atlas_index;
                }
            }

            if let Some(atlas) = item
                .sprite3d
                .as_deref_mut()
                .and_then(|sprite| sprite.texture_atlas.as_mut())
            {
                if atlas.index != frame.atlas_index {
                    atlas.index = frame.atlas_index;
                }
            }

            if let Some(atlas) = item
                .image_node
                .as_deref_mut()
                .and_then(|image| image.texture_atlas.as_mut())
            {
                if atlas.index != frame.atlas_index {
                    atlas.index = frame.atlas_index;
                }
            }

            #[cfg(feature = "custom_cursor")]
            if let Some(atlas) = item
                .cursor_icon
                .as_deref_mut()
                .and_then(|cursor_icon| {
                    if let CursorIcon::Custom(CustomCursor::Image {
                        ref mut texture_atlas,
                        ..
                    }) = *cursor_icon
                    {
                        Some(texture_atlas)
                    } else {
                        None
                    }
                })
                .and_then(|atlas| atlas.as_mut())
            {
                if atlas.index != frame.atlas_index {
                    atlas.index = frame.atlas_index;
                }
            }

            item.spritesheet_animation.progress = *progress;

            // Emit events

            Animator::emit_events(
                &frame.events,
                item.spritesheet_animation.animation_id,
                &item.entity,
                event_writer,
            );
        }

        maybe_frame
    }

    fn emit_events(
        animation_events: &[AnimationIteratorEvent],
        animation_id: AnimationId,
        entity: &Entity,
        event_writer: &mut EventWriter<AnimationEvent>,
    ) {
        animation_events.iter().for_each(|event| {
            event_writer.write(
                // Promote AnimationIteratorEvents to regular AnimationEvents
                match event {
                    AnimationIteratorEvent::MarkerHit {
                        marker_id,
                        animation_repetition,
                        clip_id,
                        clip_repetition,
                    } => AnimationEvent::MarkerHit {
                        entity: *entity,
                        marker_id: *marker_id,
                        animation_id,
                        animation_repetition: *animation_repetition,
                        clip_id: *clip_id,
                        clip_repetition: *clip_repetition,
                    },
                    AnimationIteratorEvent::ClipRepetitionEnd {
                        clip_id,
                        clip_repetition,
                    } => AnimationEvent::ClipRepetitionEnd {
                        entity: *entity,
                        animation_id,
                        clip_id: *clip_id,
                        clip_repetition: *clip_repetition,
                    },
                    AnimationIteratorEvent::ClipEnd { clip_id } => AnimationEvent::ClipEnd {
                        entity: *entity,
                        animation_id,
                        clip_id: *clip_id,
                    },
                    AnimationIteratorEvent::AnimationRepetitionEnd {
                        animation_repetition,
                    } => AnimationEvent::AnimationRepetitionEnd {
                        entity: *entity,
                        animation_id,
                        animation_repetition: *animation_repetition,
                    },
                },
            );
        });
    }
}
