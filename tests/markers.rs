pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn markers_emit_events() {
    let mut ctx = Context::new();

    let marker1_id = ctx.library().new_marker();
    let marker2_id = ctx.library().new_marker();

    let clip1 = Clip::from_frames([0, 1, 2])
        .with_marker(marker1_id, 0)
        .with_marker(marker2_id, 1)
        .with_marker(marker1_id, 2)
        .with_marker(marker2_id, 2);
    let clip1_id = ctx.library().register_clip(clip1);

    let clip2 = Clip::from_frames([7, 8, 9])
        .with_marker(marker2_id, 0)
        .with_marker(marker1_id, 2);
    let clip2_id = ctx.library().register_clip(clip2);

    let animation =
        Animation::from_clips([clip1_id, clip2_id]).with_duration(AnimationDuration::PerFrame(100));
    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(50);
    ctx.check(
        0,
        [ctx.marker_hit(marker1_id, animation_id, 0, clip1_id, 0)],
    );

    ctx.run(100); // 150
    ctx.check(
        1,
        [ctx.marker_hit(marker2_id, animation_id, 0, clip1_id, 0)],
    );

    ctx.run(100); // 250
    ctx.check(
        2,
        [
            ctx.marker_hit(marker1_id, animation_id, 0, clip1_id, 0),
            ctx.marker_hit(marker2_id, animation_id, 0, clip1_id, 0),
        ],
    );

    ctx.run(100); // 350
    ctx.check(
        7,
        [
            ctx.marker_hit(marker2_id, animation_id, 0, clip2_id, 0),
            ctx.clip_rep_end(animation_id, clip1_id, 0),
            ctx.clip_end(animation_id, clip1_id),
        ],
    );

    ctx.run(100); // 450
    ctx.check(8, []);

    ctx.run(100); // 550
    ctx.check(
        9,
        [ctx.marker_hit(marker1_id, animation_id, 0, clip2_id, 0)],
    );

    // Loop

    ctx.run(100); // 650
    ctx.check(
        0,
        [
            ctx.marker_hit(marker1_id, animation_id, 1, clip1_id, 0),
            ctx.clip_rep_end(animation_id, clip2_id, 0),
            ctx.clip_end(animation_id, clip2_id),
            ctx.anim_rep_end(animation_id, 0),
        ],
    );
}
