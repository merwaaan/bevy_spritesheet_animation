use bevy_spritesheet_animation::prelude::*;
use context::Context;

pub mod context;

#[test]
fn clip_without_frames() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([]);
    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id);
    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, []);
    }
}

#[test]
fn animation_without_clips() {
    let mut ctx = Context::new();

    let animation = Animation::from_clips([]);
    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, []);
    }
}

#[test]
fn animation_with_some_empty_clips() {
    let mut ctx = Context::new();

    let empty_clip = Clip::from_frames([]);
    let empty_clip_id = ctx.library().register_clip(empty_clip);

    let ok_clip = Clip::from_frames([9, 8]);
    let ok_clip_id = ctx.library().register_clip(ok_clip);

    let animation = Animation::from_clips([
        empty_clip_id,
        ok_clip_id,
        empty_clip_id,
        empty_clip_id,
        empty_clip_id,
        ok_clip_id,
        empty_clip_id,
        empty_clip_id,
    ])
    .with_duration(AnimationDuration::PerFrame(110));

    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(100);
    ctx.check(9, []);

    ctx.run(100); // 0.2
    ctx.check(8, []);

    ctx.run(100); // 0.3
    ctx.check(
        9,
        [
            ctx.clip_rep_end(animation_id, ok_clip_id, 0),
            ctx.clip_end(animation_id, ok_clip_id),
        ],
    );

    ctx.run(100); // 0.4
    ctx.check(8, []);
}

#[test]
fn animation_assigned_while_paused() {
    let mut ctx = Context::new();

    let clip1 = Clip::from_frames([4, 5]);
    let clip1_id = ctx.library().register_clip(clip1);

    let animation1 = Animation::from_clip(clip1_id);
    let animation1_id = ctx.library().register_animation(animation1);

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

    let clip2 = Clip::from_frames([7, 8]);
    let clip2_id = ctx.library().register_clip(clip2);

    let animation2 = Animation::from_clip(clip2_id);
    let animation2_id = ctx.library().register_animation(animation2);

    ctx.update_sprite_animation(|anim| {
        anim.switch(animation2_id);
    });

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(7, []);
    }
}
