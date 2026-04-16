use bevy::prelude::*;

use crate::{
    animation::Animation,
    animator::{Animator, SpritesheetAnimationQuery},
    events::AnimationEventWriters,
};

pub fn play_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut animator: ResMut<Animator>,
    mut animation_event_writers: AnimationEventWriters,
    mut query: Query<SpritesheetAnimationQuery>,
    mut animations: ResMut<Assets<Animation>>,
) {
    animator.update(
        &mut commands,
        &time,
        &mut animation_event_writers,
        &mut query,
        &mut animations,
    );
}
