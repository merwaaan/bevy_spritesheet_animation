use bevy::prelude::*;

use crate::{
    animation::Animation,
    animator::{Animator, SpritesheetAnimationQuery},
    events::AnimationEvent,
};

pub fn animate(
    time: Res<Time>,
    mut animator: ResMut<Animator>,
    mut message_writer: MessageWriter<AnimationEvent>,
    mut query: Query<SpritesheetAnimationQuery>,
    mut animations: ResMut<Assets<Animation>>,
) {
    animator.animate(&time, &mut message_writer, &mut query, &mut animations);
}

pub fn sync(
    mut animator: ResMut<Animator>,
    mut message_writer: MessageWriter<AnimationEvent>,
    mut query: Query<SpritesheetAnimationQuery>,
    mut animations: ResMut<Assets<Animation>>,
) {
    animator.sync(&mut message_writer, &mut query, &mut animations);
}
