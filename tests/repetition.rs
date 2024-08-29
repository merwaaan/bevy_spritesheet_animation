pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn clip_zero() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1]).with_repetitions(0);
    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Times(1));
    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, []);
    }
}

#[test]
fn clip_once() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1]).with_repetitions(1);
    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Times(1));
    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(50);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(animation_id, clip_id, 0),
            ctx.clip_end(animation_id, clip_id),
            ctx.anim_rep_end(animation_id, 0),
            ctx.anim_end(animation_id),
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

    let clip = Clip::from_frames([0, 1]).with_repetitions(10);
    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Times(1));
    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

    // 9 repetitions

    ctx.run(50);
    ctx.check(0, []);

    for i in 0..9 {
        ctx.run(100);
        ctx.check(1, []);

        ctx.run(100);
        ctx.check(0, [ctx.clip_rep_end(animation_id, clip_id, i)]);
    }

    // Last cycle

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(animation_id, clip_id, 9),
            ctx.clip_end(animation_id, clip_id),
            ctx.anim_rep_end(animation_id, 0),
            ctx.anim_end(animation_id),
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

    let zero_clip = Clip::from_frames([3, 2]).with_repetitions(0);
    let zero_clip_id = ctx.library().register_clip(zero_clip);

    let ok_clip = Clip::from_frames([9, 8]);
    let ok_clip_id = ctx.library().register_clip(ok_clip);

    let animation = Animation::from_clips([
        zero_clip_id,
        ok_clip_id,
        zero_clip_id,
        ok_clip_id,
        zero_clip_id,
        zero_clip_id,
        zero_clip_id,
    ])
    .with_duration(AnimationDuration::PerFrame(110))
    .with_repetitions(AnimationRepeat::Times(1));
    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(100);
    ctx.check(9, []);

    ctx.run(100);
    ctx.check(8, []);

    ctx.run(100);
    ctx.check(
        9,
        [
            ctx.clip_rep_end(animation_id, ok_clip_id, 0),
            ctx.clip_end(animation_id, ok_clip_id),
        ],
    );

    ctx.run(100);
    ctx.check(8, []);

    ctx.run(100);
    ctx.check(
        8,
        [
            ctx.clip_rep_end(animation_id, ok_clip_id, 0),
            ctx.clip_end(animation_id, ok_clip_id),
            ctx.anim_rep_end(animation_id, 0),
            ctx.anim_end(animation_id),
        ],
    );
}

#[test]
fn animation_zero() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1]).with_repetitions(1000); // should be ignored
    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id).with_repetitions(AnimationRepeat::Times(0));
    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, []);
    }
}

#[test]
fn animation_once() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1]);
    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Times(1));
    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(50);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(animation_id, clip_id, 0),
            ctx.clip_end(animation_id, clip_id),
            ctx.anim_rep_end(animation_id, 0),
            ctx.anim_end(animation_id),
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

    let clip = Clip::from_frames([0, 1]);
    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Times(10));
    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

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
                ctx.clip_rep_end(animation_id, clip_id, 0),
                ctx.clip_end(animation_id, clip_id),
                ctx.anim_rep_end(animation_id, i),
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
            ctx.clip_rep_end(animation_id, clip_id, 0),
            ctx.clip_end(animation_id, clip_id),
            ctx.anim_rep_end(animation_id, 9),
            ctx.anim_end(animation_id),
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

    let clip = Clip::from_frames([0, 1]);
    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Loop);
    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(50);
    ctx.check(0, []);

    for i in 0..1000 {
        ctx.run(100); // 100 * i + 50
        ctx.check(1, []);

        ctx.run(100); // 100 * i + 150
        ctx.check(
            0,
            [
                ctx.clip_rep_end(animation_id, clip_id, 0),
                ctx.clip_end(animation_id, clip_id),
                ctx.anim_rep_end(animation_id, i),
            ],
        );
    }
}
