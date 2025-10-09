pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn clip_zero() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1]).with_repetitions(0);

    ctx.attach_animation(
        Animation::from_clip(clip)
            .with_duration(AnimationDuration::PerFrame(100))
            .with_repetitions(AnimationRepeat::Times(1)),
    );

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, []);
    }
}

#[test]
fn clip_once() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1]).with_repetitions(1);

    let animation = ctx.attach_animation(
        Animation::from_clip(clip.clone())
            .with_duration(AnimationDuration::PerFrame(100))
            .with_repetitions(AnimationRepeat::Times(1)),
    );

    ctx.run(50);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(&animation, &clip, 0),
            ctx.clip_end(&animation, &clip),
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

    let clip = Clip::from_frames([0, 1]).with_repetitions(10);

    let animation = ctx.attach_animation(
        Animation::from_clip(clip.clone())
            .with_duration(AnimationDuration::PerFrame(100))
            .with_repetitions(AnimationRepeat::Times(1)),
    );

    // 9 repetitions

    ctx.run(50);
    ctx.check(0, []);

    for i in 0..9 {
        ctx.run(100);
        ctx.check(1, []);

        ctx.run(100);
        ctx.check(0, [ctx.clip_rep_end(&animation, &clip, i)]);
    }

    // Last cycle

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(&animation, &clip, 9),
            ctx.clip_end(&animation, &clip),
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

    let zero_clip = Clip::from_frames([3, 2]).with_repetitions(0);

    let ok_clip = Clip::from_frames([9, 8]);

    let animation = ctx.attach_animation(
        Animation::from_clips([
            zero_clip.clone(),
            ok_clip.clone(),
            zero_clip.clone(),
            ok_clip.clone(),
            zero_clip.clone(),
            zero_clip.clone(),
            zero_clip.clone(),
        ])
        .with_duration(AnimationDuration::PerFrame(110))
        .with_repetitions(AnimationRepeat::Times(1)),
    );

    ctx.run(100);
    ctx.check(9, []);

    ctx.run(100);
    ctx.check(8, []);

    ctx.run(100);
    ctx.check(
        9,
        [
            ctx.clip_rep_end(&animation, &ok_clip, 0),
            ctx.clip_end(&animation, &ok_clip),
        ],
    );

    ctx.run(100);
    ctx.check(8, []);

    ctx.run(100);
    ctx.check(
        8,
        [
            ctx.clip_rep_end(&animation, &ok_clip, 0),
            ctx.clip_end(&animation, &ok_clip),
            ctx.anim_rep_end(&animation, 0),
            ctx.anim_end(&animation),
        ],
    );
}

#[test]
fn animation_zero() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1]).with_repetitions(1000); // should be ignored

    ctx.attach_animation(Animation::from_clip(clip).with_repetitions(AnimationRepeat::Times(0)));

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, []);
    }
}

#[test]
fn animation_once() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1]);

    let animation = ctx.attach_animation(
        Animation::from_clip(clip.clone())
            .with_duration(AnimationDuration::PerFrame(100))
            .with_repetitions(AnimationRepeat::Times(1)),
    );

    ctx.run(50);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(&animation, &clip, 0),
            ctx.clip_end(&animation, &clip),
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

    let clip = Clip::from_frames([0, 1]);

    let animation = ctx.attach_animation(
        Animation::from_clip(clip.clone())
            .with_duration(AnimationDuration::PerFrame(100))
            .with_repetitions(AnimationRepeat::Times(10)),
    );

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
                ctx.clip_rep_end(&animation, &clip, 0),
                ctx.clip_end(&animation, &clip),
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
            ctx.clip_rep_end(&animation, &clip, 0),
            ctx.clip_end(&animation, &clip),
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

    let clip = Clip::from_frames([0, 1]);

    let animation = ctx.attach_animation(
        Animation::from_clip(clip.clone())
            .with_duration(AnimationDuration::PerFrame(100))
            .with_repetitions(AnimationRepeat::Loop),
    );

    ctx.run(50);
    ctx.check(0, []);

    for i in 0..1000 {
        ctx.run(100); // 100 * i + 50
        ctx.check(1, []);

        ctx.run(100); // 100 * i + 150
        ctx.check(
            0,
            [
                ctx.clip_rep_end(&animation, &clip, 0),
                ctx.clip_end(&animation, &clip),
                ctx.anim_rep_end(&animation, i),
            ],
        );
    }
}
