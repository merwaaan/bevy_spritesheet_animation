use std::fmt;

use crate::{clip::ClipId, easing::Easing};

/// An opaque identifier that references an [Animation].
///
/// Returned by [AnimationLibrary::register_animation](crate::prelude::AnimationLibrary::register_animation).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct AnimationId {
    pub(crate) value: usize,
}

impl fmt::Display for AnimationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "animation{}", self.value)
    }
}

/// Specifies the duration of an [Animation].
///
/// Defaults to `PerFrame(100)`.
#[derive(Debug, Clone, Copy)]
pub enum AnimationDuration {
    /// Specifies the duration of each frame in milliseconds
    PerFrame(u32),
    /// Specifies the duration of one repetition of the animation in milliseconds
    PerRepetition(u32),
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

/// Specifies the direction of an [Animation].
///
/// Defaults to `AnimationDirection::Forwards`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationDirection {
    /// Frames play from left to right
    Forwards,
    /// Frames play from right to left
    Backwards,
    /// Alternates at each repetition of the animation, starting from left to right
    PingPong,
}

impl Default for AnimationDirection {
    fn default() -> Self {
        Self::Forwards
    }
}

/// A playable animation to assign to a [SpritesheetAnimation](crate::prelude::SpritesheetAnimation) component.
///
/// An animation is composed of one or several [Clip](crate::prelude::Clip)s.
///
/// Parameters like duration, repetitions, direction and easing can be specified.
/// If specified, they will be combined with the parameters of the underlying [Clip](crate::prelude::Clip)s.
///
/// # Example
///
/// ```
/// # use bevy_spritesheet_animation::prelude::*;
/// # let mut library = AnimationLibrary::new();
/// let some_clip = Clip::from_frames([1, 2, 3])
///     .with_duration(AnimationDuration::PerRepetition(2000));
///
/// let some_clip_id = library.register_clip(some_clip);
///
/// let another_clip = Clip::from_frames([7, 8, 9, 7, 7])
///     .with_repetitions(10)
///     .with_direction(AnimationDirection::PingPong);
///
/// let another_clip_id = library.register_clip(another_clip);
///
/// let animation = Animation::from_clips([some_clip_id, another_clip_id])
///     .with_repetitions(AnimationRepeat::Loop)
///     .with_easing(Easing::In(EasingVariety::Quadratic));
///
/// let animation_id = library.register_animation(animation);
/// ```
#[derive(Debug, Clone)]
pub struct Animation {
    /// The IDs of the [Clip](crate::prelude::Clip)s that compose this animation
    clip_ids: Vec<ClipId>,

    /// The optional duration of this animation
    duration: Option<AnimationDuration>,
    /// The optional number of repetitions of this animation
    repetitions: Option<AnimationRepeat>,
    /// The optional direction of this animation
    direction: Option<AnimationDirection>,
    /// The optional easing of this animation
    easing: Option<Easing>,
}

impl Animation {
    /// Creates a new animation from a single clip.
    pub fn from_clip(clip_id: ClipId) -> Self {
        Self {
            clip_ids: vec![clip_id],
            duration: None,
            repetitions: None,
            direction: None,
            easing: None,
        }
    }

    /// Creates a new animation from a sequence of clips.
    pub fn from_clips(clip_ids: impl IntoIterator<Item = ClipId>) -> Self {
        Self {
            clip_ids: clip_ids.into_iter().collect(),
            duration: None,
            repetitions: None,
            direction: None,
            easing: None,
        }
    }

    pub fn clip_ids(&self) -> &[ClipId] {
        &self.clip_ids
    }

    pub fn duration(&self) -> &Option<AnimationDuration> {
        &self.duration
    }

    pub fn with_duration(&self, duration: AnimationDuration) -> Self {
        Self {
            duration: Some(duration),
            ..self.clone()
        }
    }

    pub fn set_duration(&mut self, duration: AnimationDuration) -> &mut Self {
        self.duration = Some(duration);
        self
    }

    pub fn repetitions(&self) -> &Option<AnimationRepeat> {
        &self.repetitions
    }

    pub fn with_repetitions(&self, repetitions: AnimationRepeat) -> Self {
        Self {
            repetitions: Some(repetitions),
            ..self.clone()
        }
    }

    pub fn set_repetitions(&mut self, repetitions: AnimationRepeat) -> &mut Self {
        self.repetitions = Some(repetitions);
        self
    }

    pub fn direction(&self) -> &Option<AnimationDirection> {
        &self.direction
    }

    pub fn with_direction(&self, direction: AnimationDirection) -> Self {
        Self {
            direction: Some(direction),
            ..self.clone()
        }
    }

    pub fn set_direction(&mut self, direction: AnimationDirection) -> &mut Self {
        self.direction = Some(direction);
        self
    }

    pub fn easing(&self) -> &Option<Easing> {
        &self.easing
    }

    pub fn with_easing(&self, easing: Easing) -> Self {
        Self {
            easing: Some(easing),
            ..self.clone()
        }
    }

    pub fn set_easing(&mut self, easing: Easing) -> &mut Self {
        self.easing = Some(easing);
        self
    }
}
