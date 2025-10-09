pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn markers_emit_events() {
    let mut ctx = Context::new();

    let mut clip1 = Clip::from_frames([0, 1, 2]);
    let clip1_marker1 = clip1.add_marker(0);
    let clip1_marker2 = clip1.add_marker(1);
    let clip1_marker3 = clip1.add_marker(2);
    let clip1_marker4 = clip1.add_marker(2);

    let mut clip2 = Clip::from_frames([7, 8, 9]);
    let clip2_marker1 = clip2.add_marker(0);
    let clip2_marker2 = clip2.add_marker(2);

    let animation = ctx.attach_animation(
        Animation::from_clips([clip1.clone(), clip2.clone()])
            .with_duration(AnimationDuration::PerFrame(100)),
    );

    ctx.run(50);
    ctx.check(0, [ctx.marker_hit(clip1_marker1, &animation, 0, &clip1, 0)]);

    ctx.run(100); // 150
    ctx.check(1, [ctx.marker_hit(clip1_marker2, &animation, 0, &clip1, 0)]);

    ctx.run(100); // 250
    ctx.check(
        2,
        [
            ctx.marker_hit(clip1_marker3, &animation, 0, &clip1, 0),
            ctx.marker_hit(clip1_marker4, &animation, 0, &clip1, 0),
        ],
    );

    ctx.run(100); // 350
    ctx.check(
        7,
        [
            ctx.marker_hit(clip2_marker1, &animation, 0, &clip2, 0),
            ctx.clip_rep_end(&animation, &clip1, 0),
            ctx.clip_end(&animation, &clip1),
        ],
    );

    ctx.run(100); // 450
    ctx.check(8, []);

    ctx.run(100); // 550
    ctx.check(9, [ctx.marker_hit(clip2_marker2, &animation, 0, &clip2, 0)]);

    // Loop

    ctx.run(100); // 650
    ctx.check(
        0,
        [
            ctx.marker_hit(clip1_marker1, &animation, 1, &clip1, 0),
            ctx.clip_rep_end(&animation, &clip2, 0),
            ctx.clip_end(&animation, &clip2),
            ctx.anim_rep_end(&animation, 0),
        ],
    );
}
