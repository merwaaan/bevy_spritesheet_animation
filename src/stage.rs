use crate::{
    animation::{AnimationDirection, AnimationDuration},
    clip::AnimationClipId,
    easing::Easing,
};

/// A stage of an [Animation](crate::prelude::Animation).
///
/// All animations are made of stages.
/// Simple animations may contain a single stage while more complex animations may contain a sequence of stages.
///
/// An [AnimationStage] references an [AnimationClip](crate::prelude::AnimationClip) that has been created with [SpritesheetLibrary::new_clip](crate::prelude::SpritesheetLibrary::new_clip).
///
/// Parameters like duration, repeat and direction can be optionally specified.
/// If specified, they will override the default parameters of the underlying [AnimationClip](crate::prelude::AnimationClip).
#[derive(Debug, Clone)]
pub struct AnimationStage {
    // The ID of the [AnimationClip] that this stage plays
    clip_id: AnimationClipId,

    /// The optional duration of this stage
    duration: Option<AnimationDuration>,
    /// The optional repetitions of this stage
    repeat: Option<u32>,
    /// The optional direction of this stage
    direction: Option<AnimationDirection>,
    /// The optional easing of this stage
    easing: Option<Easing>,
}

impl AnimationStage {
    /// Creates a new [AnimationStage] from an [AnimationClipId] with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `clip_id` - the ID of the clip referenced by this stage
    /// * `duration` - the optional duration of this stage
    /// * `repeat` - the optional number of repetitions of this stage
    /// * `direction` - the optional direction of this stage
    /// * `easing` - the optional easing of this stage
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = SpritesheetLibrary::new();
    /// # let clip_id = library.new_clip(|clip| {});
    /// let animation_id = library.new_animation(|animation| {
    ///     let mut stage = AnimationStage::new(
    ///         clip_id,
    ///         Some(AnimationDuration::PerCycle(2500)),
    ///         Some(10),
    ///         Some(AnimationDirection::Backwards),
    ///         Some(Easing::Linear)
    ///     );
    ///
    ///     animation.add_stage(stage);
    /// });
    /// ```
    pub fn new(
        clip_id: AnimationClipId,
        duration: Option<AnimationDuration>,
        repeat: Option<u32>,
        direction: Option<AnimationDirection>,
        easing: Option<Easing>,
    ) -> Self {
        Self {
            clip_id,
            duration,
            repeat,
            direction,
            easing,
        }
    }

    /// Creates a new [AnimationStage] from an [AnimationClipId] and a builder function.
    ///
    /// This is convenient if you prefer chaining calls when composing animations.
    ///
    /// # Arguments
    ///
    /// * `clip_id` - the ID of the clip referenced by this stage
    /// * `builder` - a builder function that takes the new stage as an argument so that you can configure it
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = SpritesheetLibrary::new();
    /// # let mut clip1_id = library.new_clip(|clip| {});
    /// # let mut clip2_id = library.new_clip(|clip| {});
    /// let animation_id = library.new_animation(|animation| {
    ///     animation
    ///         .add_stage(AnimationStage::new_with(clip1_id, |stage| {
    ///            stage.set_repeat(5);
    ///         }))
    ///         .add_stage(AnimationStage::new_with(clip2_id, |stage| {
    ///            stage.set_direction(AnimationDirection::PingPong);
    ///         }));
    /// });
    /// ```
    pub fn new_with<F: Fn(&mut Self)>(clip_id: AnimationClipId, builder: F) -> Self {
        let mut stage = AnimationStage::from_clip(clip_id);

        builder(&mut stage);

        stage
    }

    /// Creates a new [AnimationStage] from an [AnimationClipId].
    ///
    /// The stage will inherit its clip's default parameters.
    /// To override them, use the `set_XXX()` functions.
    ///
    /// # Arguments
    ///
    /// * `clip_id` - the ID of the clip referenced by this stage
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = SpritesheetLibrary::new();
    /// # let clip_id = library.new_clip(|clip| {});
    /// let animation_id = library.new_animation(|animation| {
    ///     let stage = AnimationStage::from_clip(clip_id);
    ///
    ///     animation.add_stage(stage);
    /// });
    /// ```
    pub fn from_clip(clip_id: AnimationClipId) -> Self {
        Self {
            clip_id,
            duration: None,
            repeat: None,
            direction: None,
            easing: None,
        }
    }

    pub fn clip_id(&self) -> &AnimationClipId {
        &self.clip_id
    }

    pub fn set_duration(&mut self, duration: AnimationDuration) -> &mut Self {
        self.duration = Some(duration);
        self
    }

    pub fn duration(&self) -> &Option<AnimationDuration> {
        &self.duration
    }

    pub fn set_repeat(&mut self, repeat: u32) -> &mut Self {
        self.repeat = Some(repeat);
        self
    }

    pub fn repeat(&self) -> &Option<u32> {
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

impl From<AnimationClipId> for AnimationStage {
    fn from(clip_id: AnimationClipId) -> Self {
        AnimationStage::from_clip(clip_id)
    }
}
