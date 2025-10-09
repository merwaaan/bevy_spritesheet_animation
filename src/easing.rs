use std::f32::consts::PI;

use bevy::prelude::*;

/// Variety to associate with [Easings](Easing) to tune the acceleration.
///
/// # References
///
/// - <https://easings.net/>
/// - <http://robertpenner.com/easing/penner_chapter7_tweening.pdf>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub enum EasingVariety {
    Quadratic,
    Cubic,
    Quartic,
    Quintic,
    Exponential,
    Circular,
    Sin,
}

/// Animation easing.
///
/// # Example
///
/// ```
/// # use bevy_spritesheet_animation::prelude::*;
/// # fn f(spritesheet: &Spritesheet) {
/// let animation = spritesheet
///     .create_animation()
///     // Clip 1 (no easing, defaults to Linear)
///     .add_row(0)
///     // Clip 2
///     .start_clip()
///     .add_row(3)
///     .set_clip_easing(Easing::In(EasingVariety::Quadratic))
///     // Apply easing on the whole animation (will combine with the clips' own easings)
///     .set_easing(Easing::InOut(EasingVariety::Cubic))
///     .build();
/// # }
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Default, PartialEq, Hash)]
pub enum Easing {
    /// Linear interpolation.
    #[default]
    Linear,
    /// Slow at the start of the animation then speeds up.
    In(EasingVariety),
    /// Fast at the start of the animation then slows down.
    Out(EasingVariety),
    /// Fast at the start and at the end of the animation, slows down in the middle.
    InOut(EasingVariety),
}

impl Easing {
    /// Remaps a value (usually the animation's progress) with the easing function.
    ///
    /// The returned value will be in the [0, 1] range.
    ///
    /// # Arguments
    ///
    /// - `x`: the value in the [0, 1] range to remap.
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

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    fn check(easing: Easing, cases: Vec<(f32, f32)>) {
        for case in cases {
            assert_relative_eq!(easing.get(case.0), case.1, epsilon = 0.00001);
        }
    }

    #[test]
    fn linear() {
        check(
            Easing::Linear,
            vec![
                (-1000.0, 0.0),
                (0.0, 0.0),
                (0.15, 0.15),
                (0.38, 0.38),
                (0.72, 0.72),
                (0.99, 0.99),
                (1.0, 1.0),
                (123.0, 1.0),
            ],
        );
    }

    // In

    #[test]
    fn in_quadratic() {
        check(
            Easing::In(EasingVariety::Quadratic),
            vec![
                (-123.5, 0.0),
                (0.0, 0.0),
                (0.12, 0.0144),
                (0.31, 0.0961),
                (0.5, 0.25),
                (0.78, 0.6084),
                (1.0, 1.0),
                (9999.0, 1.0),
            ],
        );
    }

    #[test]
    fn in_cubic() {
        check(
            Easing::In(EasingVariety::Cubic),
            vec![
                (-5670.0, 0.0),
                (0.0, 0.0),
                (0.12, 0.00172),
                (0.45, 0.09112),
                (0.88, 0.68147),
                (0.99, 0.97029),
                (1.0, 1.0),
                (1.2, 1.0),
            ],
        )
    }

