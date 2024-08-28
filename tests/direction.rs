pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

// Backwards

#[test]
fn clip_backwards() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1, 2]).with_direction(AnimationDirection::Backwards);
    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Times(2));
    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

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
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
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

    let clip = Clip::from_frames([0, 1, 2]).with_direction(AnimationDirection::Forwards);

    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id)
        .with_direction(AnimationDirection::Backwards)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Times(2));

    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

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
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
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

    let forward_clip = Clip::from_frames([0, 1, 2]).with_direction(AnimationDirection::Forwards);

    let forward_clip_id = ctx.library().register_clip(forward_clip);

    let backward_clip = Clip::from_frames([0, 1, 2]).with_direction(AnimationDirection::Backwards);

    let backward_clip_id = ctx.library().register_clip(backward_clip);

    let animation = Animation::from_clips([forward_clip_id, backward_clip_id, forward_clip_id])
        .with_direction(AnimationDirection::Backwards)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Loop);

    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

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
            ctx.clip_cycle_end(2, animation_id),
            ctx.clip_end(2, animation_id),
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
            ctx.clip_cycle_end(1, animation_id),
            ctx.clip_end(1, animation_id),
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
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
        ],
    );
}

// PingPong

#[test]
fn clip_pingpong() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1, 2])
        .with_direction(AnimationDirection::PingPong)
        .with_repetitions(3); // Needed for the ping-pong or we would only get pongs

    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Loop);

    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

    // Ping

    ctx.run(50);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(2, []);

    // Pong

    ctx.run(100);
    ctx.check(1, [ctx.clip_cycle_end(0, animation_id)]);

    ctx.run(100);
    ctx.check(0, []);

    // Ping again

    ctx.run(100);
    ctx.check(1, [ctx.clip_cycle_end(0, animation_id)]);

    ctx.run(100);
    ctx.check(2, []);

    // Loop

    ctx.run(100);
    ctx.check(
        0,
        [
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
        ],
    );
}

#[test]
fn animation_pingpong() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1, 2]);
    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id)
        .with_direction(AnimationDirection::PingPong)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Loop);

    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

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
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
        ],
    );

    ctx.run(100);
    ctx.check(0, []);

    // Ping again

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
        ],
    );

    ctx.run(100);
    ctx.check(2, []);
}

// #[test]
// fn animation_pingpong_clip_pingpong() {
//     let mut ctx = Context::new();

//     let (clip_id, clip) = ctx.library().new_clip();
//         clip.push_frame_indices([0, 1, 2])
//             .with_direction(AnimationDirection::PingPong)
//             .with_repetitions(2); // Needed for the ping-pong or we would only get pongs
//

//     let (animation_id, animation) = ctx.library().new_animation();
//         animation
//
//             .with_direction(AnimationDirection::PingPong)
//             .with_duration(AnimationDuration::PerFrame(100))
//             .with_repetitions(AnimationRepeat::Loop);
//

//     ctx.add_animation_to_sprite(animation_id);

//     // Animation ping
//     // Stage ping

//     ctx.run(50);
//     ctx.check(0, []);

//     ctx.run(100);
//     ctx.check(1, []);

//     ctx.run(100);
//     ctx.check(2, []);

//     // Stage pong

//     ctx.run(100);
//     ctx.check(1, [ctx.clip_cycle_end(0, animation_id)]);

//     ctx.run(100);
//     ctx.check(0, []);

//     // Animation pong
//     // Stage pong

//     ctx.run(100);
//     ctx.check(
//         1,
//         [
//             ctx.clip_cycle_end(0, animation_id),
//             ctx.clip_end(0, animation_id),
//             ctx.anim_cycle_end(animation_id),
//         ],
//     );

//     ctx.run(100);
//     ctx.check(2, []);

//     // Stage ping

//     ctx.run(100);
//     ctx.check(
//         1,
//         [
//             ctx.clip_cycle_end(0, animation_id),
//             ctx.clip_end(0, animation_id),
//             ctx.anim_cycle_end(animation_id),
//         ],
//     );

//     ctx.run(100);
//     ctx.check(0, []);
// }

#[test]
fn animation_pingpong_clip_backwards() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1, 2]).with_direction(AnimationDirection::Backwards);

    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id)
        .with_direction(AnimationDirection::PingPong)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Loop);

    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

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
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
        ],
    );

    ctx.run(100);
    ctx.check(2, []);

    // Ping again

    ctx.run(100);
    ctx.check(
        1,
        [
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
        ],
    );

    ctx.run(100);
    ctx.check(0, []);
}

#[test]
fn animation_backwards_clip_pingpong() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1, 2])
        .with_direction(AnimationDirection::PingPong)
        .with_repetitions(2); // Needed for the ping-pong or we would only get pongs

    let clip_id = ctx.library().register_clip(clip);

    let animation = Animation::from_clip(clip_id)
        .with_direction(AnimationDirection::Backwards)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Loop);

    let animation_id = ctx.library().register_animation(animation);

    ctx.add_animation_to_sprite(animation_id);

    // Pong

    ctx.run(50);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(1, []);

    // Ping

    ctx.run(100);
    ctx.check(2, [ctx.clip_cycle_end(0, animation_id)]);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(0, []);

    // Pong again

    ctx.run(100);
    ctx.check(
        0,
        [
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
            ctx.anim_cycle_end(animation_id),
        ],
    );

    ctx.run(100);
    ctx.check(1, []);
}
