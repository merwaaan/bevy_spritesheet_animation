use std::{sync::Arc, time::Duration};

use bevy::{log::warn, reflect::prelude::*};

use crate::{
    CRATE_NAME, animation::AnimationDirection, clip::ClipId,
    components::spritesheet_animation::AnimationProgress, events::AnimationMarkerId,
};

use super::cache::{AnimationCache, AnimationCacheEvent, CacheFrame};

/// Same as [CacheFrame] but with `animation_repetition`
#[derive(Debug, Clone, Reflect)]
#[reflect(Debug)]
pub struct IteratorFrame {
    pub atlas_index: usize,
    pub duration: Duration,
    pub clip_id: ClipId,
    pub clip_repetition: usize,
    pub animation_repetition: usize,
    pub events: Vec<AnimationIteratorEvent>,
}

/// A partial version of AnimationEvent.
///
/// The animation will promote them to regular AnimationEvents and add the information available at its level.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub enum AnimationIteratorEvent {
    MarkerHit {
        marker_id: AnimationMarkerId,
        animation_repetition: usize,
        clip_id: ClipId,
        clip_repetition: usize,
    },
    ClipRepetitionEnd {
        clip_id: ClipId,
        clip_repetition: usize,
    },
    ClipEnd {
        clip_id: ClipId,
    },
    AnimationRepetitionEnd {
        animation_repetition: usize,
    },
}

#[derive(Debug, Reflect)]
#[reflect(Debug)]
/// An iterator that advances an animation frame by frame.
///
/// `next()` will produce frames until the end of the animation.
pub struct AnimationIterator {
    /// Reference to the animation cache that contains all the frames for one repetition of the animation
    cache: Arc<AnimationCache>,

    /// Current iteration progress
    next_frame_progress: AnimationProgress,

    /// Marks when a repetition just completed so that end events can be emitted on the next iteration
    /// (the value is the last frame)
    repetition_just_ended: Option<CacheFrame>,
}

impl AnimationIterator {
    pub fn new(cache: Arc<AnimationCache>) -> Self {
        Self {
            cache,
            next_frame_progress: AnimationProgress::default(),
            repetition_just_ended: None,
        }
    }

    /// Sets the current animation progress.
    ///
    /// Returns false if the indices are invalid.
    pub fn to(&mut self, progress: AnimationProgress) -> bool {
        // Validate the target progress

        if progress.frame >= self.cache.frames.len() {
            warn!(
                "{CRATE_NAME}: invalid frame {} in {}-frame animation, cannot update progress",
                progress.frame,
                self.cache.frames.len().saturating_sub(1)
            );

            false
        } else if let Some(repetitions) = self
            .cache
            .repetitions
            .filter(|repetitions| progress.repetition >= *repetitions)
        {
            warn!(
                "{CRATE_NAME}: invalid repetition {} in {}-repetition animation, cannot update progress",
                progress.frame,
                repetitions.saturating_sub(1)
            );

            false
        } else {
            // Update the iterator

            self.next_frame_progress = progress;
            self.repetition_just_ended = None;

            true
        }
    }

    /// Promotes AnimationCacheEvents to AnimationIteratorEvents
    fn promote_events(
        animation_events: &[AnimationCacheEvent],
        animation_repetition: usize,
    ) -> Vec<AnimationIteratorEvent> {
        animation_events
            .iter()
            .map(|event| match event {
                AnimationCacheEvent::MarkerHit {
                    marker_id,
                    clip_id,
                    clip_repetition,
                } => AnimationIteratorEvent::MarkerHit {
                    marker_id: *marker_id,
                    animation_repetition,
                    clip_id: *clip_id,
                    clip_repetition: *clip_repetition,
                },
                AnimationCacheEvent::ClipRepetitionEnd {
                    clip_id,
                    clip_repetition,
                } => AnimationIteratorEvent::ClipRepetitionEnd {
                    clip_id: *clip_id,
                    clip_repetition: *clip_repetition,
                },
                AnimationCacheEvent::ClipEnd { clip_id } => {
                    AnimationIteratorEvent::ClipEnd { clip_id: *clip_id }
                }
            })
            .collect()
    }
}

impl Iterator for AnimationIterator {
    type Item = (IteratorFrame, AnimationProgress);

    fn next(&mut self) -> Option<Self::Item> {
        // Retrieve the appropriate frame set from the cache

        let cached_frames = if let Some(frames_pong) = &self.cache.frames_pong {
            if self.next_frame_progress.repetition.is_multiple_of(2) {
                // Regular frames for even PingPong repetitions
                &self.cache.frames
            } else {
                // Frames for odd PingPong repetitions
                frames_pong
            }
        } else {
            // Regular frames
            &self.cache.frames
        };

        // Fetch the current frame

        cached_frames
            .get(self.next_frame_progress.frame)
            .map(|cached_frame| {
                let current_frame_progress = self.next_frame_progress;

                // Promote the frame with the current animation repetition

                let mut frame = IteratorFrame {
                    atlas_index: cached_frame.atlas_index,
                    duration: cached_frame.duration,
                    clip_id: cached_frame.clip_id,
                    clip_repetition: cached_frame.clip_repetition,
                    animation_repetition: current_frame_progress.repetition,
                    events: Self::promote_events(
                        &cached_frame.events,
                        current_frame_progress.repetition,
                    ),
                };

                // Inject the missing end events in the returned frame

                if let Some(previous_frame) = &self.repetition_just_ended {
                    frame
                        .events
                        .push(AnimationIteratorEvent::ClipRepetitionEnd {
                            clip_id: previous_frame.clip_id,
                            clip_repetition: previous_frame.clip_repetition,
                        });

                    frame.events.push(AnimationIteratorEvent::ClipEnd {
                        clip_id: previous_frame.clip_id,
                    });

                    frame
                        .events
                        .push(AnimationIteratorEvent::AnimationRepetitionEnd {
                            animation_repetition: current_frame_progress
                                .repetition
                                .saturating_sub(1),
                        });

                    self.repetition_just_ended = None;
                }

                // Increment the indices for the next iteration

                self.next_frame_progress.frame += 1;

                // Go back to the start if we reached the end

                if self.next_frame_progress.frame >= self.cache.frames.len() {
                    self.next_frame_progress.repetition += 1;

                    // Mark that an animation repetition just ended so that the appropriate events are emitted on the next frame

                    self.repetition_just_ended = Some(cached_frame.clone());

                    // Reset the frame counter

                    if self
                        .cache
                        .repetitions
                        .map(|repetitions| self.next_frame_progress.repetition < repetitions)
                        .unwrap_or(true)
                    {
                        // PingPong: skip the first frame after the first repetition

                        self.next_frame_progress.frame = if matches!(
                            self.cache.animation_direction,
                            AnimationDirection::PingPong
                        ) {
                            1
                        } else {
                            0
                        };
                    }
                }

                (frame, current_frame_progress)
            })
    }
}
