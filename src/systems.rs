use bevy::{
    ecs::{
        entity::Entity,
        event::EventWriter,
        system::{Query, Res, ResMut},
    },
    sprite::TextureAtlas,
    time::Time,
};

#[cfg(feature = "integration-tests")]
use bevy::time::Real;

// In unit tests, we use a TimeUpdateStrategy to control how time advances.
//
// However, it currently doesn't seem to work with Time, only with Time<Real>.
// So we use different time types in release and test builds for now.
//
// https://github.com/bevyengine/bevy/issues/11127
#[cfg(feature = "integration-tests")]
pub(crate) type ActualTime = Time<Real>;
#[cfg(not(feature = "integration-tests"))]
pub(crate) type ActualTime = Time;

use crate::{
    animator::SpritesheetAnimator, component::SpritesheetAnimation, events::AnimationEvent,
    library::SpritesheetLibrary,
};

pub(crate) fn play_animations(
    time: Res<ActualTime>,
    library: Res<SpritesheetLibrary>,
    mut animator: ResMut<SpritesheetAnimator>,
    mut events: EventWriter<AnimationEvent>,
    mut query: Query<(Entity, &SpritesheetAnimation, &mut TextureAtlas)>,
) {
    animator.update(&time, &library, &mut events, &mut query);
}
