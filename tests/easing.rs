use approx::assert_relative_eq;
use bevy_spritesheet_animation::prelude::*;

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

// EaseIn

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
            (0.91, 0.68574961),
            (1.87, 1.0),
        ],
    )
}

// #[test]
// fn in_quintic() {
//     check(
//         Easing::EaseIn(EasingVariety::Quintic),
//         vec![
//             (-99999.0, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (1.87, 1.0),
//         ],
//     )
// }

// #[test]
// fn in_expo() {
//     check(
//         Easing::EaseIn(EasingVariety::Expo),
//         vec![
//             (-99999.0, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (1.87, 1.0),
//         ],
//     )
// }

// #[test]
// fn in_circular() {
//     check(
//         Easing::EaseIn(EasingVariety::Circular),
//         vec![
//             (-99999.0, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (1.87, 1.0),
//         ],
//     )
// }

// // EaseOut

// #[test]
// fn out_quadratic() {
//     check(
//         Easing::EaseOut(EasingVariety::Quadratic),
//         vec![
//             (-123.5, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (9999.0, 1.0),
//         ],
//     );
// }

// #[test]
// fn out_cubic() {
//     check(
//         Easing::EaseOut(EasingVariety::Cubic),
//         vec![
//             (-5670.0, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (1.2, 1.0),
//         ],
//     )
// }

// #[test]
// fn out_quartic() {
//     check(
//         Easing::EaseOut(EasingVariety::Quartic),
//         vec![
//             (-99999.0, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (1.87, 1.0),
//         ],
//     )
// }

// #[test]
// fn out_quintic() {
//     check(
//         Easing::EaseOut(EasingVariety::Quintic),
//         vec![
//             (-99999.0, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (1.87, 1.0),
//         ],
//     )
// }

// #[test]
// fn out_expo() {
//     check(
//         Easing::EaseOut(EasingVariety::Expo),
//         vec![
//             (-99999.0, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (1.87, 1.0),
//         ],
//     )
// }

// #[test]
// fn out_circular() {
//     check(
//         Easing::EaseOut(EasingVariety::Circular),
//         vec![
//             (-99999.0, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (1.87, 1.0),
//         ],
//     )
// }

// // EaseInOut

// #[test]
// fn in_out_quadratic() {
//     check(
//         Easing::EaseOut(EasingVariety::Quadratic),
//         vec![
//             (-123.5, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (9999.0, 1.0),
//         ],
//     );
// }

// #[test]
// fn in_out_cubic() {
//     check(
//         Easing::EaseOut(EasingVariety::Cubic),
//         vec![
//             (-5670.0, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (1.2, 1.0),
//         ],
//     )
// }

// #[test]
// fn in_out_quartic() {
//     check(
//         Easing::EaseOut(EasingVariety::Quartic),
//         vec![
//             (-99999.0, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (1.87, 1.0),
//         ],
//     )
// }

// #[test]
// fn in_out_quintic() {
//     check(
//         Easing::EaseOut(EasingVariety::Quintic),
//         vec![
//             (-99999.0, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (1.87, 1.0),
//         ],
//     )
// }

// #[test]
// fn in_out_expo() {
//     check(
//         Easing::EaseOut(EasingVariety::Expo),
//         vec![
//             (-99999.0, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (1.87, 1.0),
//         ],
//     )
// }

// #[test]
// fn in_out_circular() {
//     check(
//         Easing::EaseOut(EasingVariety::Circular),
//         vec![
//             (-99999.0, 0.0),
//             (0.0, 0.0),
//             (_, _),
//             (_, _),
//             (_, _),
//             (_, _),
//             (1.0, 1.0),
//             (1.87, 1.0),
//         ],
//     )
// }
