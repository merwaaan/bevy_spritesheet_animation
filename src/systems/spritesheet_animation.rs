use bevy::{
    ecs::{
        event::EventWriter,
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
    mut event_writer: EventWriter<AnimationEvent>,
    mut query: Query<SpritesheetAnimationQuery>,
) {
    animator.update(&time, &library, &mut event_writer, &mut query);
}
