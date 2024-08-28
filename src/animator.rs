mod cache;
mod iterator;

use bevy::{
    ecs::{
        entity::Entity,
        event::EventWriter,
        system::{Query, Resource},
    },
    sprite::TextureAtlas,
};
use std::{collections::HashMap, sync::Arc};

use crate::{
    animation::AnimationId, components::spritesheet_animation::SpritesheetAnimation,
    events::AnimationEvent, library::AnimationLibrary,
    systems::spritesheet_animation::ActualTime,
};

use self::{
    cache::{AnimationCache, AnimationFrameEvent},
    iterator::AnimationIterator,
};

/// An instance of an animation that is currently being played
struct AnimationInstance {
    animation_id: AnimationId,
    iterator: AnimationIterator,
    current_frame_duration: u32,
    current_clip_index: usize,
    accumulated_time: u32,

    /// Marks when the animation has ended to emit end events only once
    ended: bool,
}

/// The animator is responsible for playing animations as time advances.
#[derive(Resource)]
pub(crate) struct SpritesheetAnimator {
    /// Animation caches, one for each animation.
    /// They contain all the data required to play an animation.
    animation_caches: HashMap<AnimationId, Arc<AnimationCache>>,

    /// Instances of animations currently being played.
    /// Each animation instance is associated to an entity with a [SpritesheetAnimation] component.
    animation_instances: HashMap<Entity, AnimationInstance>,
}

impl SpritesheetAnimator {
    pub fn new() -> Self {
        Self {
            animation_caches: HashMap::new(),
            animation_instances: HashMap::new(),
        }
    }

    /// Run the animations
    pub fn update(
        &mut self,
        time: &ActualTime,
        library: &AnimationLibrary,
        event_writer: &mut EventWriter<AnimationEvent>,
        query: &mut Query<(Entity, &mut SpritesheetAnimation, &mut TextureAtlas)>,
    ) {
        // Clear outdated animation instances associated to entities that do not have the component anymore

        self.animation_instances
            .retain(|entity, _state| query.contains(*entity));

        // Run animations for all the entities

        for (entity, mut entity_animation, mut entity_atlas) in query.iter_mut() {
            // Create a new animation instance if:
            // - the entity is new OR
            // - it switched animation OR
            // - a reset has been requested

            let needs_new_animation_instance = match self.animation_instances.get(&entity) {
                // The entity has an animation instance already but it switched animation
                Some(instance) => {
                    instance.animation_id != entity_animation.animation_id
                        || entity_animation.reset_requested
                }
                // The entity has no animation instance yet
                None => true,
            };

            if needs_new_animation_instance {
                // Clear any reset request

                entity_animation.reset_requested = false;

                // Retrieve the cached animation data (create it if needed)

                let cache = self
                    .animation_caches
                    .entry(entity_animation.animation_id)
                    .or_insert_with(|| {
                        Arc::new(AnimationCache::new(entity_animation.animation_id, library))
                    });

                // Create a new iterator for this animation

                let mut iterator =
                    AnimationIterator::new(entity_animation.animation_id, cache.clone());

                // Immediatly assign the first frame to kicktart the animation

                let maybe_first_frame = iterator.next();

                if let Some(first_frame) = &maybe_first_frame {
                    entity_atlas.index = first_frame.atlas_index;

                    // Emit events for the first frame

                    let events = SpritesheetAnimator::promote_events(&first_frame.events, &entity);

                    for event in events {
                        event_writer.send(event);
                    }
                }

                let (first_frame_duration, first_clip_index) = maybe_first_frame
                    .map(|frame| (frame.duration, frame.clip_index))
                    .unwrap_or((u32::MAX, usize::MAX));

                self.animation_instances.insert(
                    entity,
                    AnimationInstance {
                        animation_id: entity_animation.animation_id,
                        iterator,
                        current_frame_duration: first_frame_duration,
                        current_clip_index: first_clip_index,
                        accumulated_time: 0,
                        ended: false,
                    },
                );
            }

            let animation_instance = self.animation_instances.get_mut(&entity).unwrap();

            // Skip the update if the animation is paused
            //
            // (skipped AFTER the setup above so that the first frame is assigned, even if paused)

            if !entity_animation.playing {
                continue;
            }

            // Update the animation

            animation_instance.accumulated_time +=
                (time.delta_seconds() * entity_animation.speed_factor * 1000.0) as u32;

            while animation_instance.accumulated_time > animation_instance.current_frame_duration {
                // Consume the elapsed time

                animation_instance.accumulated_time -= animation_instance.current_frame_duration;

                // Fetch the next frame

                if let Some(next_frame) = animation_instance.iterator.next() {
                    // Update the entity's texture atlas

                    entity_atlas.index = next_frame.atlas_index;

                    // Store this frame's data

                    animation_instance.current_frame_duration = next_frame.duration;
                    animation_instance.current_clip_index = next_frame.clip_index;

                    // Emit the events for this frame

                    let events = SpritesheetAnimator::promote_events(&next_frame.events, &entity);

                    for frame_event in events {
                        event_writer.send(frame_event);
                    }
                }
                // Otherwise, the animation is over
                else {
                    // Emit all the end events if the animation just ended

                    if !animation_instance.ended {
                        event_writer.send(AnimationEvent::ClipRepetitionEnd {
                            entity,
                            clip_index: animation_instance.current_clip_index,
                            animation_id: animation_instance.animation_id,
                        });

                        event_writer.send(AnimationEvent::ClipEnd {
                            entity,
                            clip_index: animation_instance.current_clip_index,
                            animation_id: animation_instance.animation_id,
                        });

                        event_writer.send(AnimationEvent::AnimationRepetitionEnd {
                            entity,
                            animation_id: animation_instance.animation_id,
                        });

                        event_writer.send(AnimationEvent::AnimationEnd {
                            entity,
                            animation_id: animation_instance.animation_id,
                        });

                        //

                        animation_instance.current_frame_duration = u32::MAX;
                    }

                    animation_instance.ended = true;

                    // Stop

                    break;
                }
            }
        }
    }

    /// Promotes AnimationFrameEvents to regular AnimationEvents
    fn promote_events(
        animation_events: &[AnimationFrameEvent],
        entity: &Entity,
    ) -> Vec<AnimationEvent> {
        animation_events
            .iter()
            .map(|event| match event {
                AnimationFrameEvent::MarkerHit {
                    marker_id,
                    clip_index,
                    animation_id,
                } => AnimationEvent::MarkerHit {
                    entity: *entity,
                    marker_id: *marker_id,
                    clip_index: *clip_index,
                    animation_id: *animation_id,
                },
                AnimationFrameEvent::ClipRepetitionEnd {
                    clip_index,
                    animation_id,
                } => AnimationEvent::ClipRepetitionEnd {
                    entity: *entity,
                    clip_index: *clip_index,
                    animation_id: *animation_id,
                },
                AnimationFrameEvent::ClipEnd {
                    clip_index,
                    animation_id,
                } => AnimationEvent::ClipEnd {
                    entity: *entity,
                    clip_index: *clip_index,
                    animation_id: *animation_id,
                },
                AnimationFrameEvent::AnimationRepetitionEnd { animation_id } => {
                    AnimationEvent::AnimationRepetitionEnd {
                        entity: *entity,
                        animation_id: *animation_id,
                    }
                }
            })
            .collect()
    }
}
