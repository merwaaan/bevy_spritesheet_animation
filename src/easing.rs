use std::f32::consts::PI;

/// Variety to associate with [Easing]s to tune the acceleration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EasingVariety {
    Quadratic,
    Cubic,
    Quartic,
    Quintic,
    Exponential,
    Circular,
    Sin,
}

/// Specifies the easing of an animation.
///
/// Defaults to [Easing::Linear].
///
/// # Example
///
/// ```
/// # use bevy_spritesheet_animation::prelude::*;
/// # let mut library = AnimationLibrary::new();
/// // Apply easing on a clip
///
/// let clip = Clip::from_frames([7, 8, 9])
///     .with_easing(Easing::In(EasingVariety::Quadratic));
///
/// let clip_id = library.register_clip(clip);
///
/// // Apply easing on the whole animation
/// // (will combine with the clip's own easing)
///
/// let animation = Animation::from_clip(clip_id)
///     .with_easing(Easing::InOut(EasingVariety::Cubic));
/// ```
///
/// # References
///
/// - <https://easings.net/>
/// - <http://robertpenner.com/easing/penner_chapter7_tweening.pdf>
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Easing {
    /// Linear interpolation
    #[default]
    Linear,
    /// Slow at the start of the animation then speeds up
    In(EasingVariety),
    /// Fast at the start of the animation then slows down
    Out(EasingVariety),
    /// Fast at the start and at the end of the animation, slows down in the middle
    InOut(EasingVariety),
}

impl Easing {
    /// Applies the easing function on `x`.
    ///
    /// Expects `x` to be be in the [0, 1] range.
    ///
    /// The returned value will be in the [0, 1] range.
    pub fn get(&self, x: f32) -> f32 {
        let x = x.clamp(0.0, 1.0);

        match *self {
            Easing::Linear => x,
            Easing::In(variety) => match variety {
                EasingVariety::Quadratic => x.powi(2),
                EasingVariety::Cubic => x.powi(3),
                EasingVariety::Quartic => x.powi(4),
                EasingVariety::Quintic => x.powi(5),
                EasingVariety::Exponential => {
                    if x == 0.0 {
                        0.0
                    } else {
                        2.0f32.powf(10.0 * x - 10.0)
                    }
                }
                EasingVariety::Circular => 1.0 - (1.0 - x.powi(2)).sqrt(),
                EasingVariety::Sin => 1.0 - ((x * PI) / 2.0).cos(),
            },
            Easing::Out(variety) => match variety {
                EasingVariety::Quadratic => 1.0 - (1.0 - x).powi(2),
                EasingVariety::Cubic => 1.0 - (1.0 - x).powi(3),
                EasingVariety::Quartic => 1.0 - (1.0 - x).powi(4),
                EasingVariety::Quintic => 1.0 - (1.0 - x).powi(5),
                EasingVariety::Exponential => {
                    if x == 1.0 {
                        1.0
                    } else {
                        1.0 - 2.0f32.powf(-10.0 * x)
                    }
                }
                EasingVariety::Circular => (1.0 - (x - 1.0).powi(2)).sqrt(),
                EasingVariety::Sin => ((x * PI) / 2.0).sin(),
            },
            Easing::InOut(variety) => match variety {
                EasingVariety::Quadratic => {
                    if x < 0.5 {
                        2.0 * x.powi(2)
                    } else {
                        1.0 - (-2.0 * x + 2.0).powi(2) / 2.0
                    }
                }
                EasingVariety::Cubic => {
                    if x < 0.5 {
                        4.0 * x.powi(3)
                    } else {
                        1.0 - (-2.0 * x + 2.0).powi(3) / 2.0
                    }
                }
                EasingVariety::Quartic => {
                    if x < 0.5 {
                        8.0 * x.powi(4)
                    } else {
                        1.0 - (-2.0 * x + 2.0).powi(4) / 2.0
                    }
                }
                EasingVariety::Quintic => {
                    if x < 0.5 {
                        16.0 * x.powi(5)
                    } else {
                        1.0 - (-2.0 * x + 2.0).powi(5) / 2.0
                    }
                }
                EasingVariety::Exponential => {
                    if x == 0.0 {
                        0.0
                    } else if x == 1.0 {
                        1.0
                    } else if x < 0.5 {
                        2.0f32.powf(20.0 * x - 10.0) / 2.0
                    } else {
                        (2.0 - 2.0f32.powf(-20.0 * x + 10.0)) / 2.0
                    }
                }
                EasingVariety::Circular => {
                    if x < 0.5 {
                        (1.0 - (1.0 - (2.0 * x).powi(2)).sqrt()) / 2.0
                    } else {
                        ((1.0 - (-2.0 * x + 2.0).powi(2)).sqrt() + 1.0) / 2.0
                    }
                }
                EasingVariety::Sin => -(((x * PI).cos() - 1.0) / 2.0),
            },
        }
    }
}
