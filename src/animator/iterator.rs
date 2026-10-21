use std::{sync::Arc, time::Duration};

use bevy::prelude::*;

use crate::{
    CRATE_NAME,
    animation::AnimationDirection,
    animator::cache::{AnimationCache, AnimationCacheEvent, CacheFrame},
    clip::ClipId,
    components::spritesheet_animation::AnimationProgress,
    events::Marker,
};

/// Same as [CacheFrame] but with `animation_repetition`
#[derive(Debug, Clone, Copy, Reflect)]
#[reflect(Debug)]
pub(crate) struct IteratorFrame {
    pub atlas_index: usize,
    pub duration: Duration,
    pub clip_id: ClipId,
    pub clip_repetition: usize,
    pub animation_repetition: usize,

    /// Set on the first frame after an animation repetition ended so that the matching end events are emitted
    pub ended_repetition: Option<EndedRepetition>,
}

/// The animation repetition that just ended before an [IteratorFrame]
#[derive(Debug, Clone, Copy, Reflect)]
#[reflect(Debug)]
pub(crate) struct EndedRepetition {
    pub clip_id: ClipId,
    pub clip_repetition: usize,
    pub animation_repetition: usize,
}

/// A partial version of AnimationEvent.
///
/// The animation will promote them to regular AnimationEvents and add the information available at its level.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub(crate) enum AnimationIteratorEvent {
    MarkerHit {
        marker: Marker,
        animation_repetition: usize,
        clip_id: ClipId,
        clip_repetition: usize,
    },
    ClipRepetitionEnd {
        clip_id: ClipId,
        clip_repetition: usize,
        animation_repetition: usize,
    },
    ClipEnd {
        clip_id: ClipId,
        animation_repetition: usize,
    },
    AnimationRepetitionEnd {
        animation_repetition: usize,
    },
}

/// An iterator that advances an animation frame by frame.
///
/// `next()` will produce frames until the end of the animation.
#[derive(Debug, Reflect)]
#[reflect(Debug)]
pub(crate) struct AnimationIterator {
    /// Reference to the animation cache that contains all the frames for one repetition of the animation
    cache: Arc<AnimationCache>,

    /// Current iteration progress
    next_frame_progress: AnimationProgress,

    /// Marks when a repetition just completed so that end events can be emitted on the next iteration
    /// (the value is the clip ID and clip repetition of the last frame)
    repetition_just_ended: Option<(ClipId, usize)>,
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
            error!(
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
            error!(
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

    /// Returns the events of a frame.
    ///
    /// The cached events are promoted on the fly and the end events of the previous animation repetition are appended.
    pub fn events<'a>(
        &'a self,
        frame: &IteratorFrame,
        progress: AnimationProgress,
    ) -> impl Iterator<Item = AnimationIteratorEvent> + 'a {
        let animation_repetition = frame.animation_repetition;

        let cached_events = Self::frames_for(&self.cache, progress.repetition)
            .get(progress.frame)
            .map(|cached_frame| cached_frame.events.as_slice())
            .unwrap_or_default()
            .iter()
            .map(move |event| Self::promote_event(event, animation_repetition));

        let end_events = frame.ended_repetition.into_iter().flat_map(|ended| {
            [
                AnimationIteratorEvent::ClipRepetitionEnd {
                    clip_id: ended.clip_id,
                    clip_repetition: ended.clip_repetition,
                    animation_repetition: ended.animation_repetition,
                },
                AnimationIteratorEvent::ClipEnd {
                    clip_id: ended.clip_id,
                    animation_repetition: ended.animation_repetition,
                },
                AnimationIteratorEvent::AnimationRepetitionEnd {
                    animation_repetition: ended.animation_repetition,
                },
            ]
        });

        cached_events.chain(end_events)
    }

    /// Retrieves the appropriate frame set from the cache for the given animation repetition
    fn frames_for(cache: &AnimationCache, animation_repetition: usize) -> &[CacheFrame] {
        if let Some(frames_pong) = &cache.frames_pong {
            if animation_repetition.is_multiple_of(2) {
                // Regular frames for even PingPong repetitions
                &cache.frames
            } else {
                // Frames for odd PingPong repetitions
                frames_pong
            }
        } else {
            // Regular frames
            &cache.frames
        }
    }

    /// Promotes an AnimationCacheEvent to an AnimationIteratorEvent
    fn promote_event(
        event: &AnimationCacheEvent,
        animation_repetition: usize,
    ) -> AnimationIteratorEvent {
        match event {
            AnimationCacheEvent::MarkerHit {
                marker,
                clip_id,
                clip_repetition,
            } => AnimationIteratorEvent::MarkerHit {
                marker: *marker,
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
                animation_repetition,
            },
            AnimationCacheEvent::ClipEnd { clip_id } => AnimationIteratorEvent::ClipEnd {
                clip_id: *clip_id,
                animation_repetition,
            },
        }
    }
}

impl Iterator for AnimationIterator {
    type Item = (IteratorFrame, AnimationProgress);

    fn next(&mut self) -> Option<Self::Item> {
        // Fetch the current frame

        let cached_frames = Self::frames_for(&self.cache, self.next_frame_progress.repetition);

        cached_frames
            .get(self.next_frame_progress.frame)
            .map(|cached_frame| {
                let current_frame_progress = self.next_frame_progress;

                // Promote the frame with the current animation repetition
                //
                // Also mark the missing end events of the previous repetition so that they are emitted along with this frame's events

                let frame = IteratorFrame {
                    atlas_index: cached_frame.atlas_index,
                    duration: cached_frame.duration,
                    clip_id: cached_frame.clip_id,
                    clip_repetition: cached_frame.clip_repetition,
                    animation_repetition: current_frame_progress.repetition,
                    ended_repetition: self.repetition_just_ended.take().map(
                        |(clip_id, clip_repetition)| EndedRepetition {
                            clip_id,
                            clip_repetition,
                            animation_repetition: current_frame_progress
                                .repetition
                                .saturating_sub(1),
                        },
                    ),
                };

                // Increment the indices for the next iteration

                self.next_frame_progress.frame += 1;

                // Go back to the start if we reached the end

                if self.next_frame_progress.frame >= self.cache.frames.len() {
                    self.next_frame_progress.repetition += 1;

                    // Mark that an animation repetition just ended so that the appropriate events are emitted on the next frame

                    self.repetition_just_ended =
                        Some((cached_frame.clip_id, cached_frame.clip_repetition));

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
