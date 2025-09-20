pub mod context;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn library_available_as_a_resource() {
    let ctx = Context::new();

    assert!(ctx.app.world().get_resource::<AnimationLibrary>().is_some());
}

#[test]
fn animation_messages_available_as_a_resource() {
    let ctx = Context::new();

    assert!(ctx
        .app
        .world()
        .get_resource::<Messages<AnimationMessage>>()
        .is_some());
}
