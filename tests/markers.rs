pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn markers_emit_events() {
    let mut ctx = Context::new();

    let mut clip1_id = ClipId::dummy();
    let clip1_marker1 = Marker::new();
    let clip1_marker2 = Marker::new();
    let clip1_marker3 = Marker::new();
    let clip1_marker4 = Marker::new();

    let mut clip2_id = ClipId::dummy();
    let clip2_marker1 = Marker::new();
    let clip2_marker2 = Marker::new();

    let animation = ctx.attach_animation(|builder| {
        builder
            .set_duration(AnimationDuration::PerFrame(100))
            // Clip 1
            .add_indices([0, 1, 2])
            .add_clip_marker(clip1_marker1, 0)
            .add_clip_marker(clip1_marker2, 1)
            .add_clip_marker(clip1_marker3, 2)
            .add_clip_marker(clip1_marker4, 2)
            .get_current_clip_id(&mut clip1_id)
            // Clip 2
            .start_clip()
            .add_indices([7, 8, 9])
            .add_clip_marker(clip2_marker1, 0)
            .add_clip_marker(clip2_marker2, 2)
            .get_current_clip_id(&mut clip2_id)
    });

    ctx.run(50);
    ctx.check(
        0,
        [ctx.marker_hit(clip1_marker1, &animation, 0, clip1_id, 0)],
    );

    ctx.run(100); // 150
    ctx.check(
        1,
        [ctx.marker_hit(clip1_marker2, &animation, 0, clip1_id, 0)],
    );

    ctx.run(100); // 250
    ctx.check(
        2,
        [
            ctx.marker_hit(clip1_marker3, &animation, 0, clip1_id, 0),
            ctx.marker_hit(clip1_marker4, &animation, 0, clip1_id, 0),
        ],
    );

    ctx.run(100); // 350
    ctx.check(
        7,
        [
            ctx.marker_hit(clip2_marker1, &animation, 0, clip2_id, 0),
            ctx.clip_rep_end(&animation, clip1_id, 0),
            ctx.clip_end(&animation, clip1_id),
        ],
    );

    ctx.run(100); // 450
    ctx.check(8, []);

    ctx.run(100); // 550
    ctx.check(
        9,
        [ctx.marker_hit(clip2_marker2, &animation, 0, clip2_id, 0)],
    );

    // Loop

    ctx.run(100); // 650
    ctx.check(
        0,
        [
            ctx.marker_hit(clip1_marker1, &animation, 1, clip1_id, 0),
            ctx.clip_rep_end(&animation, clip2_id, 0),
            ctx.clip_end(&animation, clip2_id),
            ctx.anim_rep_end(&animation, 0),
        ],
    );
}
