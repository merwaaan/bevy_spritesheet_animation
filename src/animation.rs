use crate::{easing::Easing, stage::AnimationStage};
use std::fmt;

/// An opaque identifier that references an [Animation].
///
/// Returned by [SpritesheetLibrary::new_animation](crate::prelude::SpritesheetLibrary::new_animation).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct AnimationId {
    pub(crate) value: usize,
}

impl fmt::Display for AnimationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "animation:{}", self.value)
    }
}

/// Specifies the duration of an animation.
///
/// Defaults to `PerFrame(100)`.
#[derive(Debug, Clone, Copy)]
pub enum AnimationDuration {
    /// Specifies the duration of each frame in milliseconds
    PerFrame(u32),
    /// Specifies the duration of one animation cycle in milliseconds
    PerCycle(u32),
}

impl Default for AnimationDuration {
    fn default() -> Self {
        Self::PerFrame(100)
    }
}

/// Specifies how many times an [Animation] repeats.
///
/// Defaults to `AnimationRepeat::Loop`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationRepeat {
    /// Loops indefinitely
    Loop,
    /// Repeats a fixed number of times
    Times(u32),
}

impl Default for AnimationRepeat {
    fn default() -> Self {
        Self::Loop
    }
}

/// Specifies the direction of an animation.
///
/// Defaults to `AnimationDirection::Forwards`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationDirection {
    /// Frames play from left to right
    Forwards,
    /// Frames play from right to left
    Backwards,
    /// Alternates at each animation cycle, starting from left to right
    PingPong,
}

impl Default for AnimationDirection {
    fn default() -> Self {
        Self::Forwards
    }
}

/// A playable animation to assign to a [SpritesheetAnimation](crate::prelude::SpritesheetAnimation) component.
///
/// An animation is composed of one or several [AnimationStage]s.
///
/// Parameters like duration, repeat, direction and easing can be specified.
///
/// If specified, they will be combined with the parameters of the underlying [AnimationStage]s and [AnimationClip](crate::prelude::AnimationClip)s.
///
/// # Example
///
/// ```
/// # use bevy_spritesheet_animation::prelude::*;
/// # let mut library = SpritesheetLibrary::new();
/// # let some_clip_id = library.new_clip(|clip| {});
/// # let another_clip_id = library.new_clip(|clip| {});
/// let animation_id = library.new_animation(|animation| {
///     let mut stage1 = AnimationStage::from_clip(some_clip_id);
///     stage1
///         .set_duration(AnimationDuration::PerCycle(2000))
///         .set_easing(Easing::In(EasingVariety::Quadratic));
///
///     let mut stage2 = AnimationStage::from_clip(another_clip_id);
///     stage2
///         .set_repeat(10)
///         .set_direction(AnimationDirection::PingPong);
///
///     animation
///         .add_stage(stage1)
///         .add_stage(stage2)
///         .set_repeat(AnimationRepeat::Times(5));
/// });
/// ```
#[derive(Debug, Clone)]
pub struct Animation {
    /// The [AnimationStage]s that compose this animation
    stages: Vec<AnimationStage>,

    /// The optional duration of this animation
    duration: Option<AnimationDuration>,
    /// The optional number of repetitions of this animation
    repeat: Option<AnimationRepeat>,
    /// The optional direction of this animation
    direction: Option<AnimationDirection>,
    /// The optional easing of this animation
    easing: Option<Easing>,
}

impl Animation {
    pub(crate) fn new() -> Self {
        Self {
            stages: Vec::new(),
            duration: None,
            repeat: None,
            direction: None,
            easing: None,
        }
    }

    /// Adds a stage to the animation.
    ///
    /// Stages are played in the same order that they are added.
    ///
    /// # Arguments
    ///
    /// `stage` - the stage to add to the animation
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = SpritesheetLibrary::new();
    /// # let some_clip_id = library.new_clip(|clip| {});
    /// let animation_id = library.new_animation(|animation| {
    ///     // Directly add a first clip to the animation as a stage
    ///
    ///     animation.add_stage(some_clip_id.into());
    ///
    ///     // Add a second clip
    ///     //
    ///     // This time, we create an explicit stage to tweak the clip's parameters
    ///
    ///     let mut stage = AnimationStage::from_clip(some_clip_id);
    ///     stage.set_direction(AnimationDirection::Backwards);
    ///
    ///     animation.add_stage(stage);
    /// });
    /// ```
    pub fn add_stage(&mut self, stage: AnimationStage) -> &mut Self {
        self.stages.push(stage);
        self
    }

    pub fn stages(&self) -> &[AnimationStage] {
        &self.stages
    }

    pub fn set_duration(&mut self, duration: AnimationDuration) -> &mut Self {
        self.duration = Some(duration);
        self
    }

    pub fn duration(&self) -> &Option<AnimationDuration> {
        &self.duration
    }

    pub fn set_repeat(&mut self, repeat: AnimationRepeat) -> &mut Self {
        self.repeat = Some(repeat);
        self
    }

    pub fn repeat(&self) -> &Option<AnimationRepeat> {
        &self.repeat
    }

    pub fn set_direction(&mut self, direction: AnimationDirection) -> &mut Self {
        self.direction = Some(direction);
        self
    }

    pub fn direction(&self) -> &Option<AnimationDirection> {
        &self.direction
    }

    pub fn set_easing(&mut self, easing: Easing) -> &mut Self {
        self.easing = Some(easing);
        self
    }

    pub fn easing(&self) -> &Option<Easing> {
        &self.easing
    }
}
