pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

// Clip

#[test]
fn clip_zero() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1]).set_default_repeat(0);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation
            .add_stage(clip_id.into())
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Cycles(1));
    });

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, &[]);
    }
}

#[test]
fn clip_once() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1]).set_default_repeat(1);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation
            .add_stage(clip_id.into())
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Cycles(1));
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(50);
    ctx.check(0, &[]);

    ctx.run(100);
    ctx.check(1, &[]);

    ctx.run(100);
    ctx.check(
        1,
        &[
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
            ctx.anim_end(animation_id),
        ],
    );

    // Over

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(1, &[]);
    }
}

#[test]
fn clip_many() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1]).set_default_repeat(10);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation
            .add_stage(clip_id.into())
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Cycles(1));
    });

    ctx.add_animation_to_sprite(animation_id);

    // 9 cycles

    ctx.run(50);
    ctx.check(0, &[]);

    for _ in 0..9 {
        ctx.run(100);
        ctx.check(1, &[]);

        ctx.run(100);
        ctx.check(0, &[ctx.clip_cycle_end(0, animation_id)]);
    }

    // Last cycle

    ctx.run(100);
    ctx.check(1, &[]);

    ctx.run(100);
    ctx.check(
        1,
        &[
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
            ctx.anim_end(animation_id),
        ],
    );

    // Over

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(1, &[]);
    }
}

// Stage

#[test]
fn stage_zero() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1]).set_default_repeat(99999); // should be ignored
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage.set_repeat(0);

        animation
            .add_stage(stage)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Cycles(1));
    });

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, &[]);
    }
}

#[test]
fn stage_once() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1]).set_default_repeat(1000); // should be ignored
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage.set_repeat(1);

        animation
            .add_stage(stage)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Cycles(1));
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(50);
    ctx.check(0, &[]);

    ctx.run(100);
    ctx.check(1, &[]);

    ctx.run(100);
    ctx.check(
        1,
        &[
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
            ctx.anim_end(animation_id),
        ],
    );

    // Over

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(1, &[]);
    }
}

#[test]
fn stage_many() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1]).set_default_repeat(1000); // should be ignored
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage.set_repeat(10);

        animation
            .add_stage(stage)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Cycles(1));
    });

    ctx.add_animation_to_sprite(animation_id);

    // 9 cycles

    ctx.run(50);
    ctx.check(0, &[]);

    for _ in 0..9 {
        ctx.run(100);
        ctx.check(1, &[]);

        ctx.run(100);
        ctx.check(0, &[ctx.clip_cycle_end(0, animation_id)]);
    }

    // Last cycle

    ctx.run(100);
    ctx.check(1, &[]);

    ctx.run(100);
    ctx.check(
        1,
        &[
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
            ctx.anim_end(animation_id),
        ],
    );

    // Over

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(1, &[]);
    }
}

#[test]
fn some_clips_repeated_zero_times() {
    let mut ctx = Context::new();

    let zero_clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([3, 2]).set_default_repeat(0);
    });

    let ok_clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([9, 8]);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation
            .add_stage(zero_clip_id.into())
            .add_stage(ok_clip_id.into())
            .add_stage(zero_clip_id.into())
            .add_stage(ok_clip_id.into())
            .add_stage(zero_clip_id.into())
            .add_stage(zero_clip_id.into())
            .add_stage(zero_clip_id.into())
            .set_duration(AnimationDuration::PerFrame(110))
            .set_repeat(AnimationRepeat::Cycles(1));
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(100);
    ctx.check(9, &[]);

    ctx.run(100);
    ctx.check(8, &[]);

    ctx.run(100);
    ctx.check(
        9,
        &[
            ctx.clip_cycle_end(1, animation_id),
            ctx.clip_end(1, animation_id),
        ],
    );

    ctx.run(100);
    ctx.check(8, &[]);

    ctx.run(100);
    ctx.check(
        8,
        &[
            ctx.clip_cycle_end(3, animation_id),
            ctx.clip_end(3, animation_id),
            ctx.anim_cycle_end(animation_id),
            ctx.anim_end(animation_id),
        ],
    );
}

// Animation

#[test]
fn animation_zero() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1]).set_default_repeat(1000); // should be ignored
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage.set_repeat(1000); // should be ignored

        animation
            .add_stage(stage)
            .set_repeat(AnimationRepeat::Cycles(0));
    });

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, &[]);
    }
}

#[test]
fn animation_once() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1]);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation
            .add_stage(clip_id.into())
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Cycles(1));
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(50);
    ctx.check(0, &[]);

    ctx.run(100);
    ctx.check(1, &[]);

    ctx.run(100);
    ctx.check(
        1,
        &[
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
            ctx.anim_end(animation_id),
        ],
    );

    // Over

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(1, &[]);
    }
}

#[test]
fn animation_many() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1]);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation
            .add_stage(clip_id.into())
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Cycles(10));
    });

    ctx.add_animation_to_sprite(animation_id);

    // 9 cycles

    ctx.run(50);
    ctx.check(0, &[]);

    for _ in 0..9 {
        ctx.run(100);
        ctx.check(1, &[]);

        ctx.run(100);
        ctx.check(
            0,
            &[
                ctx.clip_cycle_end(0, animation_id),
                ctx.clip_end(0, animation_id),
                ctx.anim_cycle_end(animation_id),
            ],
        );
    }

    // Last cycle

    ctx.run(100);
    ctx.check(1, &[]);

    ctx.run(100);
    ctx.check(
        1,
        &[
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
            ctx.anim_end(animation_id),
        ],
    );

    // Over

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(1, &[]);
    }
}

#[test]
fn animation_forever() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1]);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation
            .add_stage(clip_id.into())
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Loop);
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(50);
    ctx.check(0, &[]);

    for _ in 0..1000 {
        ctx.run(100);
        ctx.check(1, &[]);

        ctx.run(100);
        ctx.check(
            0,
            &[
                ctx.clip_cycle_end(0, animation_id),
                ctx.clip_end(0, animation_id),
                ctx.anim_cycle_end(animation_id),
            ],
        );
    }
}
