pub mod context;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn animation_assets_available_as_a_resource() {
    assert!(
        Context::new()
            .app
            .world()
            .contains_resource::<Assets<Animation>>()
    );
}

#[test]
fn animation_events_available_as_a_resource() {
    let ctx = Context::new();

    assert!(
        ctx.app
            .world()
            .get_resource::<Messages<AnimationEvent>>()
            .is_some()
    );
}
