pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn markers_emit_events() {
    let mut ctx = Context::new();

    let marker1_id = ctx.library().new_marker();
    let marker2_id = ctx.library().new_marker();

    let clip1_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1, 2])
            .add_marker(marker1_id, 0)
            .add_marker(marker2_id, 1)
            .add_marker(marker1_id, 2)
            .add_marker(marker2_id, 2);
    });

    let clip2_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([7, 8, 9])
            .add_marker(marker2_id, 0)
            .add_marker(marker1_id, 2);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation
            .add_stage(clip1_id.into())
            .add_stage(clip2_id.into())
            .set_duration(AnimationDuration::PerFrame(100));
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(50);
    ctx.check(0, [ctx.marker_hit(marker1_id, 0, animation_id)]);

    ctx.run(100); // 150
    ctx.check(1, [ctx.marker_hit(marker2_id, 0, animation_id)]);

    ctx.run(100); // 250
    ctx.check(
        2,
        [
            ctx.marker_hit(marker1_id, 0, animation_id),
            ctx.marker_hit(marker2_id, 0, animation_id),
        ],
    );

    ctx.run(100); // 350
    ctx.check(
        7,
        [
            ctx.marker_hit(marker2_id, 1, animation_id),
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
        ],
    );

    ctx.run(100); // 450
    ctx.check(8, []);

    ctx.run(100); // 550
    ctx.check(9, [ctx.marker_hit(marker1_id, 1, animation_id)]);

    // Loop

    ctx.run(100); // 650
    ctx.check(
        0,
        [
            ctx.marker_hit(marker1_id, 0, animation_id),
            ctx.clip_cycle_end(1, animation_id),
            ctx.clip_end(1, animation_id),
            ctx.anim_cycle_end(animation_id),
        ],
    );
}
