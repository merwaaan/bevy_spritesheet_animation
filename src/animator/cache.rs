use crate::{
    animation::{AnimationDirection, AnimationDuration, AnimationId, AnimationRepeat},
    clip::{Clip, ClipId},
    easing::Easing,
    library::AnimationLibrary,
    messages::AnimationMarkerId,
    CRATE_NAME,
};
use bevy::{log::warn, reflect::prelude::*};
use std::time::Duration;

/// A pre-computed frame of animation, ready to be played back.
#[derive(Debug, Clone, Reflect)]
#[reflect(Debug)]
pub struct CacheFrame {
    pub atlas_index: usize,
    pub duration: Duration,
    pub clip_id: ClipId,
    pub clip_repetition: usize,
    pub messages: Vec<AnimationCacheMessage>,
}

/// A partial version of AnimationMessage.
///
/// The iterator will promote them to regular AnimationIteratorMessages and
/// add the information available at its level.
///
/// The iterator & animator will also generate extra messages that cannot be cached:
///  - ClipRepetitionEnd, ClipEnd, AnimationRepetitionEnd on the first frame of the repetitions
///  - AnimationEnd after the last frame
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub enum AnimationCacheMessage {
    MarkerHit {
        marker_id: AnimationMarkerId,
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
}

#[derive(Debug, Reflect)]
#[reflect(Debug)]
/// The [AnimationCache] contains pre-computed frames for an animation.
///
/// The idea is to cache for each frame its atlas index, duration and emitted messages
/// so that playing an animation becomes just a matter of iterating over this cache
/// without re-evaluating all the animation  parameters.
pub struct AnimationCache {
    /// All the frames
    pub frames: Vec<CacheFrame>,

    /// Frames for odd repetitions when the direction is PingPong.
    /// None for other directions.
    pub frames_pong: Option<Vec<CacheFrame>>,

    /// The total number of repetitions to play.
    /// None if looping indefinitely.
    pub repetitions: Option<usize>,

    /// The direction of the animation to handle the PingPong case
    /// (after the first repetition, the first frame must be skipped)
    pub animation_direction: AnimationDirection,
}

impl AnimationCache {
    fn empty() -> Self {
        Self {
            frames: Vec::new(),
            frames_pong: None,
            repetitions: None,
            animation_direction: AnimationDirection::Forwards,
        }
    }

    pub fn new(animation_id: AnimationId, library: &AnimationLibrary) -> AnimationCache {
        let animation = library.get_animation(animation_id);

        // If the animation repeats 0 times, just create an empty cache that will play no frames
        // TODO should use the first frame only instead?

        let animation_repetitions = animation.repetitions().unwrap_or_default();

        if matches!(animation_repetitions, AnimationRepeat::Times(0)) {
            return Self::empty();
        }

        // Gather data for all the clips

        let clips_data = animation
            .clip_ids()
            .iter()
            .map(|clip_id| ClipData::new(*clip_id, library))
            // Filter out clips with 0 frames / 0 repetitions / durations of 0
            //
            // Doing so at this point will simplify what follows as well as the playback code as we won't have to handle those special cases
            .filter(|data| {
                !data.clip.frames().is_empty()
                    && data.repetitions > 0
                    && data.duration_with_repetitions_ms > 0
            });

        // Compute the total duration of one cycle of the animation in milliseconds

        let animation_duration_ms: u32 = clips_data
            .clone()
            .map(|data| data.duration_with_repetitions_ms)
            .sum();

        // If the animation lasts 0 ms, just create an empty cache that will play no frames
        // TODO should use the first frame only instead?

        if animation_duration_ms == 0 {
            return Self::empty();
        }

        // Generate the full animation from all the clips

        let clip_frames = clips_data
            .map(|clip_data| {
                // Adjust the actual duration of the current clip if the animation specifies its own duration

                let clip_corrected_duration = match animation.duration() {
                    // No duration is defined for the animation: keep the clip's duration
                    None => clip_data.duration,

                    // The per-frame duration is defined for the animation: override the clip's duration with it
                    Some(AnimationDuration::PerFrame(animation_frame_duration)) => {
                        AnimationDuration::PerFrame(*animation_frame_duration)
                    }

                    // The per-cycle duration of the animation is defined:
                    // assign a duration to the clip that stays proportional to its base duration with respect to the total animation duration
                    Some(AnimationDuration::PerRepetition(animation_cycle_duration)) => {
                        let clip_ratio = clip_data.duration_with_repetitions_ms as f32
                            / animation_duration_ms as f32;

                        AnimationDuration::PerRepetition(
                            (*animation_cycle_duration as f32 * clip_ratio
                                / clip_data.repetitions as f32) as u32,
                        )
                    }
                };

                // Compute the duration of a single frame

                let clip_frame_corrected_duration_ms = match clip_corrected_duration {
                    AnimationDuration::PerFrame(frame_duration_ms) => frame_duration_ms,
                    AnimationDuration::PerRepetition(cycle_duration_ms) => {
                        cycle_duration_ms / clip_data.clip.frames().len() as u32
                    }
                };

                // Generate the frames for the current clip

                ClipFrames::new(clip_data, clip_frame_corrected_duration_ms)
            })
            .collect();

        let animation_frames = AnimationFrames::new(clip_frames);

        let animation_direction = animation.direction().unwrap_or_default();
        let animation_easing = animation.easing().unwrap_or_default();

        let (all_frames, all_frames_pong) =
            animation_frames.build(animation_direction, animation_easing);

        // Done!

        let animation_repetition_count = match animation_repetitions {
            AnimationRepeat::Loop => None,
            AnimationRepeat::Times(n) => Some(n),
        };

        Self {
            frames: all_frames,
            frames_pong: all_frames_pong,
            repetitions: animation_repetition_count,
            animation_direction,
        }
    }
}

#[derive(Clone)]
struct ClipData {
    id: ClipId,
    clip: Clip,
    duration: AnimationDuration,
    repetitions: usize,
    direction: AnimationDirection,
    easing: Easing,
    duration_with_repetitions_ms: u32,
}

impl ClipData {
    fn new(clip_id: ClipId, library: &AnimationLibrary) -> Self {
        let clip = library.get_clip(clip_id).clone();

        let duration = clip.duration().unwrap_or_default();
        let repetitions = clip.repetitions().unwrap_or(1);
        let direction = clip.direction().unwrap_or_default();
        let easing = clip.easing().unwrap_or_default();

        // Compute the clip's duration in milliseconds, taking repetitions into account

        let frame_count_with_repetitions = match direction {
            AnimationDirection::Forwards | AnimationDirection::Backwards => {
                clip.frames().len() as u32 * repetitions as u32
            }
            AnimationDirection::PingPong => {
                clip.frames().len().saturating_sub(1) as u32 * repetitions as u32 + 1
            }
        };

        let duration_with_repetitions_ms = match duration {
            AnimationDuration::PerFrame(frame_duration) => {
                frame_duration * frame_count_with_repetitions
            }
            AnimationDuration::PerRepetition(repetition_duration) => repetition_duration,
        };

        Self {
            id: clip_id,
            clip,
            duration,
            repetitions,
            direction,
            easing,
            duration_with_repetitions_ms,
        }
    }
}

// Helper structures to build the full animation

#[derive(Clone)]
struct Frame {
    atlas_index: usize,
    duration: Duration,
    markers: Vec<AnimationMarkerId>,
}

#[derive(Clone)]
struct ClipRepetitionFrames {
    frames: Vec<Frame>,
}

impl ClipRepetitionFrames {
    fn new(clip_data: &ClipData, frame_duration_ms: u32) -> Self {
        Self {
            frames: clip_data
                .clip
                .frames()
                .iter()
                .enumerate()
                .map(move |(frame_index, frame_atlas_index)| {
                    // Collect the markers for the current frame

                    let markers = clip_data
                        .clip
                        .markers()
                        .get(&frame_index)
                        .cloned()
                        .unwrap_or(Vec::new());

                    Frame {
                        atlas_index: *frame_atlas_index,
                        duration: Duration::from_millis(frame_duration_ms as u64),
                        markers,
                    }
                })
                // Filter out frames with no duration
                .filter(|frame| !frame.duration.is_zero())
                .collect(),
        }
    }

    fn backwards(&self) -> Self {
        Self {
            frames: self.frames.iter().rev().cloned().collect(),
        }
    }

    fn ping(&self) -> Self {
        Self {
            frames: self.frames.iter().skip(1).cloned().collect(),
        }
    }

    fn pong(&self) -> Self {
        Self {
            frames: self.frames.iter().rev().skip(1).cloned().collect(),
        }
    }
}

#[derive(Clone)]
struct ClipFrames {
    repetitions: Vec<ClipRepetitionFrames>,
    data: ClipData,
}

impl ClipFrames {
    fn new(clip_data: ClipData, frame_duration_override_ms: u32) -> Self {
        let reference_repetition =
            ClipRepetitionFrames::new(&clip_data, frame_duration_override_ms);

        Self {
            repetitions: (0..clip_data.repetitions)
                .map(|repetition| {
                    match clip_data.direction {
                        AnimationDirection::Forwards => reference_repetition.clone(),
                        AnimationDirection::Backwards => reference_repetition.backwards(),
                        AnimationDirection::PingPong => {
                            if repetition == 0 {
                                // First ping cycle: use all the frames (ping() would remove the first one)
                                reference_repetition.clone()
                            } else if repetition % 2 == 0 {
                                reference_repetition.ping()
                            } else {
                                reference_repetition.pong()
                            }
                        }
                    }
                })
                // Filter out repetitions with no frames
                .filter(|repetition| !repetition.frames.is_empty())
                .collect(),
            data: clip_data,
        }
    }

    fn backwards(&self) -> Self {
        Self {
            repetitions: self
                .repetitions
                .iter()
                // Reverse each clip internally
                .map(|clip| clip.backwards())
                // Reverse the clip order
                .rev()
                .collect(),
            data: self.data.clone(),
        }
    }
}

#[derive(Default, Clone)]
struct AnimationFrames {
    clips: Vec<ClipFrames>,
}

impl AnimationFrames {
    fn new(clips: Vec<ClipFrames>) -> Self {
        Self {
            clips: clips
                .iter()
                // Filter out repetitions with no repetitions
                .filter(|clip| !clip.repetitions.is_empty())
                .cloned()
                .collect(),
        }
    }

    fn backwards(&self) -> Self {
        Self {
            clips: self
                .clips
                .iter()
                .map(|clip| clip.backwards())
                .rev()
                .collect(),
        }
    }

    fn build(
        &self,
        direction: AnimationDirection,
        easing: Easing,
    ) -> (Vec<CacheFrame>, Option<Vec<CacheFrame>>) {
        // Returns (regular frames, maybe pong frames)

        // Order the frames depending on the direction of the animation

        let (animation_frames, animation_frames_pong) = match direction {
            // Forwards: just use the frames as-is
            AnimationDirection::Forwards => (self.clone(), None),

            // Backwards: reverse all the frames
            AnimationDirection::Backwards => (self.backwards(), None),

            // PingPong: reverse ALL the frame in the alternate "pong" collection
            // (all the frame because the iterator will skip the first frame of all the ping & pong repetitions after the first one)
            AnimationDirection::PingPong => (self.clone(), Some(self.backwards())),
        };

        // Assemble the nested animation/clip/repetition tree into a single sequence of frames

        let merge = |mut frames: AnimationFrames| {
            let mut all_frames = Vec::new();

            let mut previous_clip = None;
            let mut previous_clip_repetition = None;

            for clip in &mut frames.clips {
                let mut all_clip_frames = Vec::new();

                for (repetition_index, repetition) in clip.repetitions.iter_mut().enumerate() {
                    // Apply easing to the clip repetition

                    let clip_frame_durations = repetition
                        .frames
                        .iter_mut()
                        .map(|frame| &mut frame.duration)
                        .collect();

                    apply_easing(clip_frame_durations, clip.data.easing);

                    // Convert to runtime AnimationFrames

                    let mut clip_frames: Vec<_> = repetition
                        .frames
                        .iter()
                        .map(|frame| CacheFrame {
                            atlas_index: frame.atlas_index,
                            duration: frame.duration,
                            clip_id: clip.data.id,
                            clip_repetition: repetition_index,
                            // Convert the markers to messages
                            messages: frame
                                .markers
                                .iter()
                                .map(|marker| AnimationCacheMessage::MarkerHit {
                                    marker_id: *marker,
                                    clip_id: clip.data.id,
                                    clip_repetition: repetition_index,
                                })
                                .collect(),
                        })
                        .collect();

                    // Inject a ClipRepetitionEnd message on the first frame of each repetition after the first one

                    if let Some((previous_clip_id, previous_clip_repetition)) =
                        previous_clip_repetition
                    {
                        // At this point, we can safely access [0] as empty cycles have been filtered out
                        clip_frames[0]
                            .messages
                            .push(AnimationCacheMessage::ClipRepetitionEnd {
                                clip_id: previous_clip_id,
                                clip_repetition: previous_clip_repetition,
                            });
                    }

                    previous_clip_repetition = Some((clip.data.id, repetition_index));

                    // Merge with the full clip

                    all_clip_frames.extend(clip_frames);
                }

                // Inject a ClipEnd message on the first frame of each clip after the first one
                //
                // Because we'll return None at the end of the animation, the Animator will be
                // responsible for generating ClipRepetitionEnd/ClipEnd for the last animation cycle

                if let Some(previous_clip_id) = previous_clip {
                    all_clip_frames[0]
                        .messages
                        .push(AnimationCacheMessage::ClipEnd {
                            clip_id: previous_clip_id,
                        });
                }

                previous_clip = Some(clip.data.id);

                // Merge with the full animation

                all_frames.extend(all_clip_frames);
            }

            // Apply easing on the whole animation

            let animation_frame_durations = all_frames
                .iter_mut()
                .map(|frame| &mut frame.duration)
                .collect();

            apply_easing(animation_frame_durations, easing);

            all_frames
        };

        (merge(animation_frames), animation_frames_pong.map(merge))
    }
}

fn apply_easing(frame_durations: Vec<&mut Duration>, easing: Easing) {
    // Linear easing: there's nothing to do

    if matches!(easing, Easing::Linear) {
        return;
    }

    // If the total duration is zero, exit early to prevent arithmetic errors

    let total_duration_ms: u32 = frame_durations.iter().map(|d| d.as_millis() as u32).sum();

    if total_duration_ms == 0 {
        warn!("{CRATE_NAME}: zero duration, cannot apply easing");

        return;
    }

    // Apply the easing

    let mut accumulated_time = 0;
    let mut previous_eased_time = 0.0;

    for frame_duration in frame_durations {
        // Apply the easing

        let normalized_time = accumulated_time as f32 / total_duration_ms as f32;

        let normalized_eased_time = easing.get(normalized_time);

        // Convert back to a duration

        let eased_time = normalized_eased_time * total_duration_ms as f32;

        let eased_duration = (eased_time - previous_eased_time) as u32;

        accumulated_time += frame_duration.as_millis();
        previous_eased_time = eased_time;

        // Update the frame

        *frame_duration = Duration::from_millis(eased_duration as u64);
    }
}
