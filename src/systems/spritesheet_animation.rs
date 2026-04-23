use bevy::prelude::*;

use crate::{
    animation::Animation,
    animator::{Animator, SpritesheetAnimationQuery},
    components::spritesheet_animation::SpritesheetAnimation,
    events::AnimationEvent,
};

pub fn play_animations(
    time: Res<Time>,
    mut animator: ResMut<Animator>,
    mut message_writer: MessageWriter<AnimationEvent>,
    mut query: Query<SpritesheetAnimationQuery>,
    mut animations: ResMut<Assets<Animation>>,
) {
    animator.update(&time, &mut message_writer, &mut query, &mut animations);
}

pub fn sync_animation_changes(
    mut animator: ResMut<Animator>,
    mut message_writer: MessageWriter<AnimationEvent>,
    mut query: Query<SpritesheetAnimationQuery, Changed<SpritesheetAnimation>>,
    mut animations: ResMut<Assets<Animation>>,
) {
    animator.sync_changes(&mut message_writer, &mut query, &mut animations);
}
