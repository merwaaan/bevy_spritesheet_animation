use bevy::prelude::*;
use rand::Rng;

const DEFAULT_WINDOW_WIDTH: f32 = 1280.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 720.0;

/// Returns the screen-space position of the nth item in a grid
pub fn grid_position(columns: u32, rows: u32, n: usize) -> Vec3 {
    const MARGIN: f32 = 100.0;

    let width = DEFAULT_WINDOW_WIDTH - MARGIN * 2.0;
    let height = DEFAULT_WINDOW_HEIGHT - MARGIN * 2.0;

    let xgap = width / columns.saturating_sub(1) as f32;
    let ygap = height / rows.saturating_sub(1) as f32;

    let x = (n as u32 % columns) as f32;
    let y = (n as u32 / columns) as f32;

    Vec3::new(
        x * xgap - width / 2.0,
        -y * ygap + height / 2.0, // flip Y
        0.0,
    )
}

/// Returns a random on-screen position
pub fn random_position() -> Vec3 {
    let mut rng = rand::thread_rng();

    Vec3::new(
        rng.gen_range(-DEFAULT_WINDOW_WIDTH / 2.0..DEFAULT_WINDOW_WIDTH / 2.0),
        rng.gen_range(-DEFAULT_WINDOW_WIDTH / 2.0..DEFAULT_WINDOW_HEIGHT / 2.0),
        0.0,
    )
}
