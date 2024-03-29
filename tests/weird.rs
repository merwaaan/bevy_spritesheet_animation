pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

// Clips

#[test]
fn clip_without_frames() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|_clip| {});

    let animation_id = ctx.library().new_animation(|animation| {
        animation.add_stage(clip_id.into());
    });

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, &[]);
    }
}

#[test]
fn clip_repeated_zero_times() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([1, 2, 3]).set_default_repeat(0);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation.add_stage(clip_id.into());
    });

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, &[]);
    }
}

#[test]
fn clip_with_zero_duration() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([4, 5, 6])
            .set_default_duration(AnimationDuration::PerFrame(0));
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation.add_stage(clip_id.into());
    });

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, &[]);
    }
}

// Stages

#[test]
fn stage_repeated_zero_times() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([1, 2, 3]);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage.set_repeat(0);

        animation.add_stage(stage);
    });

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, &[]);
    }
}

#[test]
fn stage_with_zero_duration() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([4, 5, 6]);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage.set_duration(AnimationDuration::PerFrame(0));

        animation.add_stage(stage);
    });

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, &[]);
    }
}

// Animations

#[test]
fn animation_without_stages() {
    let mut ctx = Context::new();

    let animation_id = ctx.library().new_animation(|_animation| {});

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, &[]);
    }
}

#[test]
fn animation_with_some_empty_clips() {
    let mut ctx = Context::new();

    let empty_clip_id = ctx.library().new_clip(|_clip| {});

    let ok_clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([9, 8]);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation
            .add_stage(empty_clip_id.into())
            .add_stage(ok_clip_id.into())
            .add_stage(empty_clip_id.into())
            .add_stage(ok_clip_id.into())
            .add_stage(empty_clip_id.into())
            .set_duration(AnimationDuration::PerFrame(110));
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(100);
    ctx.check(9, &[]);

    ctx.run(100); // 0.2
    ctx.check(8, &[]);

    ctx.run(100); // 0.3
    ctx.check(
        9,
        &[
            ctx.stage_cycle_end(1, animation_id),
            ctx.stage_end(1, animation_id),
        ],
    );

    ctx.run(100); // 0.4
    ctx.check(8, &[]);
}

#[test]
fn animation_repeated_zero_times() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([1, 2, 3]);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation
            .add_stage(clip_id.into())
            .set_repeat(AnimationRepeat::Cycles(0));
    });

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, &[]);
    }
}

#[test]
fn animation_with_some_clips_repeated_zero_times() {
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
            .set_duration(AnimationDuration::PerFrame(110));
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
            ctx.stage_cycle_end(1, animation_id),
            ctx.stage_end(1, animation_id),
        ],
    );

    ctx.run(100);
    ctx.check(8, &[]);
}
