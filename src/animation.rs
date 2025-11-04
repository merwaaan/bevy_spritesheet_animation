use bevy::prelude::*;

use crate::{clip::Clip, easing::Easing};

/// The duration of an [Animation].
#[derive(Debug, Clone, Copy, Reflect)]
#[reflect(Debug)]
pub enum AnimationDuration {
    /// Specifies the duration of one frame in milliseconds (default = `PerFrame(100)`).
    PerFrame(u32),
    /// Specifies the duration of one repetition of the animation in milliseconds.
    PerRepetition(u32),
}

impl Default for AnimationDuration {
    fn default() -> Self {
        Self::PerFrame(100)
    }
}

/// How many times an [Animation] repeats.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub enum AnimationRepeat {
    /// Loops forever (default).
    #[default]
    Loop,
    /// Repeats n times.
    Times(usize),
}

/// The direction in which the frames of an [Animation] are played.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub enum AnimationDirection {
    /// Frames play from left to right (default).
    #[default]
    Forwards,
    /// Frames play from right to left.
    Backwards,
    /// Alternates at each repetition of the animation, starting from left to right.
    PingPong,
}

/// A playable animation to assign to a [SpritesheetAnimation](crate::prelude::SpritesheetAnimation) component.
///
/// Use [Spritesheet::create_animation()](crate::prelude::Spritesheet::create_animation) to build new animations.
///
/// An animation is composed of one or several [Clips](crate::prelude::Clip).
/// - For simple animation, you can directly add frames to the default clip (it doesn't need to be created explicitly).
/// - For more sophisticated animations, you can create new clips with [start_clip()](crate::prelude::AnimationBuilder::start_clip) and [copy_clip()](crate::prelude::AnimationBuilder::copy_clip).
///
/// # Parameters
///
/// Playback parameters like [duration](crate::prelude::AnimationDuration), [repetitions](crate::prelude::AnimationRepeat), [direction](crate::prelude::AnimationDirection) and [easing](crate::prelude::Easing) can be specified.
///
/// Those animation-level parameters will be combined with the parameters of the underlying [Clips](crate::prelude::Clip).
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// # fn f(assets: &AssetServer) {
/// let image = assets.load("character.png");
///
/// let spritesheet = Spritesheet::new(&image, 8, 4);
///
/// let animation = spritesheet
///     .create_animation()
///     // Global animation parameters
///     .set_repetitions(AnimationRepeat::Loop)
///     .set_easing(Easing::In(EasingVariety::Quadratic))
///     // Clip 1 (default clip, doesn't need to be created explicitly)
///     .add_row(3)
///     .set_clip_duration(AnimationDuration::PerRepetition(2000))
///     // Clip 2
///     .start_clip()
///     .add_row(5)
///     .set_clip_repetitions(10)
///     .set_clip_direction(AnimationDirection::PingPong)
///     // Get the final animation
///     .build();
/// # }
/// ```
#[derive(Asset, Debug, Clone, Reflect)]
#[reflect(Debug)]
pub struct Animation {
    pub(crate) clips: Vec<Clip>,

    pub(crate) duration: Option<AnimationDuration>,
    pub(crate) repetitions: Option<AnimationRepeat>,
    pub(crate) direction: Option<AnimationDirection>,
    pub(crate) easing: Option<Easing>,
}

impl Animation {
    /// Private empty animation constructor.
    ///
    /// Users should not create empty animations.
    /// Our animation builder starts with such an empty animation though.
    pub(crate) fn empty() -> Self {
        Self {
            clips: vec![Clip::empty()],
            duration: None,
            repetitions: None,
            direction: None,
            easing: None,
        }
    }

    /// The [Clips](crate::prelude::Clip) that compose this animation
    pub fn clips(&self) -> &[Clip] {
        &self.clips
    }

    /// The optional duration of this animation
    pub fn duration(&self) -> &Option<AnimationDuration> {
        &self.duration
    }

    /// The optional number of repetitions of this animation
    pub fn repetitions(&self) -> &Option<AnimationRepeat> {
        &self.repetitions
    }

    /// The optional direction of this animation
    pub fn direction(&self) -> &Option<AnimationDirection> {
        &self.direction
    }

    /// The optional easing of this animation
    pub fn easing(&self) -> &Option<Easing> {
        &self.easing
    }
}
