mod context;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use context::*;

#[derive(Resource)]
struct NextAnimation(Handle<Animation>);

fn switch_anim_system(
    mut commands: Commands,
    mut messages: MessageReader<AnimationEvent>,
    next_animation: Res<NextAnimation>,
) {
    for message in messages.read() {
        if let AnimationEvent::AnimationRepetitionEnd { entity, .. }
        | AnimationEvent::ClipRepetitionEnd { entity, .. } = message
        {
            commands
                .entity(*entity)
                .insert(SpritesheetAnimation::new(next_animation.0.clone()));
        }
    }
}

#[test]
fn switch_animation_on_repetition_end() {
    let mut ctx = Context::new();

    let anim1 = ctx.create_animation(|builder| {
        builder
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Loop)
            .add_indices([0, 1])
    });

    let anim2 = ctx.create_animation(|builder| {
        builder
            .set_duration(AnimationDuration::PerFrame(100))
            .set_repetitions(AnimationRepeat::Loop)
            .add_indices([2, 3])
    });

    ctx.app.insert_resource(NextAnimation(anim2.clone()));

    ctx.app.add_systems(Update, switch_anim_system);

    ctx.app
        .world_mut()
        .entity_mut(ctx.sprite_entity)
        .insert(SpritesheetAnimation::new(anim1.clone()));

    // 0ms
    ctx.run(50);
    ctx.check(0, []);

    // 100ms
    ctx.run(100);
    ctx.check(1, []);

    ctx.run(100);

    let index = ctx
        .app
        .world()
        .entity(ctx.sprite_entity)
        .get::<Sprite>()
        .and_then(|sprite| sprite.texture_atlas.as_ref())
        .unwrap()
        .index;

    assert_eq!(
        index, 2,
        "Expected index 2 (start of anim2), but found index {}. The animation momentarily reverted to the start of the old animation (anim1) before switching.",
        index
    );
}
