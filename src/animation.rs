use bevy::{asset::Asset, reflect::prelude::*};

use crate::{clip::Clip, easing::Easing};

/// Specifies the duration of an [Animation].
///
/// Defaults to `PerFrame(100)`.
#[derive(Debug, Clone, Copy, Reflect)]
#[reflect(Debug)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub enum AnimationRepeat {
    /// Loops indefinitely
    Loop,
    /// Repeats a fixed number of times
    Times(usize),
}

impl Default for AnimationRepeat {
    fn default() -> Self {
        Self::Loop
    }
}

/// Specifies the direction of an [Animation].
///
/// Defaults to `AnimationDirection::Forwards`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
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
/// let some_clip = Clip::from_frames([1, 2, 3])
///     .with_duration(AnimationDuration::PerRepetition(2000));
///
/// let another_clip = Clip::from_frames([7, 8, 9, 7, 7])
///     .with_repetitions(10)
///     .with_direction(AnimationDirection::PingPong);
///
/// let animation = Animation::from_clips([some_clip, another_clip])
///     .with_repetitions(AnimationRepeat::Loop)
///     .with_easing(Easing::In(EasingVariety::Quadratic));
/// ```
#[derive(Asset, TypePath, Debug, Clone)]
//TODOmerwan#[reflect(Debug)]
pub struct Animation {
    /// The [Clip](crate::prelude::Clip)s that compose this animation
    clips: Vec<Clip>,

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
    pub fn from_clip(clip: Clip) -> Self {
        Self {
            clips: vec![clip],
            duration: None,
            repetitions: None,
            direction: None,
            easing: None,
        }
    }

    /// Creates a new animation from a sequence of clips.
    pub fn from_clips(clips: impl IntoIterator<Item = Clip>) -> Self {
        Self {
            clips: clips.into_iter().collect(),
            duration: None,
            repetitions: None,
            direction: None,
            easing: None,
        }
    }

    pub fn clips(&self) -> &[Clip] {
        &self.clips
    }

    pub fn duration(&self) -> &Option<AnimationDuration> {
        &self.duration
    }

    pub fn with_duration(mut self, duration: AnimationDuration) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn set_duration(&mut self, duration: AnimationDuration) -> &mut Self {
        self.duration = Some(duration);
        self
    }

    pub fn repetitions(&self) -> &Option<AnimationRepeat> {
        &self.repetitions
    }

    pub fn with_repetitions(mut self, repetitions: AnimationRepeat) -> Self {
        self.repetitions = Some(repetitions);
        self
    }

    pub fn set_repetitions(&mut self, repetitions: AnimationRepeat) -> &mut Self {
        self.repetitions = Some(repetitions);
        self
    }

    pub fn direction(&self) -> &Option<AnimationDirection> {
        &self.direction
    }

    pub fn with_direction(mut self, direction: AnimationDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn set_direction(&mut self, direction: AnimationDirection) -> &mut Self {
        self.direction = Some(direction);
        self
    }

    pub fn easing(&self) -> &Option<Easing> {
        &self.easing
    }

    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = Some(easing);
        self
    }

    pub fn set_easing(&mut self, easing: Easing) -> &mut Self {
        self.easing = Some(easing);
        self
    }
}
