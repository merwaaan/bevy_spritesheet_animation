use std::{sync::Arc, time::Duration};

use bevy::{log::warn, reflect::prelude::*};

use crate::{
    CRATE_NAME, animation::AnimationDirection, clip::ClipId,
    components::spritesheet_animation::AnimationProgress, messages::AnimationMarkerId,
};

use super::cache::{AnimationCache, AnimationCacheMessage, CacheFrame};

/// Same as [CacheFrame] but with `animation_repetition`
#[derive(Debug, Clone, Reflect)]
#[reflect(Debug)]
pub struct IteratorFrame {
    pub atlas_index: usize,
    pub duration: Duration,
    pub clip_id: ClipId,
    pub clip_repetition: usize,
    pub animation_repetition: usize,
    pub messages: Vec<AnimationIteratorMessage>,
}

/// A partial version of AnimationMessage.
///
/// The animation will promote them to regular AnimationMessages and add the information available at its level.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub enum AnimationIteratorMessage {
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

    /// Marks when a repetition just completed so that end messages can be emitted on the next iteration
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

    /// Promotes AnimationCacheMessages to AnimationIteratorMessages
    fn promote_messages(
        animation_messages: &[AnimationCacheMessage],
        animation_repetition: usize,
    ) -> Vec<AnimationIteratorMessage> {
        animation_messages
            .iter()
            .map(|message| match message {
                AnimationCacheMessage::MarkerHit {
                    marker_id,
                    clip_id,
                    clip_repetition,
                } => AnimationIteratorMessage::MarkerHit {
                    marker_id: *marker_id,
                    animation_repetition,
                    clip_id: *clip_id,
                    clip_repetition: *clip_repetition,
                },
                AnimationCacheMessage::ClipRepetitionEnd {
                    clip_id,
                    clip_repetition,
                } => AnimationIteratorMessage::ClipRepetitionEnd {
                    clip_id: *clip_id,
                    clip_repetition: *clip_repetition,
                },
                AnimationCacheMessage::ClipEnd { clip_id } => {
                    AnimationIteratorMessage::ClipEnd { clip_id: *clip_id }
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
                    messages: Self::promote_messages(
                        &cached_frame.messages,
                        current_frame_progress.repetition,
                    ),
                };

                // Inject the missing end messages in the returned frame

                if let Some(previous_frame) = &self.repetition_just_ended {
                    frame
                        .messages
                        .push(AnimationIteratorMessage::ClipRepetitionEnd {
                            clip_id: previous_frame.clip_id,
                            clip_repetition: previous_frame.clip_repetition,
                        });

                    frame.messages.push(AnimationIteratorMessage::ClipEnd {
                        clip_id: previous_frame.clip_id,
                    });

                    frame
                        .messages
                        .push(AnimationIteratorMessage::AnimationRepetitionEnd {
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

                    // Mark that an animation repetition just ended so that the appropriate messages are emitted on the next frame

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
