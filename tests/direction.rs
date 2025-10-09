pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

// Backwards

#[test]
fn clip_backwards() {
    let mut ctx = Context::new();

    let mut clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .add_indices([0, 1, 2])
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Times(2))
            .set_clip_direction(AnimationDirection::Backwards)
            .get_current_clip_id(&mut clip_id)
    });

    ctx.run(50);
    ctx.check(2, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(
        2,
        [
            ctx.clip_rep_end(&animation, clip_id, 0),
            ctx.clip_end(&animation, clip_id),
            ctx.anim_rep_end(&animation, 0),
        ],
    );

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(0, []);
}

#[test]
fn animation_backwards() {
    let mut ctx = Context::new();

    let mut clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .add_indices([0, 1, 2])
            .set_direction(AnimationDirection::Backwards)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Times(2))
            .get_current_clip_id(&mut clip_id)
    });

    ctx.run(50);
    ctx.check(2, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(
        2,
        [
            ctx.clip_rep_end(&animation, clip_id, 0),
            ctx.clip_end(&animation, clip_id),
            ctx.anim_rep_end(&animation, 0),
        ],
    );

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(0, []);
}

#[test]
fn animation_backwards_clip_backwards() {
    let mut ctx = Context::new();

    // Backward clip sandwiched between two forward clips

    let mut forward_clip_id = ClipId::dummy();
    let mut backward_clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .set_direction(AnimationDirection::Backwards)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Loop)
            // Clip 1
            .add_indices([0, 1, 2])
            .set_clip_direction(AnimationDirection::Forwards)
            .get_current_clip_id(&mut forward_clip_id)
            // Clip 2
            .start_clip()
            .add_indices([0, 1, 2])
            .set_clip_direction(AnimationDirection::Backwards)
            .get_current_clip_id(&mut backward_clip_id)
            // Clip 3: copy of clip 1
            .copy_clip(forward_clip_id)
    });

    // clip 3 (played backwards)

    ctx.run(50);
    ctx.check(2, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(0, []);

    // clip 2 (played backwards but was backwards so now forwards!)

    ctx.run(100);
    ctx.check(
        0,
        [
            ctx.clip_rep_end(&animation, forward_clip_id, 0),
            ctx.clip_end(&animation, forward_clip_id),
        ],
    );

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(2, []);

    // clip 1 (played backwards)

    ctx.run(100);
    ctx.check(
        2,
        [
            ctx.clip_rep_end(&animation, backward_clip_id, 0),
            ctx.clip_end(&animation, backward_clip_id),
        ],
    );

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(0, []);

    // Loop

    ctx.run(100);
    ctx.check(
        2,
        [
            ctx.clip_rep_end(&animation, forward_clip_id, 0),
            ctx.clip_end(&animation, forward_clip_id),
            ctx.anim_rep_end(&animation, 0),
        ],
    );
}

// PingPong

#[test]
fn clip_pingpong() {
    let mut ctx = Context::new();

    let mut clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Loop)
            .add_indices([0, 1, 2])
            .set_clip_direction(AnimationDirection::PingPong)
            .set_clip_repetitions(3)
            .get_current_clip_id(&mut clip_id)
    });

    // Ping

    ctx.run(50);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(2, []);

    // Pong

    ctx.run(100);
    ctx.check(1, [ctx.clip_rep_end(&animation, clip_id, 0)]);

    ctx.run(100);
    ctx.check(0, []);

    // Ping again

    ctx.run(100);
    ctx.check(1, [ctx.clip_rep_end(&animation, clip_id, 1)]);

    ctx.run(100);
    ctx.check(2, []);

    // Loop

    ctx.run(100);
    ctx.check(
        0,
        [
            ctx.clip_rep_end(&animation, clip_id, 2),
            ctx.clip_end(&animation, clip_id),
            ctx.anim_rep_end(&animation, 0),
        ],
    );
}

