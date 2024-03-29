use itertools::Itertools;

use crate::{
    animation::{AnimationDirection, AnimationDuration, AnimationId, AnimationRepeat},
    easing::Easing,
    events::AnimationMarkerId,
    library::SpritesheetLibrary,
};

/// A partial version of AnimationEvent.
///
/// The SpritesheetAnimator will promote them to regular AnimationEvents,
/// adding the information available at its level (entity).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum AnimationFrameEvent {
    MarkerHit {
        marker_id: AnimationMarkerId,
        stage_index: usize,
        animation_id: AnimationId,
    },
    ClipCycleEnd {
        stage_index: usize,
        animation_id: AnimationId,
    },
    ClipEnd {
        stage_index: usize,
        animation_id: AnimationId,
    },
    AnimationCycleEnd {
        animation_id: AnimationId,
    },
}

/// A single frame of animation, ready to be played back.
#[derive(Debug, Clone)]
pub(super) struct AnimationFrame {
    pub atlas_index: usize,
    pub duration: u32,
    pub events: Vec<AnimationFrameEvent>,
    pub stage_index: usize,
}

/// The [AnimationCache] contains pre-computed frames for an animation.
///
/// The idea is to cache for each frame its atlas index, stage index, duration and emitted events
/// so that playing an animation becomes just a matter of iterating over this cache without the need
/// to re-evaluate its parameters.
pub(super) struct AnimationCache {
    /// All the frames
    pub frames_ping: Vec<AnimationFrame>,

    /// Frames for odd cycles when the direction is PingPong, None for other directions
    pub frames_pong: Option<Vec<AnimationFrame>>,

    // The total number of cycles to play.
    // None if infinite.
    pub cycle_count: Option<u32>,
}

