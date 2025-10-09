pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::Context;

#[test]
fn animation_with_some_empty_clips() {
    let mut ctx = Context::new();

    let mut empty_clip_id = ClipId::dummy();
    let mut ok_clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .set_duration(AnimationDuration::PerFrame(110))
            .get_current_clip_id(&mut empty_clip_id)
            .start_clip()
            .add_indices([9, 8])
            .get_current_clip_id(&mut ok_clip_id)
            .copy_clip(empty_clip_id)
            .copy_clip(empty_clip_id)
            .copy_clip(empty_clip_id)
            .copy_clip(ok_clip_id)
            .copy_clip(empty_clip_id)
            .copy_clip(empty_clip_id)
    });

    ctx.run(100);
    ctx.check(9, []);

    ctx.run(100); // 0.2
    ctx.check(8, []);

    ctx.run(100); // 0.3
    ctx.check(
        9,
        [
            ctx.clip_rep_end(&animation, ok_clip_id, 0),
            ctx.clip_end(&animation, ok_clip_id),
        ],
    );

    ctx.run(100); // 0.4
    ctx.check(8, []);
}

#[test]
fn animation_assigned_while_paused() {
    let mut ctx = Context::new();

    ctx.attach_animation(|builder| builder.add_indices([4, 5]));

    // Start paused, the first frame should be assigned anyway

    ctx.get_sprite(|sprite| {
        sprite.playing = false;
    });

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(4, []);
    }

    // Stay paused and change the animation with switch(), the first frame of the new animation should be assigned anyway

    let animation2 = ctx.create_animation(|builder| builder.add_indices([7, 8]));

    ctx.get_sprite(|sprite| {
        sprite.switch(animation2.clone());
    });

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(7, []);
    }

    // Same thing with a direct assignation

    let animation3 = ctx.create_animation(|builder| builder.add_indices([0, 1]));

    ctx.get_sprite(|sprite| {
        sprite.animation = animation3.clone();
    });

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, []);
    }
}