#[test]
fn animation_pingpong() {
    let mut ctx = Context::new();

    let mut clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .set_direction(AnimationDirection::PingPong)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Loop)
            .add_indices([0, 1, 2])
            .get_current_clip_id(&mut clip_id)
    });

    // Ping

    ctx.run(50);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(2, []);

    // Pong

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(&animation, clip_id, 0),
            ctx.clip_end(&animation, clip_id),
            ctx.anim_rep_end(&animation, 0),
        ],
    );

    ctx.run(100);
    ctx.check(0, []);

    // Ping again

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(&animation, clip_id, 0),
            ctx.clip_end(&animation, clip_id),
            ctx.anim_rep_end(&animation, 1),
        ],
    );

    ctx.run(100);
    ctx.check(2, []);
}

// TODO
// #[test]
// fn animation_pingpong_clip_pingpong() {
//     let mut ctx = Context::new();

//     let (clip, clip) = ctx.library().new_clip();
//         clip.push_frame_indices([0, 1, 2])
//             .with_direction(AnimationDirection::PingPong)
//             .with_repetitions(2); // Needed for the ping-pong or we would only get pongs
//

//     let (animation, animation) = ctx.library().new_animation();
//         animation
//
//             .with_direction(AnimationDirection::PingPong)
//             .with_duration(AnimationDuration::PerFrame(100))
//             .with_repetitions(AnimationRepeat::Loop);
//

//     ctx.attach_animation(&animation);

//     // Animation ping
//     // Clip ping

//     ctx.run(50);
//     ctx.check(0, []);

//     ctx.run(100);
//     ctx.check(1, []);

//     ctx.run(100);
//     ctx.check(2, []);

//     // Clip pong

//     ctx.run(100);
//     ctx.check(1, [ctx.clip_rep_end(0, animation)]);

//     ctx.run(100);
//     ctx.check(0, []);

//     // Animation pong
//     // Clip pong

//     ctx.run(100);
//     ctx.check(
//         1,
//         [
//             ctx.clip_rep_end(&animation, clip, 0),
//             ctx.clip_end(&animation, &clip),
//             ctx.anim_rep_end(&animation, 0),
//         ],
//     );

//     ctx.run(100);
//     ctx.check(2, []);

//     // Clip ping

//     ctx.run(100);
//     ctx.check(
//         1,
//         [
//             ctx.clip_rep_end(&animation, clip, 0),
//             ctx.clip_end(&animation, &clip),
//             ctx.anim_rep_end(&animation, 0),
//         ],
//     );

//     ctx.run(100);
//     ctx.check(0, []);
// }

#[test]
fn animation_pingpong_clip_backwards() {
    let mut ctx = Context::new();

    let mut clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .set_direction(AnimationDirection::PingPong)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Loop)
            .add_indices([0, 1, 2])
            .set_clip_direction(AnimationDirection::Backwards)
            .get_current_clip_id(&mut clip_id)
    });

    // Ping

    ctx.run(50);
    ctx.check(2, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(0, []);

    // Pong

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(&animation, clip_id, 0),
            ctx.clip_end(&animation, clip_id),
            ctx.anim_rep_end(&animation, 0),
        ],
    );

    ctx.run(100);
    ctx.check(2, []);

    // Ping again

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_rep_end(&animation, clip_id, 0),
            ctx.clip_end(&animation, clip_id),
            ctx.anim_rep_end(&animation, 1),
        ],
    );

    ctx.run(100);
    ctx.check(0, []);
}

#[test]
fn animation_backwards_clip_pingpong() {
    let mut ctx = Context::new();

    let mut clip_id = ClipId::dummy();

    let animation = ctx.attach_animation(|builder| {
        builder
            .set_direction(AnimationDirection::Backwards)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Loop)
            .add_indices([0, 1, 2])
            .set_clip_direction(AnimationDirection::PingPong)
            .set_clip_repetitions(2) // Needed for the ping-pong or we would only get pongs
            .get_current_clip_id(&mut clip_id)
    });

    // Pong

    ctx.run(50);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(1, []);

    // Ping

    ctx.run(100);
    ctx.check(2, [ctx.clip_rep_end(&animation, clip_id, 0)]);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(0, []);

    // Pong again

    ctx.run(100);
    ctx.check(
        0,
        [
            ctx.clip_rep_end(&animation, clip_id, 1),
            ctx.clip_end(&animation, clip_id),
            ctx.anim_rep_end(&animation, 0),
        ],
    );

    ctx.run(100);
    ctx.check(1, []);
}