impl AnimationCache {
    pub fn new(animation_id: AnimationId, library: &SpritesheetLibrary) -> AnimationCache {
        let animation = library
            .animations()
            .get(&animation_id)
            // In practice, this cannot fail as the library is the sole creator of animation/animation IDs
            .unwrap();

        // If the animation repeats 0 times, just create an empty cache that will play no frames

        let animation_repeat = animation.repeat().unwrap_or_default();

        if matches!(animation_repeat, AnimationRepeat::Cycles(0)) {
            return Self {
                frames_ping: Vec::new(),
                frames_pong: None,
                cycle_count: None,
            };
        }

        // Gather the data for all the stages

        let stages_data = animation
            .stages()
            .iter()
            .enumerate()
            .map(|(stage_index, stage)| {
                let stage_clip = library
                    .clips()
                    .get(stage.clip_id())
                    // In practice, this cannot fail as the library is the sole creator of clip/clip IDs
                    .unwrap();

                let stage_duration = stage
                    // Use the stage's duration first
                    .duration()
                    // Fallback to the clip's default duration
                    .or(*stage_clip.default_duration())
                    // Fallback to the global default value
                    .unwrap_or(AnimationDuration::default());

                let stage_repeat = stage
                    // Use the stage's repetitions first
                    .repeat()
                    // Fallback to the clip's default repetitions
                    .or(*stage_clip.default_repeat())
                    // Fallback to a default value
                    .unwrap_or(1);

                let stage_direction = stage
                    // Use the stage's direction first
                    .direction()
                    // Fallback to the clip's default direction
                    .or(*stage_clip.default_direction())
                    // Fallback to the global default value
                    .unwrap_or(AnimationDirection::default());

                let stage_easing = stage
                    // Use the stage's easing first
                    .easing()
                    // Fallback to the clip's default easing
                    .or(*stage_clip.default_easing())
                    // Fallback to the global default value
                    .unwrap_or(Easing::default());

                // Compute the stage's duration in milliseconds, taking repetitions into account

                let frame_count_with_repetitions = match stage_direction {
                    AnimationDirection::Forwards | AnimationDirection::Backwards => {
                        stage_clip.frame_count() as u32 * stage_repeat
                    }
                    AnimationDirection::PingPong => {
                        stage_clip.frame_count().saturating_sub(1) as u32 * stage_repeat + 1
                    }
                };

                let stage_duration_with_repetitions_ms = match stage_duration {
                    AnimationDuration::PerFrame(frame_duration) => {
                        frame_duration * frame_count_with_repetitions as u32
                    }
                    AnimationDuration::PerCycle(cycle_duration) => cycle_duration,
                };

                (
                    stage_index,
                    stage_clip,
                    stage_duration,
                    stage_repeat,
                    stage_direction,
                    stage_easing,
                    stage_duration_with_repetitions_ms,
                )
            });

        // Filter out stages with 0 frames / durations of 0
        //
        // Doing so at this stage will simplify what follows as well as the playback code as we won't have to handle those special cases

        let stages_data = stages_data.filter(
            |(_, stage_clip, _, _, _, _, stage_duration_with_repetitions_ms)| {
                stage_clip.frame_count() > 0 && *stage_duration_with_repetitions_ms > 0
            },
        );

        // Compute the total duration of one cycle of the animation in milliseconds

        let animation_cycle_duration_ms = stages_data
            .clone()
            .map(|(_, _, _, _, _, _, stage_duration_with_repetitions_ms)| {
                stage_duration_with_repetitions_ms
            })
            .sum::<u32>();

        // If the animation lasts 0 ms, just create an empty cache that will play no frames

        if animation_cycle_duration_ms == 0 {
            return Self {
                frames_ping: Vec::new(),
                frames_pong: None,
                cycle_count: None,
            };
        }

        // Generate all the frames that make up one cycle of the animation

        let mut frames_ping: Vec<AnimationFrame> = Vec::new();

        // Track when the previous "valid" stage so that we can reference it in the end events added at the start of the following stages
        let mut previous_valid_stage_index: Option<usize> = None;

        for (
            stage_index,
            stage_clip,
            stage_duration,
            stage_repeat,
            stage_direction,
            stage_easing,
            stage_duration_with_repetitions_ms,
        ) in stages_data
        {
            let mut current_stage_frames: Vec<AnimationFrame> = Vec::new();

            // Adjust the actual duration of the stage if the animation specifies its own duration

            let stage_corrected_duration = match animation.duration() {
                // No duration is defined for the animation: keep the stage's duration
                None => stage_duration,

                // The per-frame duration is defined for the animation: override the stage's duration with it
                Some(AnimationDuration::PerFrame(animation_frame_duration)) => {
                    AnimationDuration::PerFrame(*animation_frame_duration)
                }

                // The per-cycle duration of the animation is defined:
                // assign a duration to the stage that stays proportional to its base duration with respect to the total animation duration
                Some(AnimationDuration::PerCycle(animation_cycle_duration)) => {
                    let stage_ratio = stage_duration_with_repetitions_ms as f32
                        / animation_cycle_duration_ms as f32;

                    AnimationDuration::PerCycle(
                        (*animation_cycle_duration as f32 * stage_ratio / stage_repeat as f32)
                            as u32,
                    )
                }
            };

            // Compute the duration of a single frame

            let stage_frame_duration_ms = match stage_corrected_duration {
                AnimationDuration::PerFrame(frame_duration_ms) => frame_duration_ms,
                AnimationDuration::PerCycle(cycle_duration_ms) => {
                    cycle_duration_ms / stage_clip.frame_count() as u32
                }
            };

            // Generate all the frames for a single cycle of the stage

            let one_cycle_frames = stage_clip.frame_indices().iter().enumerate().map(
                move |(frame_index, atlas_index)| {
                    // Convert this frame's markers into events to emit when reaching it

                    let events = stage_clip
                        .markers()
                        .get(&frame_index)
                        .map(|frame_markers| {
                            frame_markers
                                .iter()
                                .map(|marker| AnimationFrameEvent::MarkerHit {
                                    marker_id: *marker,
                                    stage_index,
                                    animation_id,
                                })
                                .collect()
                        })
                        .unwrap_or(Vec::new());

                    AnimationFrame {
                        atlas_index: *atlas_index,
                        duration: stage_frame_duration_ms,
                        events,
                        stage_index,
                    }
                },
            );

            // Repeat and reverse the cycle into frames for all the cycles of the stage

            for cycle_index in 0..stage_repeat {
                let mut current_cycle_frames = match stage_direction {
                    AnimationDirection::Forwards => one_cycle_frames.clone().collect_vec(),
                    AnimationDirection::Backwards => one_cycle_frames.clone().rev().collect_vec(),
                    AnimationDirection::PingPong => {
                        // First cycle: use all the frames
                        if cycle_index == 0 {
                            one_cycle_frames.clone().collect_vec()
                        }
                        // Following odd cycles, use all the frames but the first one, and reversed
                        else if cycle_index % 2 == 1 {
                            one_cycle_frames.clone().rev().skip(1).collect_vec()
                        }
                        // Even cycles: use all the frames but the first one
                        else {
                            one_cycle_frames.clone().skip(1).collect_vec()
                        }
                    }
                };

                // Inject a ClipCycleEnd event on the first frame of each cycle after the first one

                if cycle_index > 0 {
                    if let Some(cycle_first_frame) = current_cycle_frames.get_mut(0) {
                        cycle_first_frame
                            .events
                            .push(AnimationFrameEvent::ClipCycleEnd {
                                stage_index,
                                animation_id,
                            });
                    }
                }

                current_stage_frames.extend(current_cycle_frames);
            }

            // Inject end events on the first frame of each stage after the first one
            //
            // Because we'll return None at the end of the animation, the parent Animator
            // will be responsible for generating this event for the last animation cycle

            if let Some(previous_stage_index) = previous_valid_stage_index {
                if let Some(stage_first_frame) = current_stage_frames.get_mut(0) {
                    // The last ClipCycleEnd event

                    stage_first_frame
                        .events
                        .push(AnimationFrameEvent::ClipCycleEnd {
                            stage_index: previous_stage_index,
                            animation_id,
                        });

                    // The ClipEnd event

                    stage_first_frame.events.push(AnimationFrameEvent::ClipEnd {
                        stage_index: previous_stage_index,
                        animation_id,
                    });
                }
            }

            previous_valid_stage_index = Some(stage_index);

            // Apply easing on the stage

            apply_easing(&mut current_stage_frames, &stage_easing);

            // Push the stage's frames to the animation's frames

            frames_ping.extend(current_stage_frames);
        }

        // Apply easing on the whole animation

        let animation_easing = animation.easing().unwrap_or(Easing::default());

        apply_easing(&mut frames_ping, &animation_easing);

        // Filter out frames with a duration of 0 that could have ended here because of floating-point errors.
        //
        // Removing them does not change the nature of the animation and simplifies the playback code since
        // we won't have to consider this special case.

        frames_ping.retain(|frame| frame.duration > 0);

        // Apply the direction on the whole animation

        let animation_direction = animation.direction().unwrap_or_default();

        let mut frames_pong = None;

        match animation_direction {
            // Backward: reverse the frames
            AnimationDirection::Backwards => frames_ping.reverse(),

            // PingPong: copy and reverse the frames in a second cache to be played on odd cycles
            AnimationDirection::PingPong => {
                frames_pong = Some(frames_ping.iter().cloned().rev().collect_vec())
            }
            _ => (),
        }

        // Compute the total number of stages (taking repetitions into account)

        let animation_repeat = animation.repeat().unwrap_or_default();

        let cycle_count = match animation_repeat {
            AnimationRepeat::Loop => None,
            AnimationRepeat::Cycles(n) => Some(n),
        };

        Self {
            frames_ping,
            frames_pong,
            cycle_count,
        }
    }
}

fn apply_easing(frames: &mut Vec<AnimationFrame>, easing: &Easing) {
    // Linear easing: exit early, there's nothing to do

    if matches!(easing, Easing::Linear) {
        return;
    }

    // Compute the total duration of the sequence

    let total_duration_ms: u32 = frames.iter().map(|frame| frame.duration).sum();

    // If the total duration is zero, exit early to prevent arithmetic errors

    if total_duration_ms == 0 {
        return;
    }

    // Apply the easing

    let mut accumulated_time = 0;
    let mut previous_eased_time = 0.0;

    for frame in frames {
        // Convert the duration to a normalized time

        let normalized_time = accumulated_time as f32 / total_duration_ms as f32;

        // Apply the easing

        let normalized_eased_time = easing.get(normalized_time);

        // Convert back to a duration

        let eased_time = normalized_eased_time * total_duration_ms as f32;

        let eased_duration = (eased_time - previous_eased_time) as u32;

        accumulated_time += frame.duration;
        previous_eased_time = eased_time;

        // Update the frame

        frame.duration = eased_duration;
    }
}
