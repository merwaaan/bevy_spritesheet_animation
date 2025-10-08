use bevy::{
    ecs::{
        message::MessageWriter,
        system::{Query, Res, ResMut},
    },
    time::Time,
};

use crate::{
    animator::{Animator, SpritesheetAnimationQuery},
    events::AnimationEvent,
    library::AnimationLibrary,
};

pub fn play_animations(
    time: Res<Time>,
    library: Res<AnimationLibrary>,
    mut animator: ResMut<Animator>,
    mut message_writer: MessageWriter<AnimationEvent>,
    mut query: Query<SpritesheetAnimationQuery>,
) {
    animator.update(&time, &library, &mut message_writer, &mut query);
}
