pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn clip_timing_per_frame() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([4, 5, 6])
            .set_default_duration(AnimationDuration::PerFrame(120));
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation.add_stage(clip_id.into());
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(100);
    ctx.check(4, &[]);

    ctx.run(100);
    ctx.check(5, &[]);

    ctx.run(100);
    ctx.check(6, &[]);
}

#[test]
fn clip_timing_per_cycle() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([4, 5, 6])
            .set_default_duration(AnimationDuration::PerCycle(330));
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation.add_stage(clip_id.into());
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(100);
    ctx.check(4, &[]);

    ctx.run(100);
    ctx.check(5, &[]);

    ctx.run(100);
    ctx.check(6, &[]);
}

#[test]
fn stage_timing_per_frame() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([5, 1, 7])
            .set_default_duration(AnimationDuration::PerFrame(99999999)); // should be ignored
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage.set_duration(AnimationDuration::PerFrame(1000));

        animation.add_stage(stage);
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(400);
    ctx.check(5, &[]);

    ctx.run(400); // 800
    ctx.check(5, &[]);

    ctx.run(400); // 1200
    ctx.check(1, &[]);

    ctx.run(1000); // 2200
    ctx.check(7, &[]);
}

#[test]
fn stage_timing_per_cycle() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([4, 5, 6])
            .set_default_duration(AnimationDuration::PerFrame(123456789)); // should be ignored
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage = AnimationStage::from_clip(clip_id);
        stage.set_duration(AnimationDuration::PerCycle(3000));

        animation.add_stage(stage);
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(500);
    ctx.check(4, &[]);

    ctx.run(1000); // 1.5
    ctx.check(5, &[]);

    ctx.run(1000); // 1.5
    ctx.check(6, &[]);
}

#[test]
fn animation_timing_per_frame() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1])
            .set_default_duration(AnimationDuration::PerFrame(99999999)); // should be ignored
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage1 = AnimationStage::from_clip(clip_id);
        stage1.set_duration(AnimationDuration::PerFrame(1)); // should be ignored

        let mut stage2 = AnimationStage::from_clip(clip_id);
        stage2
            .set_duration(AnimationDuration::PerFrame(123456)) // should be ignored
            .set_repeat(2);

        animation
            .add_stage(stage1)
            .add_stage(stage2)
            .set_duration(AnimationDuration::PerFrame(500));
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.run(400);
    ctx.check(0, &[]);

    ctx.run(400); // 800
    ctx.check(1, &[]);

    ctx.run(400); // 1200
    ctx.check(
        0,
        &[
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
        ],
    );

    ctx.run(400); // 1600
    ctx.check(1, &[]);

    ctx.run(600); // 2200
    ctx.check(0, &[ctx.clip_cycle_end(1, animation_id)]);

    ctx.run(400); // 2600
    ctx.check(1, &[]);
}

#[test]
fn animation_timing_per_cycle() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([0, 1])
            .set_default_duration(AnimationDuration::PerFrame(999999)); // should be ignored
    });

    let animation_id = ctx.library().new_animation(|animation| {
        let mut stage1 = AnimationStage::from_clip(clip_id);
        stage1.set_duration(AnimationDuration::PerCycle(1000));

        let mut stage2 = AnimationStage::from_clip(clip_id);
        stage2
            .set_duration(AnimationDuration::PerFrame(2000))
            .set_repeat(2);

        animation
            .add_stage(stage1)
            .add_stage(stage2)
            .set_duration(AnimationDuration::PerCycle(10_000));
    });

    ctx.add_animation_to_sprite(animation_id);

    // Animation duration = 10 000 per cycle
    //
    // Stage 1 duration = 1000 per cycle
    // Stage 2 duration = 2000 per frame * 2 repetitions = 8000 per cycle
    //
    // So stage 1 takes 1000/9000th of the animation time = 1111
    // And stage 2 takes 8000/9000th = 8888

    // stage 0, frame 0: 0 to 555

    ctx.run(200);
    ctx.check(0, &[]);

    ctx.run(350); // 550
    ctx.check(0, &[]);

    // stage 0, frame 1: 555 to 1111

    ctx.run(10); // 560
    ctx.check(1, &[]);

    ctx.run(540); // 1100
    ctx.check(1, &[]);

    // stage 1, frame 0: 1111 to 3333

    ctx.run(20); // 1120
    ctx.check(
        0,
        &[
            ctx.clip_cycle_end(0, animation_id),
            ctx.clip_end(0, animation_id),
        ],
    );

    ctx.run(2200); // 3320
    ctx.check(0, &[]);

    // stage 1, frame 1: 3333 to 5555

    ctx.run(20); // 3340
    ctx.check(1, &[]);

    ctx.run(2210); // 5550
    ctx.check(1, &[]);

    // stage 1, frame 0 (repeated): 5555 to 7777

    ctx.run(20); // 5570
    ctx.check(0, &[ctx.clip_cycle_end(1, animation_id)]);

    ctx.run(2200); // 7770
    ctx.check(0, &[]);

    // stage 1, frame 1: 7777 to 9999

    ctx.run(10); // 7780
    ctx.check(1, &[]);

    ctx.run(2210); // 9990
    ctx.check(1, &[]);

    // wrap

    ctx.run(20); // 10010
    ctx.check(
        0,
        &[
            ctx.clip_cycle_end(1, animation_id),
            ctx.clip_end(1, animation_id),
            ctx.anim_cycle_end(animation_id),
        ],
    );
}

#[test]
fn pause_resume() {
    let mut ctx = Context::new();

    let clip_id = ctx.library().new_clip(|clip| {
        clip.push_frame_indices([1, 2, 3, 4])
            .set_default_duration(AnimationDuration::PerFrame(110));
    });

    let animation_id = ctx.library().new_animation(|animation| {
        animation.add_stage(clip_id.into());
    });

    ctx.add_animation_to_sprite(animation_id);

    ctx.update_sprite_animation(|anim| {
        anim.playing = false;
    });

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(1, &[]);
    }

    ctx.update_sprite_animation(|anim| {
        anim.playing = true;
    });

    ctx.run(100);
    ctx.check(1, &[]);

    ctx.run(100); // 200
    ctx.check(2, &[]);

    ctx.update_sprite_animation(|anim| {
        anim.playing = false;
    });

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(2, &[]);
    }

    ctx.update_sprite_animation(|anim| {
        anim.playing = true;
    });

    ctx.run(100); // 300
    ctx.check(3, &[]);

    ctx.run(100); // 400
    ctx.check(4, &[]);
}
