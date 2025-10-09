pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn manual_control() {
    let mut ctx = Context::new();

    ctx.attach_animation(|builder| {
        builder
            .add_indices([4, 5, 6])
            .set_clip_duration(AnimationDuration::PerFrame(1000))
    });

    ctx.run(800);
    ctx.check(4, []);

    ctx.run(400); // 1200, switched to the next frame
    ctx.check(5, []);

    ctx.get_sprite(|sprite| {
        sprite.progress.frame = 0;
    });

    ctx.run(200); // 1400 but ~200
    ctx.check(4, []);

    ctx.run(700); // ~900, still the same
    ctx.check(4, []);

    ctx.run(300); // ~1200, switched to the next frame
    ctx.check(5, []);

    ctx.get_sprite(|sprite| {
        sprite.progress.frame = 2;
    });

    ctx.run(100); // ~1300
    ctx.check(6, []);
}

#[test]
fn manual_control_while_paused() {
    let mut ctx = Context::new();

    ctx.attach_animation(|builder| {
        builder
            .add_indices([4, 5, 6])
            .set_clip_duration(AnimationDuration::PerFrame(1000))
    });

    ctx.run(500);
    ctx.check(4, []);

    // Pause

    ctx.get_sprite(|sprite| {
        sprite.playing = false;
    });

    // No changes

    ctx.run(1000);
    ctx.check(4, []);

    // Manual change

    ctx.get_sprite(|sprite| {
        sprite.progress.frame = 1;
    });

    ctx.run(2000);
    ctx.check(5, []);

    // Manual change

    ctx.get_sprite(|sprite| {
        sprite.progress.frame = 0;
    });

    ctx.run(1);
    ctx.check(4, []);

    // Manual change

    ctx.get_sprite(|sprite| {
        sprite.progress.frame = 2;
    });

    ctx.run(100);
    ctx.check(6, []);
}

#[test]
fn manual_control_startup() {
    let mut ctx = Context::new();

    ctx.attach_animation(|builder| {
        builder
            .add_indices([4, 5, 6])
            .set_clip_duration(AnimationDuration::PerFrame(1000))
    });

    ctx.get_sprite(|sprite| {
        sprite.progress.frame = 1;
        sprite.progress.repetition = 12;
    });

    ctx.run(500);
    ctx.get_sprite(|sprite| {
        assert_eq!(sprite.progress.frame, 1);
        assert_eq!(sprite.progress.repetition, 12);
    });
}

#[test]
fn manual_control_startup_paused() {
    let mut ctx = Context::new();

    ctx.attach_animation(|builder| {
        builder
            .add_indices([4, 5, 6])
            .set_clip_duration(AnimationDuration::PerFrame(1000))
    });

    ctx.get_sprite(|sprite| {
        sprite.progress.frame = 1;
        sprite.progress.repetition = 12;
        sprite.playing = false;
    });

    ctx.run(1500);
    ctx.get_sprite(|sprite| {
        assert_eq!(sprite.progress.frame, 1);
        assert_eq!(sprite.progress.repetition, 12);
    });
}

#[test]
fn manual_control_invalid_frame() {
    let mut ctx = Context::new();

    ctx.attach_animation(|builder| {
        builder
            .add_indices([4, 5, 6])
            .set_clip_duration(AnimationDuration::PerFrame(1000))
    });

    // From startup

    ctx.get_sprite(|sprite| {
        sprite.progress.frame = 100;
        sprite.progress.repetition = 100;
    });

    ctx.run(500);
    ctx.get_sprite(|sprite| {
        assert_eq!(sprite.progress.frame, 0);
        assert_eq!(sprite.progress.repetition, 0);
    });

    // While playing

    ctx.get_sprite(|sprite| {
        sprite.progress.frame = 100;
        sprite.progress.repetition = 100;
    });

    ctx.run(1500);
    ctx.get_sprite(|sprite| {
        assert_eq!(sprite.progress.frame, 1);
        assert_eq!(sprite.progress.repetition, 0);
    });
}

#[test]
fn manual_control_invalid_repetition() {
    let mut ctx = Context::new();

    ctx.attach_animation(|builder| {
        builder
            .set_repetitions(AnimationRepeat::Times(3))
            .add_indices([4, 5, 6])
            .set_clip_duration(AnimationDuration::PerFrame(1000))
    });

    // From startup

    ctx.get_sprite(|sprite| {
        sprite.progress.frame = 1;
        sprite.progress.repetition = 100;
    });

    ctx.run(500);
    ctx.get_sprite(|sprite| {
        assert_eq!(sprite.progress.frame, 0);
        assert_eq!(sprite.progress.repetition, 0);
    });

    // While playing

    ctx.get_sprite(|sprite| {
        sprite.progress.frame = 1;
        sprite.progress.repetition = 100;
    });

    ctx.run(1500);
    ctx.get_sprite(|sprite| {
        assert_eq!(sprite.progress.frame, 1);
        assert_eq!(sprite.progress.repetition, 0);
    });
}
