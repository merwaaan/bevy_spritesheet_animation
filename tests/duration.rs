pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn clip_duration_per_frame() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([5, 1, 7]).with_duration(AnimationDuration::PerFrame(1000));

    ctx.attach_animation(Animation::from_clip(clip));

    ctx.run(400);
    ctx.check(5, []);

    ctx.run(400); // 800
    ctx.check(5, []);

    ctx.run(400); // 1200
    ctx.check(1, []);

    ctx.run(1000); // 2200
    ctx.check(7, []);
}

#[test]
fn clip_duration_per_cycle() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([4, 5, 6]).with_duration(AnimationDuration::PerRepetition(3000));

    ctx.attach_animation(Animation::from_clip(clip));

    ctx.run(500);
    ctx.check(4, []);

    ctx.run(1000); // 1.5
    ctx.check(5, []);

    ctx.run(1000); // 1.5
    ctx.check(6, []);
}

#[test]
fn clip_with_zero_duration() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([4, 5, 6]).with_duration(AnimationDuration::PerFrame(0));

    ctx.attach_animation(Animation::from_clip(clip));

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, []);
    }
}

#[test]
fn animation_duration_per_frame() {
    let mut ctx = Context::new();

    let clip1 = Clip::from_frames([0, 1]).with_duration(AnimationDuration::PerFrame(123456)); // should be ignored

    let clip2 = Clip::from_frames([0, 1])
        .with_duration(AnimationDuration::PerFrame(9999999)) // should be ignored
        .with_repetitions(2);

    let animation = ctx.attach_animation(
        Animation::from_clips([clip1.clone(), clip2.clone()])
            .with_duration(AnimationDuration::PerFrame(500)),
    );

    ctx.run(400);
    ctx.check(0, []);

    ctx.run(400); // 800
    ctx.check(1, []);

    ctx.run(400); // 1200
    ctx.check(
        0,
        [
            ctx.clip_rep_end(&animation, &clip1, 0),
            ctx.clip_end(&animation, &clip1),
        ],
    );

    ctx.run(400); // 1600
    ctx.check(1, []);

    ctx.run(600); // 2200
    ctx.check(0, [ctx.clip_rep_end(&animation, &clip2, 0)]);

    ctx.run(400); // 2600
    ctx.check(1, []);
}

#[test]
fn animation_duration_per_cycle() {
    let mut ctx = Context::new();

    let clip1 = Clip::from_frames([0, 1]).with_duration(AnimationDuration::PerRepetition(1000));

    let clip2 = Clip::from_frames([0, 1])
        .with_duration(AnimationDuration::PerFrame(2000))
        .with_repetitions(2);

    let animation = ctx.attach_animation(
        Animation::from_clips([clip1.clone(), clip2.clone()])
            .with_duration(AnimationDuration::PerRepetition(10_000)),
    );

    // Animation duration = 10 000 per cycle
    //
    // Clip 1 duration = 1000 per cycle
    // Clip 2 duration = 2000 per frame * 2 repetitions = 8000 per cycle
    //
    // So clip 1 takes 1000/9000th of the animation time = 1111
    // And clip 2 takes 8000/9000th = 8888

    // clip 1, frame 0: 0 to 555

    ctx.run(200);
    ctx.check(0, []);

    ctx.run(350); // 550
    ctx.check(0, []);

    // clip 1, frame 1: 555 to 1111

    ctx.run(10); // 560
    ctx.check(1, []);

    ctx.run(540); // 1100
    ctx.check(1, []);

    // clip 2, frame 0: 1111 to 3333

    ctx.run(20); // 1120
    ctx.check(
        0,
        [
            ctx.clip_rep_end(&animation, &clip1, 0),
            ctx.clip_end(&animation, &clip1),
        ],
    );

    ctx.run(2200); // 3320
    ctx.check(0, []);

    // clip 2, frame 1: 3333 to 5555

    ctx.run(20); // 3340
    ctx.check(1, []);

    ctx.run(2210); // 5550
    ctx.check(1, []);

    // clip 2, frame 0 (repeated): 5555 to 7777

    ctx.run(20); // 5570
    ctx.check(0, [ctx.clip_rep_end(&animation, &clip2, 0)]);

    ctx.run(2200); // 7770
    ctx.check(0, []);

    // clip 2, frame 1: 7777 to 9999

    ctx.run(10); // 7780
    ctx.check(1, []);

    ctx.run(2210); // 9990
    ctx.check(1, []);

    // wrap

    ctx.run(20); // 10010
    ctx.check(
        0,
        [
            ctx.clip_rep_end(&animation, &clip2, 1),
            ctx.clip_end(&animation, &clip2),
            ctx.anim_rep_end(&animation, 0),
        ],
    );
}

#[test]
fn animation_with_zero_duration() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([4, 5, 6]);

    ctx.attach_animation(
        Animation::from_clip(clip).with_duration(AnimationDuration::PerRepetition(0)),
    );

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(0, []);
    }
}

#[test]
fn pause_resume() {
    let mut ctx = Context::new();

    let clip = Clip::from_frames([4, 5, 6, 7]).with_duration(AnimationDuration::PerFrame(100));

    let animation = ctx.attach_animation(Animation::from_clip(clip.clone()));

    ctx.run(50);
    ctx.check(4, []);

    ctx.run(50);
    ctx.check(5, []);

    ctx.run(150);
    ctx.check(6, []);

    ctx.run(100);
    ctx.check(7, []);

    // Pause

    ctx.get_sprite(|sprite| {
        sprite.playing = false;
    });

    for _ in 0..100 {
        ctx.run(100);
        ctx.check(7, []); // Stays on the same frame
    }

    // Resume

    ctx.get_sprite(|sprite| {
        sprite.playing = true;
    });

    // wrap

    ctx.run(100);
    ctx.check(
        4,
        [
            ctx.clip_rep_end(&animation, &clip, 0),
            ctx.clip_end(&animation, &clip),
            ctx.anim_rep_end(&animation, 0),
        ],
    );
}

#[test]
fn speed_factor() {
    let mut ctx = Context::new();

    let clip =
        Clip::from_frames([1, 2, 3, 4, 5, 6, 7]).with_duration(AnimationDuration::PerFrame(100));

    ctx.attach_animation(Animation::from_clip(clip));

    // x2

    ctx.get_sprite(|sprite| {
        sprite.speed_factor = 2.0;
    });

    ctx.run(60); // +60*2 = 120
    ctx.check(2, []);

    ctx.run(50); // +50*2 = 220
    ctx.check(3, []);

    // x0.1

    ctx.get_sprite(|sprite| {
        sprite.speed_factor = 0.1;
    });

    ctx.run(600); // +600*0.1 = 280
    ctx.check(3, []);

    ctx.run(1400); // +1400*0.1 = 420
    ctx.check(5, []);

    // x1

    ctx.get_sprite(|sprite| {
        sprite.speed_factor = 1.0;
    });

    ctx.run(100); // 520
    ctx.check(6, []);
}
