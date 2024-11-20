use bevy::{
    ecs::{
        entity::Entity,
        event::EventWriter,
        system::{Query, Res, ResMut},
    },
    sprite::TextureAtlas,
    time::Time,
};

use crate::{
    animator::Animator, components::spritesheet_animation::SpritesheetAnimation,
    events::AnimationEvent, library::AnimationLibrary,
};

pub fn play_animations(
    time: Res<Time>,
    library: Res<AnimationLibrary>,
    mut animator: ResMut<Animator>,
    mut event_writer: EventWriter<AnimationEvent>,
    mut query: Query<(Entity, &mut SpritesheetAnimation, Option<&mut TextureAtlas>)>,
) {
    animator.update(&time, &library, &mut event_writer, &mut query);
}
