use std::sync::Arc;

use crate::animation::{AnimationDirection, AnimationId};

use super::cache::{AnimationCache, AnimationFrame, AnimationFrameEvent};

/// An animation iterator that is associated with an Entity having a [SpritesheetAnimation] component
/// by the [SpritesheetLibrary] and takes care of advancing the animation frame by frame.
///
/// next() will produce a frame until the end of the animation.
pub(super) struct AnimationIterator {
    /// The animation associated with this iterator
    ///
    /// We store this to be able to populate animation events with the ID.
    animation_id: AnimationId,

    /// Reference to the animation cache that contains all the frames for one cycle of the animation
    cache: Arc<AnimationCache>,

    /// Iteration indices
    current_frame_index: usize,
    current_animation_cycle_index: usize,

    /// Marks when a cycle just ended so that events can be emitted on the next iteration
    animation_cycle_just_ended: Option<usize>, // value = stage index
}

impl AnimationIterator {
    pub fn new(animation_id: AnimationId, cache: Arc<AnimationCache>) -> Self {
        Self {
            animation_id,
            cache,
            current_frame_index: 0,
            current_animation_cycle_index: 0,
            animation_cycle_just_ended: None,
        }
    }
}

impl Iterator for AnimationIterator {
    type Item = AnimationFrame;

    fn next(&mut self) -> Option<Self::Item> {
        // Retrieve the appropriate set of frames from the cache

        let frames = if let Some(frames_pong) = &self.cache.frames_pong {
            if self.current_animation_cycle_index % 2 == 0 {
                &self.cache.frames
            } else {
                // PingPong + odd repetitions
                frames_pong
            }
        } else {
            &self.cache.frames
        };

        // Fetch the current frame

        if let Some(mut frame) = frames.get(self.current_frame_index).cloned() {
            // Advance to the next frame

            self.current_frame_index += 1;

            // Push the various end events in the returned frame

            if let Some(stage_index) = self.animation_cycle_just_ended {
                frame.events.push(AnimationFrameEvent::ClipCycleEnd {
                    stage_index,
                    animation_id: self.animation_id,
                });

                frame.events.push(AnimationFrameEvent::ClipEnd {
                    stage_index,
                    animation_id: self.animation_id,
                });

                frame.events.push(AnimationFrameEvent::AnimationCycleEnd {
                    animation_id: self.animation_id,
                });

                self.animation_cycle_just_ended = None;
            }

            // Go back to the start of the cycle if we reached the end

            if self.current_frame_index >= self.cache.frames.len() {
                self.current_animation_cycle_index += 1;

                // Mark that an animation cycle just ended so that the appropriate events are emitted on the next frame

                self.animation_cycle_just_ended = Some(frame.stage_index);

                // Reset the frame counter

                if self
                    .cache
                    .repetitions
                    .map(|cycle_count| self.current_animation_cycle_index < cycle_count as usize)
                    .unwrap_or(true)
                {
                    // PingPong: skip the first frame

                    self.current_frame_index =
                        if matches!(self.cache.animation_direction, AnimationDirection::PingPong) {
                            1
                        } else {
                            0
                        };
                }
            }

            // Forward the frame

            Some(frame)
        } else {
            None
        }
    }
}
