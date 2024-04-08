pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn clip_without_frames() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|_| {});

    let animation_id = ctx.library().new_animation(|animation| {
        animation.add_stage(clip_id.into());
    });

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, []);
    }
}

#[test]
fn animation_without_stages() {
    let mut ctx = Context::new();

    let animation_id = ctx.library().new_animation(|_| {});

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, []);
    }
}

#[test]
fn animation_with_some_empty_clips() {
    let mut ctx = Context::new();

    let empty_clip_id = ctx.library().new_clip(|_| {});

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
    ctx.check(9, []);

    ctx.run(100); // 0.2
    ctx.check(8, []);

    ctx.run(100); // 0.3
    ctx.check(
        9,
        [
            ctx.clip_cycle_end(1, animation_id),
            ctx.clip_end(1, animation_id),
        ],
    );

    ctx.run(100); // 0.4
    ctx.check(8, []);
}

#[test]
fn animation_assigned_while_paused() {
    let mut ctx = Context::new();

    let clip1_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([4, 5]);
    });

    let animation1_id = ctx.library().new_animation(|animation| {
        animation.add_stage(clip1_id.into());
    });

    let clip2_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([7, 8]);
    });

    let animation2_id = ctx.library().new_animation(|animation| {
        animation.add_stage(clip2_id.into());
    });

    // Start paused, the first frame should be assigned anyway

    ctx.add_animation_to_sprite(animation1_id);

    ctx.update_sprite_animation(|anim| {
        anim.playing = false;
    });

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(4, []);
    }

    // Stay paused and change the animation, the first frame of the new animation should be assigned anyway

    ctx.update_sprite_animation(|anim| {
        anim.animation_id = animation2_id;
    });

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(7, []);
    }
}