    #[test]
    fn in_quartic() {
        check(
            Easing::In(EasingVariety::Quartic),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.11, 0.00014641),
                (0.36, 0.01679615),
                (0.52, 0.07311616),
                (0.91, 0.685_749_6),
                (1.87, 1.0),
            ],
        )
    }

    #[test]
    fn in_quintic() {
        check(
            Easing::In(EasingVariety::Quintic),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.16, 0.00010),
                (0.29, 0.00205),
                (0.5, 0.03125),
                (0.81, 0.34867),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }

    #[test]
    fn in_expo() {
        check(
            Easing::In(EasingVariety::Exponential),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.05, 0.001381),
                (0.37, 0.012691),
                (0.62, 0.071793),
                (0.88, 0.435275),
                (0.93, 0.615572),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }

    #[test]
    fn in_circular() {
        check(
            Easing::In(EasingVariety::Circular),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.1, 0.005012),
                (0.15, 0.011314),
                (0.48, 0.122731),
                (0.79, 0.386893),
                (0.98, 0.801002),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }

    #[test]
    fn in_sin() {
        check(
            Easing::In(EasingVariety::Sin),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.04, 0.001973),
                (0.37, 0.164192),
                (0.52, 0.315452),
                (0.62, 0.437916),
                (0.97, 0.952893),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }

    // Out

    #[test]
    fn out_quadratic() {
        check(
            Easing::Out(EasingVariety::Quadratic),
            vec![
                (-123.5, 0.0),
                (0.0, 0.0),
                (0.06, 0.1164),
                (0.36, 0.5904),
                (0.49, 0.7399),
                (0.68, 0.8976),
                (0.8, 0.96),
                (0.94, 0.9964),
                (1.0, 1.0),
                (9999.0, 1.0),
            ],
        );
    }

    #[test]
    fn out_cubic() {
        check(
            Easing::Out(EasingVariety::Cubic),
            vec![
                (-5670.0, 0.0),
                (0.0, 0.0),
                (0.12, 0.31853),
                (0.27, 0.61098),
                (0.48, 0.85939),
                (0.57, 0.92049),
                (0.87, 0.99780),
                (0.98, 0.99999),
                (1.0, 1.0),
                (1.2, 1.0),
            ],
        )
    }

    #[test]
    fn out_quartic() {
        check(
            Easing::Out(EasingVariety::Quartic),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.07, 0.25195),
                (0.19, 0.56953),
                (0.31, 0.77333),
                (0.52, 0.94692),
                (0.68, 0.98951),
                (0.87, 0.99971),
                (0.9, 0.99990),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }

    #[test]
    fn out_quintic() {
        check(
            Easing::Out(EasingVariety::Quintic),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.19, 0.65132),
                (0.35, 0.88397),
                (0.52, 0.97452),
                (0.71, 0.99795),
                (0.88, 0.99998),
                (0.98, 1.00000),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }

    #[test]
    fn out_expo() {
        check(
            Easing::Out(EasingVariety::Exponential),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.02, 0.12945),
                (0.15, 0.64645),
                (0.33, 0.89847),
                (0.54, 0.97632),
                (0.73, 0.99365),
                (0.95, 0.99862),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }

    #[test]
    fn out_circular() {
        check(
            Easing::Out(EasingVariety::Circular),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.06, 0.34117),
                (0.16, 0.54259),
                (0.39, 0.79240),
                (0.53, 0.88267),
                (0.74, 0.96561),
                (0.92, 0.99679),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }

    #[test]
    fn out_sin() {
        check(
            Easing::Out(EasingVariety::Sin),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.02, 0.03141),
                (0.27, 0.41151),
                (0.49, 0.69591),
                (0.62, 0.82708),
                (0.83, 0.96456),
                (0.93, 0.99396),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }

    // InOut

    #[test]
    fn in_out_quadratic() {
        check(
            Easing::InOut(EasingVariety::Quadratic),
            vec![
                (-123.5, 0.0),
                (0.0, 0.0),
                (0.05, 0.00500),
                (0.17, 0.05780),
                (0.31, 0.19220),
                (0.46, 0.42320),
                (0.62, 0.71120),
                (0.81, 0.92780),
                (0.97, 0.99820),
                (1.0, 1.0),
                (9999.0, 1.0),
            ],
        );
    }

    #[test]
    fn in_out_cubic() {
        check(
            Easing::InOut(EasingVariety::Cubic),
            vec![
                (-5670.0, 0.0),
                (0.0, 0.0),
                (0.01, 0.00000),
                (0.07, 0.00137),
                (0.26, 0.07030),
                (0.47, 0.41529),
                (0.59, 0.72432),
                (0.71, 0.90244),
                (0.81, 0.97256),
                (0.99, 1.00000),
                (1.0, 1.0),
                (1.2, 1.0),
            ],
        )
    }

    #[test]
    fn in_out_quartic() {
        check(
            Easing::InOut(EasingVariety::Quartic),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.02, 0.00000),
                (0.21, 0.01556),
                (0.32, 0.08389),
                (0.49, 0.46118),
                (0.59, 0.77394),
                (0.78, 0.98126),
                (0.91, 0.99948),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }

    #[test]
    fn in_out_quintic() {
        check(
            Easing::InOut(EasingVariety::Quintic),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.02, 0.00000),
                (0.07, 0.00003),
                (0.21, 0.00653),
                (0.48, 0.40769),
                (0.59, 0.81463),
                (0.61, 0.85564),
                (0.79, 0.99347),
                (0.99, 1.00000),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }

    #[test]
    fn in_out_expo() {
        check(
            Easing::InOut(EasingVariety::Exponential),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.07, 0.00129),
                (0.19, 0.00680),
                (0.26, 0.01795),
                (0.43, 0.18946),
                (0.62, 0.90527),
                (0.88, 0.99742),
                (0.96, 0.99915),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }

    #[test]
    fn in_out_circular() {
        check(
            Easing::InOut(EasingVariety::Circular),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.01, 0.00010),
                (0.22, 0.05100),
                (0.35, 0.14293),
                (0.52, 0.64000),
                (0.62, 0.82496),
                (0.89, 0.98775),
                (0.92, 0.99356),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }

    #[test]
    fn in_out_sin() {
        check(
            Easing::InOut(EasingVariety::Sin),
            vec![
                (-99999.0, 0.0),
                (0.0, 0.0),
                (0.05, 0.00616),
                (0.21, 0.10492),
                (0.43, 0.39093),
                (0.54, 0.56267),
                (0.69, 0.78104),
                (0.81, 0.91354),
                (0.99, 0.99975),
                (1.0, 1.0),
                (1.87, 1.0),
            ],
        )
    }
}
