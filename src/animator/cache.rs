use std::collections::HashMap;

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
    pub frames: Vec<AnimationFrame>,

    /// Frames for odd cycles when the direction is PingPong, None for other directions
    pub frames_pong: Option<Vec<AnimationFrame>>,

    // The total number of cycles to play.
    // None if infinite.
    pub cycle_count: Option<u32>,

    // The direction of the animation to handle a special case for PingPong
    pub animation_direction: AnimationDirection,
}

impl AnimationCache {
    pub fn new(animation_id: AnimationId, library: &SpritesheetLibrary) -> AnimationCache {
        // Retrieve the animation

        let animation = library
            .animations()
            .get(&animation_id)
            // In practice, this cannot fail as the library is the sole creator of animation/animation IDs
            .unwrap();

        let animation_direction = animation.direction().unwrap_or_default();

        // If the animation repeats 0 times, just create an empty cache that will play no frames

        let animation_repeat = animation.repeat().unwrap_or_default();

        if matches!(animation_repeat, AnimationRepeat::Cycles(0)) {
            return Self {
                frames: Vec::new(),
                frames_pong: None,
                cycle_count: None,
                animation_direction,
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
        // Doing so at this point will simplify what follows as well as the playback code as we won't have to handle those special cases

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
                frames: Vec::new(),
                frames_pong: None,
                cycle_count: None,
                animation_direction,
            };
        }

        // Generate all the frames that make up one full cycle of the animation
        //
        // Level 1: stages
        // Level 2: cycles
        // Level 3: frames
        //
        // This nested structure is not ideal to work with but it's convenient as it preserves the clip boundaries
        // that we need to inject events at the appropriate frames

        let mut all_cycles: Vec<Vec<Vec<AnimationFrame>>> = Vec::new();

        let mut all_cycles_pong = None;

        for (
            stage_index,
            stage_clip,
            stage_duration,
            stage_repeat,
            stage_direction,
            _stage_easing,
            stage_duration_with_repetitions_ms,
        ) in stages_data.clone()
        {
            // Adjust the actual duration of the current stage if the animation specifies its own duration

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

            // Generate all the frames for a single cycle of the current stage

            let one_cycle = stage_clip.frame_indices().iter().enumerate().map(
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

            // Repeat/reverse the cycle for all the cycles of the current stage

            let mut stage_cycles = Vec::new();

            for cycle_index in 0..stage_repeat {
                stage_cycles.push(match stage_direction {
                    AnimationDirection::Forwards => one_cycle.clone().collect_vec(),
                    AnimationDirection::Backwards => one_cycle.clone().rev().collect_vec(),
                    AnimationDirection::PingPong => {
                        // First cycle: use all the frames
                        if cycle_index == 0 {
                            one_cycle.clone().collect_vec()
                        }
                        // Following odd cycles, use all the frames but the first one, and reversed
                        else if cycle_index % 2 == 1 {
                            one_cycle.clone().rev().skip(1).collect_vec()
                        }
                        // Even cycles: use all the frames but the first one
                        else {
                            one_cycle.clone().skip(1).collect_vec()
                        }
                    }
                });
            }

            all_cycles.push(stage_cycles);
        }

        // Filter out empty stages/cycles/frames
        //
        // Removing them does not change the nature of the animation and simplifies the playback code since
        // we won't have to consider this special case.
        //
        // This must be done before attaching events or we might lose some of them!

        for stage in &mut all_cycles {
            for cycle in &mut *stage {
                cycle.retain(|frame| frame.duration > 0);
            }

            stage.retain(|cycle| cycle.len() > 0);
        }

        all_cycles.retain(|stage| stage.len() > 0);

        // Order/reverse the cycles to match the animation direction if needed

        let reverse = |all_cycles: &mut Vec<Vec<Vec<AnimationFrame>>>| {
            for stage in &mut *all_cycles {
                for cycle in &mut *stage {
                    cycle.reverse();
                }

                stage.reverse();
            }

            all_cycles.reverse();
        };

        match animation_direction {
            // Backwards: reverse all the frames
            AnimationDirection::Backwards => reverse(&mut all_cycles),

            // PingPong: reverse all the frame in the alternate "pong" collection
            AnimationDirection::PingPong => {
                all_cycles_pong = Some(all_cycles.clone());
                reverse(all_cycles_pong.as_mut().unwrap())
            }

            // Forwards: nothing to do
            _ => (),
        }

        // Merge the nested frames into a single sequence

        let merge_cycles = |cycles: &mut Vec<Vec<Vec<AnimationFrame>>>| {
            let mut all_frames = Vec::new();

            // Inject events at clip/clip cycle boundaries

            let mut previous_stage_stage_index = None;
            let mut previous_cycle_stage_index = None;

            for stage in &mut *cycles {
                for cycle in &mut *stage {
                    // Inject a ClipCycleEnd event on the first frame of each cycle after the first one

                    if let Some(stage_index) = previous_cycle_stage_index {
                        // At this point, we can safely access [0] as empty cycles have been filtered out
                        // TODO ???

                        cycle[0].events.push(AnimationFrameEvent::ClipCycleEnd {
                            stage_index,
                            animation_id,
                        });
                    }

                    previous_cycle_stage_index = Some(cycle[0].stage_index);
                }

                // Inject a ClipEnd event on the first frame of each stage after the first one
                //
                // Because we'll return None at the end of the animation, the parent Animator
                // will be responsible for generating ClipCycleEnd/ClipEnd for the last animation cycle

                if let Some(stage_index) = previous_stage_stage_index {
                    stage[0][0].events.push(AnimationFrameEvent::ClipEnd {
                        stage_index,
                        animation_id,
                    });
                }

                previous_stage_stage_index = Some(stage[0][0].stage_index);
            }

            // Build a (stage index, easing) record

            let stages_easing: HashMap<usize, Easing> = HashMap::from_iter(
                stages_data
                    .clone()
                    .map(|(stage_index, _, _, _, _, stage_easing, _)| (stage_index, stage_easing)),
            );

            // Merge the nested frames into a single sequence

            for stage in cycles {
                let mut stage_frames = Vec::new();

                for cycle in stage {
                    stage_frames.extend(cycle.clone());
                }

                // Apply easing on the stage

                let stage_index = stage_frames[0].stage_index;
                let easing = stages_easing[&stage_index];

                apply_easing(&mut stage_frames, easing);

                all_frames.extend(stage_frames.clone());
            }

            // Apply easing on the whole animation

            let animation_easing = animation.easing().unwrap_or(Easing::default());

            apply_easing(&mut all_frames, animation_easing);

            all_frames
        };

        let all_frames = merge_cycles(&mut all_cycles);

        let all_frames_pong = if let Some(cycles) = &mut all_cycles_pong {
            Some(merge_cycles(cycles))
        } else {
            None
        };

        // Compute the total number of stages (taking repetitions into account)

        let animation_repeat = animation.repeat().unwrap_or_default();

        let cycle_count = match animation_repeat {
            AnimationRepeat::Loop => None,
            AnimationRepeat::Cycles(n) => Some(n),
        };

        // Done!

        Self {
            frames: all_frames,
            frames_pong: all_frames_pong,
            cycle_count,
            animation_direction,
        }
    }
}

fn apply_easing(frames: &mut Vec<AnimationFrame>, easing: Easing) {
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
