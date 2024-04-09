pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

// Backwards

#[test]
fn stage_backwards() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1, 2])
            .set_default_direction(AnimationDirection::PingPong); // should be ignored
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage.set_direction(AnimationDirection::Backwards);

        animation
            .add_stage(stage)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Cycles(2));
    });

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

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1, 2])
            .set_default_direction(AnimationDirection::PingPong); // should be ignored
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage.set_direction(AnimationDirection::Forwards);

        animation
            .add_stage(stage)
            .set_direction(AnimationDirection::Backwards)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Cycles(2));
    });

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
fn animation_backwards_stage_backwards() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1, 2]);
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage.set_direction(AnimationDirection::Backwards);

        // Backward stage sandwiched between two forward stages

        animation
            .add_stage(clip_id.into())
            .add_stage(stage)
            .add_stage(clip_id.into())
            .set_direction(AnimationDirection::Backwards)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Loop);
    });

    ctx.add_animation_to_sprite(animation_id);

    // Stage 3 (played backwards)

    ctx.run(50);
    ctx.check(2, []);

    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);
    ctx.check(0, []);

    // Stage 2 (played backwards but was backwards so now forwards!)

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

    // stage 1 (played backwards)

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
fn stage_pingpong() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1, 2])
            .set_default_direction(AnimationDirection::Backwards); // should be ignored
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage
            .set_direction(AnimationDirection::PingPong)
            .set_repeat(3); // Needed for the ping-pong or we would only get pongs

        animation
            .add_stage(stage)
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repeat(AnimationRepeat::Loop);
    });

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

// #[test]
// fn animation_pingpong() {
//     let mut ctx = Context::new();

//     let clip_id = ctx.library().new_clip(|clip| {
//         clip.push_frame_indices([0, 1, 2]);
//     });

//     let animation_id = ctx.library().new_animation(|animation| {
//         animation
//             .add_stage(clip_id.into())
//             .set_direction(AnimationDirection::PingPong)
//             .set_duration(AnimationDuration::PerFrame(100))
//             .set_repeat(AnimationRepeat::Loop);
//     });

//     ctx.add_animation_to_sprite(animation_id);

//     // Ping

//     ctx.run(50);
//     ctx.check(0, []);

//     ctx.run(100);
//     ctx.check(1, []);

//     ctx.run(100);
//     ctx.check(2, []);

//     // Pong

//     ctx.run(100);
//     ctx.check(1, [ctx.clip_cycle_end(0, animation_id)]);

//     ctx.run(100);
//     ctx.check(0, []);

//     // Ping again

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
// }

// #[test]
// fn animation_pingpong_stage_pingpong() {
//     let mut ctx = Context::new();

//     let clip_id = ctx.library().new_clip(|clip| {
//         clip.push_frame_indices([0, 1, 2])
//             .set_default_direction(AnimationDirection::PingPong)
//             .set_default_repeat(2); // Needed for the ping-pong or we would only get pongs
//     });

//     let animation_id = ctx.library().new_animation(|animation| {
//         animation
//             .add_stage(clip_id.into())
//             .set_direction(AnimationDirection::PingPong)
//             .set_duration(AnimationDuration::PerFrame(100))
//             .set_repeat(AnimationRepeat::Loop);
//     });

//     ctx.add_animation_to_sprite(animation_id);

//     panic!();

//     // Ping

//     ctx.run(50);
//     ctx.check(0, []);

//     ctx.run(100);
//     ctx.check(1, []);

//     ctx.run(100);
//     ctx.check(2, []);

//     // Pong

//     ctx.run(100);
//     ctx.check(1, [ctx.clip_cycle_end(0, animation_id)]);

//     ctx.run(100);
//     ctx.check(0, []);

//     // Ping again

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
// }

// #[test]
// fn animation_pingpong_stage_backwards() {
//     let mut ctx = Context::new();

//     let clip_id = ctx.library().new_clip(|clip| {
//         clip.push_frame_indices([0, 1, 2])
//             .set_default_direction(AnimationDirection::Backwards);
//     });

//     let animation_id = ctx.library().new_animation(|animation| {
//         animation
//             .add_stage(clip_id.into())
//             .set_direction(AnimationDirection::PingPong)
//             .set_duration(AnimationDuration::PerFrame(100))
//             .set_repeat(AnimationRepeat::Loop);
//     });

//     ctx.add_animation_to_sprite(animation_id);

//     panic!();

//     // Ping

//     ctx.run(50);
//     ctx.check(0, []);

//     ctx.run(100);
//     ctx.check(1, []);

//     ctx.run(100);
//     ctx.check(2, []);

//     // Pong

//     ctx.run(100);
//     ctx.check(1, [ctx.clip_cycle_end(0, animation_id)]);

//     ctx.run(100);
//     ctx.check(0, []);

//     // Ping again

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
// }

// #[test]
// fn animation_backwards_stage_pingpong() {
//     let mut ctx = Context::new();

//     let clip_id = ctx.library().new_clip(|clip| {
//         clip.push_frame_indices([0, 1, 2])
//             .set_default_direction(AnimationDirection::PingPong)
//             .set_default_repeat(2); // Needed for the ping-pong or we would only get pongs
//     });

//     let animation_id = ctx.library().new_animation(|animation| {
//         animation
//             .add_stage(clip_id.into())
//             .set_direction(AnimationDirection::Backwards)
//             .set_duration(AnimationDuration::PerFrame(100))
//             .set_repeat(AnimationRepeat::Loop);
//     });

//     ctx.add_animation_to_sprite(animation_id);

//     panic!();

//     // Ping

//     ctx.run(50);
//     ctx.check(0, []);

//     ctx.run(100);
//     ctx.check(1, []);

//     ctx.run(100);
//     ctx.check(2, []);

//     // Pong

//     ctx.run(100);
//     ctx.check(1, [ctx.clip_cycle_end(0, animation_id)]);

//     ctx.run(100);
//     ctx.check(0, []);

//     // Ping again

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
// }
