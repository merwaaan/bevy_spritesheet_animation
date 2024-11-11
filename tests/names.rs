pub mod context;

use bevy_spritesheet_animation::prelude::*;
use context::*;

#[test]
fn clips() {
    let mut ctx = Context::new();

    // Create a first clip

    let clip1 = Clip::from_frames([]);
    let clip1_id = ctx.library().register_clip(clip1);

    assert!(!ctx.library().is_clip_name(clip1_id, "first"));
    assert_eq!(ctx.library().clip_with_name("first"), None);
    assert_eq!(ctx.library().clip_names().len(), 0);

    // Name it

    assert!(ctx.library().name_clip(clip1_id, "first").is_ok());

    assert!(ctx.library().is_clip_name(clip1_id, "first"));
    assert_eq!(ctx.library().clip_with_name("first"), Some(clip1_id));

    assert_eq!(ctx.library().clip_names().len(), 1);
    assert_eq!(
        ctx.library().clip_names().get("first").copied(),
        Some(clip1_id)
    );

    // Give it a second name, this is not forbidden

    assert!(ctx.library().name_clip(clip1_id, "first again").is_ok());

    assert!(ctx.library().is_clip_name(clip1_id, "first"));
    assert_eq!(ctx.library().clip_with_name("first"), Some(clip1_id));

    assert!(ctx.library().is_clip_name(clip1_id, "first again"));
    assert_eq!(ctx.library().clip_with_name("first again"), Some(clip1_id));

    assert_eq!(ctx.library().clip_names().len(), 2);
    assert_eq!(
        ctx.library().clip_names().get("first").copied(),
        Some(clip1_id)
    );
    assert_eq!(
        ctx.library().clip_names().get("first again").copied(),
        Some(clip1_id)
    );

    // Give it the same names again, this is a no-op

    assert!(ctx.library().name_clip(clip1_id, "first").is_ok());
    assert!(ctx.library().name_clip(clip1_id, "first again").is_ok());

    // Create another marker and reuse one of those names, this should not work

    let clip2 = Clip::from_frames([]);
    let clip2_id = ctx.library().register_clip(clip2);

    assert!(matches!(
        ctx.library().name_clip(clip2_id, "first"),
        Err(LibraryError::NameAlreadyTaken)
    ));

    assert!(!ctx.library().is_clip_name(clip2_id, "first"));

    assert_eq!(ctx.library().clip_with_name("first"), Some(clip1_id));
    assert!(ctx.library().is_clip_name(clip1_id, "first"));

    assert_eq!(ctx.library().clip_names().len(), 2);
}

#[test]
fn animations() {
    let mut ctx = Context::new();

    // Create a first animation

    let animation1 = Animation::from_clips([]);
    let animation1_id = ctx.library().register_animation(animation1);

    assert!(!ctx.library().is_animation_name(animation1_id, "first"));
    assert_eq!(ctx.library().animation_with_name("first"), None);
    assert_eq!(ctx.library().animation_names().len(), 0);

    // Name it

    assert!(ctx.library().name_animation(animation1_id, "first").is_ok());

    assert!(ctx.library().is_animation_name(animation1_id, "first"));
    assert_eq!(
        ctx.library().animation_with_name("first"),
        Some(animation1_id)
    );

    assert_eq!(ctx.library().animation_names().len(), 1);
    assert_eq!(
        ctx.library().animation_names().get("first").copied(),
        Some(animation1_id)
    );

    // Give it a second name, this is not forbidden

    assert!(ctx
        .library()
        .name_animation(animation1_id, "first again")
        .is_ok());

    assert!(ctx.library().is_animation_name(animation1_id, "first"));
    assert_eq!(
        ctx.library().animation_with_name("first"),
        Some(animation1_id)
    );

    assert!(ctx
        .library()
        .is_animation_name(animation1_id, "first again"));
    assert_eq!(
        ctx.library().animation_with_name("first again"),
        Some(animation1_id)
    );

    assert_eq!(ctx.library().animation_names().len(), 2);
    assert_eq!(
        ctx.library().animation_names().get("first").copied(),
        Some(animation1_id)
    );
    assert_eq!(
        ctx.library().animation_names().get("first again").copied(),
        Some(animation1_id)
    );

    // Give it the same names again, this is a no-op

    assert!(ctx.library().name_animation(animation1_id, "first").is_ok());
    assert!(ctx
        .library()
        .name_animation(animation1_id, "first again")
        .is_ok());

    // Create another animation and reuse one of those names, this should not work

    let anim2 = Animation::from_clips([]);
    let anim2_id = ctx.library().register_animation(anim2);

    assert!(matches!(
        ctx.library().name_animation(anim2_id, "first"),
        Err(LibraryError::NameAlreadyTaken)
    ));

    assert!(!ctx.library().is_animation_name(anim2_id, "first"));

    assert_eq!(
        ctx.library().animation_with_name("first"),
        Some(animation1_id)
    );
    assert!(ctx.library().is_animation_name(animation1_id, "first"));

    assert_eq!(ctx.library().animation_names().len(), 2);
}

#[test]
fn markers() {
    let mut ctx = Context::new();

    // Create a first marker

    let marker1 = ctx.library().new_marker();

    assert!(!ctx.library().is_marker_name(marker1, "first"));
    assert_eq!(ctx.library().marker_with_name("first"), None);
    assert_eq!(ctx.library().marker_names().len(), 0);

    // Name it

    assert!(ctx.library().name_marker(marker1, "first").is_ok());

    assert!(ctx.library().is_marker_name(marker1, "first"));
    assert_eq!(ctx.library().marker_with_name("first"), Some(marker1));

    assert_eq!(ctx.library().marker_names().len(), 1);
    assert_eq!(
        ctx.library().marker_names().get("first").copied(),
        Some(marker1)
    );

    // Give it a second name, this is not forbidden

    assert!(ctx.library().name_marker(marker1, "first again").is_ok());

    assert!(ctx.library().is_marker_name(marker1, "first"));
    assert_eq!(ctx.library().marker_with_name("first"), Some(marker1));

    assert!(ctx.library().is_marker_name(marker1, "first again"));
    assert_eq!(ctx.library().marker_with_name("first again"), Some(marker1));

    assert_eq!(ctx.library().marker_names().len(), 2);
    assert_eq!(
        ctx.library().marker_names().get("first").copied(),
        Some(marker1)
    );
    assert_eq!(
        ctx.library().marker_names().get("first again").copied(),
        Some(marker1)
    );

    // Give it the same names again, this is a no-op

    assert!(ctx.library().name_marker(marker1, "first").is_ok());
    assert!(ctx.library().name_marker(marker1, "first again").is_ok());

    // Create another marker and reuse one of those names, this should not work

    let marker2 = ctx.library().new_marker();

    assert!(matches!(
        ctx.library().name_marker(marker2, "first"),
        Err(LibraryError::NameAlreadyTaken)
    ));

    assert!(!ctx.library().is_marker_name(marker2, "first"));

    assert_eq!(ctx.library().marker_with_name("first"), Some(marker1));
    assert!(ctx.library().is_marker_name(marker1, "first"));

    assert_eq!(ctx.library().marker_names().len(), 2);
}
