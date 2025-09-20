use bevy::{
    ecs::{
        message::MessageWriter,
        system::{Query, Res, ResMut},
    },
    time::Time,
};

use crate::{
    animator::{Animator, SpritesheetAnimationQuery},
    library::AnimationLibrary,
    messages::AnimationMessage,
};

pub fn play_animations(
    time: Res<Time>,
    library: Res<AnimationLibrary>,
    mut animator: ResMut<Animator>,
    mut message_writer: MessageWriter<AnimationMessage>,
    mut query: Query<SpritesheetAnimationQuery>,
) {
    animator.update(&time, &library, &mut message_writer, &mut query);
}
