use bevy::{
    ecs::{
        entity::Entity,
        event::EventWriter,
        system::{Query, Res, ResMut},
    },
    sprite::Sprite,
    time::Time,
    ui::widget::ImageNode,
};

use crate::{
    animator::Animator,
    components::{sprite3d::Sprite3d, spritesheet_animation::SpritesheetAnimation},
    events::AnimationEvent,
    library::AnimationLibrary,
};

pub fn play_animations(
    time: Res<Time>,
    library: Res<AnimationLibrary>,
    mut animator: ResMut<Animator>,
    mut event_writer: EventWriter<AnimationEvent>,
    mut query: Query<(
        Entity,
        &mut SpritesheetAnimation,
        Option<&mut Sprite>,
        Option<&mut Sprite3d>,
        Option<&mut ImageNode>,
    )>,
) {
    animator.update(&time, &library, &mut event_writer, &mut query);
}
