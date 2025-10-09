pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

// Backwards

#[test]
fn clip_backwards() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1, 2]).with_direction(AnimationDirection::Backwards);

    let animation = ctx.attach_animation(
        Animation::from_clip(clip.clone())
            .with_duration(AnimationDuration::PerFrame(100))
            .with_repetitions(AnimationRepeat::Times(2)),
    );

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
            ctx.clip_rep_end(&animation, &clip, 0),
            ctx.clip_end(&animation, &clip),
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

    let clip = Clip::from_frames([0, 1, 2]).with_direction(AnimationDirection::Forwards);

    let animation = ctx.attach_animation(
        Animation::from_clip(clip.clone())
            .with_direction(AnimationDirection::Backwards)
            .with_duration(AnimationDuration::PerFrame(100))
            .with_repetitions(AnimationRepeat::Times(2)),
    );

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
            ctx.clip_rep_end(&animation, &clip, 0),
            ctx.clip_end(&animation, &clip),
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

    let forward_clip = Clip::from_frames([0, 1, 2]).with_direction(AnimationDirection::Forwards);

    let backward_clip = Clip::from_frames([0, 1, 2]).with_direction(AnimationDirection::Backwards);

    let animation = ctx.attach_animation(
        Animation::from_clips([
            forward_clip.clone(),
            backward_clip.clone(),
            forward_clip.clone(),
        ])
        .with_direction(AnimationDirection::Backwards)
        .with_duration(AnimationDuration::PerFrame(100))
        .with_repetitions(AnimationRepeat::Loop),
    );

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
            ctx.clip_rep_end(&animation, &forward_clip, 0),
            ctx.clip_end(&animation, &forward_clip),
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
            ctx.clip_rep_end(&animation, &backward_clip, 0),
            ctx.clip_end(&animation, &backward_clip),
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
            ctx.clip_rep_end(&animation, &forward_clip, 0),
            ctx.clip_end(&animation, &forward_clip),
            ctx.anim_rep_end(&animation, 0),
        ],
    );
}

// PingPong

#[test]
fn clip_pingpong() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1, 2])
        .with_direction(AnimationDirection::PingPong)
        .with_repetitions(3);

    let animation = ctx.attach_animation(
        Animation::from_clip(clip.clone())
            .with_duration(AnimationDuration::PerFrame(100))
            .with_repetitions(AnimationRepeat::Loop),
    );

    // Ping

    ctx.run(50);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(2, []);

    // Pong

    ctx.run(100);
    ctx.check(1, [ctx.clip_rep_end(&animation, &clip, 0)]);

    ctx.run(100);
    ctx.check(0, []);

    // Ping again

    ctx.run(100);
    ctx.check(1, [ctx.clip_rep_end(&animation, &clip, 1)]);

    ctx.run(100);
    ctx.check(2, []);

    // Loop

    ctx.run(100);
    ctx.check(
        0,
        [
            ctx.clip_rep_end(&animation, &clip, 2),
            ctx.clip_end(&animation, &clip),
            ctx.anim_rep_end(&animation, 0),
        ],
    );
}

#[test]
fn animation_pingpong() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([0, 1, 2]);

    let animation = ctx.attach_animation(
        Animation::from_clip(clip.clone())
            .with_direction(AnimationDirection::PingPong)
            .with_duration(AnimationDuration::PerFrame(100))
            .with_repetitions(AnimationRepeat::Loop),
    );

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
            ctx.clip_rep_end(&animation, &clip, 0),
            ctx.clip_end(&animation, &clip),
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
            ctx.clip_rep_end(&animation, &clip, 0),
            ctx.clip_end(&animation, &clip),
            ctx.anim_rep_end(&animation, 1),
        ],
    );

    ctx.run(100);
    ctx.check(2, []);
}

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

    let clip = Clip::from_frames([0, 1, 2]).with_direction(AnimationDirection::Backwards);

    let animation = ctx.attach_animation(
        Animation::from_clip(clip.clone())
            .with_direction(AnimationDirection::PingPong)
            .with_duration(AnimationDuration::PerFrame(100))
            .with_repetitions(AnimationRepeat::Loop),
    );

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
            ctx.clip_rep_end(&animation, &clip, 0),
            ctx.clip_end(&animation, &clip),
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
            ctx.clip_rep_end(&animation, &clip, 0),
            ctx.clip_end(&animation, &clip),
            ctx.anim_rep_end(&animation, 1),
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

    let animation = ctx.attach_animation(
        Animation::from_clip(clip.clone())
            .with_direction(AnimationDirection::Backwards)
            .with_duration(AnimationDuration::PerFrame(100))
            .with_repetitions(AnimationRepeat::Loop),
    );

    // Pong

    ctx.run(50);
    ctx.check(0, []);

    ctx.run(100);
    ctx.check(1, []);

    // Ping

    ctx.run(100);
    ctx.check(2, [ctx.clip_rep_end(&animation, &clip, 0)]);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(0, []);

    // Pong again

    ctx.run(100);
    ctx.check(
        0,
        [
            ctx.clip_rep_end(&animation, &clip, 1),
            ctx.clip_end(&animation, &clip),
            ctx.anim_rep_end(&animation, 0),
        ],
    );

    ctx.run(100);
    ctx.check(1, []);
}
