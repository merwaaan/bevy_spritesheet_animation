pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

// Clip

#[test]
fn clip_backwards() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1, 2])
            .set_default_direction(AnimationDirection::Backwards);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation
            .add_stage(clip_id.into())
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Cycles(2));
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(50);
    ctx.check(2, &[]);

    ctx.run(100);
    ctx.check(1, &[]);

    ctx.run(100);
    ctx.check(0, &[]);

    ctx.run(100);
    ctx.check(
        2,
        &[
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
        ],
    );

    ctx.run(100);
    ctx.check(1, &[]);

    ctx.run(100);
    ctx.check(0, &[]);
}

// Stage

#[test]
fn stage_backwards() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1, 2])
            .set_default_direction(AnimationDirection::PingPong); // should be ignored
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage.set_direction(AnimationDirection::Backwards);

        animation
            .add_stage(stage)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Cycles(2));
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(50);
    ctx.check(2, &[]);

    ctx.run(100);
    ctx.check(1, &[]);

    ctx.run(100);
    ctx.check(0, &[]);

    ctx.run(100);
    ctx.check(
        2,
        &[
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
        ],
    );

    ctx.run(100);
    ctx.check(1, &[]);

    ctx.run(100);
    ctx.check(0, &[]);
}

// Stage

#[test]
fn animation_backwards() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1, 2])
            .set_default_direction(AnimationDirection::PingPong); // should be ignored
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage.set_direction(AnimationDirection::Forwards); // should be ignored

        animation
            .add_stage(stage)
            .set_direction(AnimationDirection::Backwards)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Cycles(2));
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(50);
    ctx.check(2, &[]);

    ctx.run(100);
    ctx.check(1, &[]);

    ctx.run(100);
    ctx.check(0, &[]);

    ctx.run(100);
    ctx.check(
        2,
        &[
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
        ],
    );

    ctx.run(100);
    ctx.check(1, &[]);

    ctx.run(100);
    ctx.check(0, &[]);
}

// TODO anim backwards + stage backwards
