pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn clip_zero() {
    let mut ctx = Context::new();

    ctx.attach_animation(|builder| {
        builder
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Times(1))
            .add_indices([0, 1])
            .set_clip_repetitions(0)
    });

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, []);
    }
}

#[test]
fn clip_once() {
    let mut ctx = Context::new();

    let mut clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Times(1))
            .add_indices([0, 1])
            .set_clip_repetitions(1)
            .get_current_clip_id(&mut clip_id)
    });

    ctx.run(50);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(&animation, clip_id, 0),
            ctx.clip_end(&animation, clip_id),
            ctx.anim_rep_end(&animation, 0),
            ctx.anim_end(&animation),
        ],
    );

    // Over

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(1, []);
    }
}

#[test]
fn clip_many() {
    let mut ctx = Context::new();

    let mut clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Times(1))
            .add_indices([0, 1])
            .set_clip_repetitions(10)
            .get_current_clip_id(&mut clip_id)
    });

    // 9 repetitions

    ctx.run(50);
    ctx.check(0, []);

    for i in 0..9 {
        ctx.run(100);
        ctx.check(1, []);

        ctx.run(100);
        ctx.check(0, [ctx.clip_rep_end(&animation, clip_id, i)]);
    }

    // Last cycle

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(&animation, clip_id, 9),
            ctx.clip_end(&animation, clip_id),
            ctx.anim_rep_end(&animation, 0),
            ctx.anim_end(&animation),
        ],
    );

    // Over

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(1, []);
    }
}

#[test]
fn some_clips_repeated_zero_times() {
    let mut ctx = Context::new();

    let mut zero_clip_id = ClipId::dummy();
    let mut ok_clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .set_duration(AnimationDuration::PerFrame(110))
            .set_repetitions(AnimationRepeat::Times(1))
            // Clip 1: zero repetitions
            .add_indices([3, 2])
            .set_clip_repetitions(0)
            .get_current_clip_id(&mut zero_clip_id)
            // Clip 2
            .start_clip()
            .add_indices([9, 8])
            .get_current_clip_id(&mut ok_clip_id)
            // Other clips: copies
            .copy_clip(zero_clip_id)
            .copy_clip(ok_clip_id)
            .copy_clip(zero_clip_id)
            .copy_clip(zero_clip_id)
            .copy_clip(zero_clip_id)
    });

    ctx.run(100);
    ctx.check(9, []);

    ctx.run(100);
    ctx.check(8, []);

    ctx.run(100);
    ctx.check(
        9,
        [
            ctx.clip_rep_end(&animation, ok_clip_id, 0),
            ctx.clip_end(&animation, ok_clip_id),
        ],
    );

    ctx.run(100);
    ctx.check(8, []);

    ctx.run(100);
    ctx.check(
        8,
        [
            ctx.clip_rep_end(&animation, ok_clip_id, 0),
            ctx.clip_end(&animation, ok_clip_id),
            ctx.anim_rep_end(&animation, 0),
            ctx.anim_end(&animation),
        ],
    );
}

#[test]
fn animation_zero() {
    let mut ctx = Context::new();

    ctx.attach_animation(|builder| {
        builder
            .set_repetitions(AnimationRepeat::Times(0))
            .add_indices([0, 1])
            .set_clip_repetitions(1000)
    });

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, []);
    }
}

#[test]
fn animation_once() {
    let mut ctx = Context::new();

    let mut clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Times(1))
            .add_indices([0, 1])
            .get_current_clip_id(&mut clip_id)
    });

    ctx.run(50);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(&animation, clip_id, 0),
            ctx.clip_end(&animation, clip_id),
            ctx.anim_rep_end(&animation, 0),
            ctx.anim_end(&animation),
        ],
    );

    // Over

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(1, []);
    }
}

#[test]
fn animation_many() {
    let mut ctx = Context::new();

    let mut clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Times(10))
            .add_indices([0, 1])
            .get_current_clip_id(&mut clip_id)
    });

    // 9 repetitions

    ctx.run(50);
    ctx.check(0, []);

    for i in 0..9 {
        ctx.run(100);
        ctx.check(1, []);

        ctx.run(100);
        ctx.check(
            0,
            [
                ctx.clip_rep_end(&animation, clip_id, 0),
                ctx.clip_end(&animation, clip_id),
                ctx.anim_rep_end(&animation, i),
            ],
        );
    }

    // Last cycle

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(&animation, clip_id, 0),
            ctx.clip_end(&animation, clip_id),
            ctx.anim_rep_end(&animation, 9),
            ctx.anim_end(&animation),
        ],
    );

    // Over

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(1, []);
    }
}

#[test]
fn animation_forever() {
    let mut ctx = Context::new();

    let mut clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Loop)
            .add_indices([0, 1])
            .get_current_clip_id(&mut clip_id)
    });

    ctx.run(50);
    ctx.check(0, []);

    for i in 0..1000 {
        ctx.run(100); // 100 * i + 50
        ctx.check(1, []);

        ctx.run(100); // 100 * i + 150
        ctx.check(
            0,
            [
                ctx.clip_rep_end(&animation, clip_id, 0),
                ctx.clip_end(&animation, clip_id),
                ctx.anim_rep_end(&animation, i),
            ],
        );
    }
}
