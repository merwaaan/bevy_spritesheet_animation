pub mod context;

use bevy_spritesheet_animation::library::LibraryError;
use context::*;

#[test]
fn clips() {
    let mut ctx = Context::new();

    // Create a first marker

    let clip1 = ctx.library().new_clip(|_| {});

    assert!(!ctx.library().is_clip_name(clip1, "first"));
    assert_eq!(ctx.library().clip_with_name("first"), None);

    // Name it

    assert!(ctx.library().name_clip(clip1, "first").is_ok());

    assert!(ctx.library().is_clip_name(clip1, "first"));
    assert_eq!(ctx.library().clip_with_name("first"), Some(clip1));

    // Give it a second name, this is not forbidden

    assert!(ctx.library().name_clip(clip1, "first again").is_ok());

    assert!(ctx.library().is_clip_name(clip1, "first"));
    assert_eq!(ctx.library().clip_with_name("first"), Some(clip1));

    assert!(ctx.library().is_clip_name(clip1, "first again"));
    assert_eq!(ctx.library().clip_with_name("first again"), Some(clip1));

    // Give it the same names again, this is a no-op

    assert!(ctx.library().name_clip(clip1, "first").is_ok());
    assert!(ctx.library().name_clip(clip1, "first again").is_ok());

    // Create another marker and reuse one of those names, this should not work

    let clip2 = ctx.library().new_clip(|_| {});

    assert!(matches!(
        ctx.library().name_clip(clip2, "first"),
        Err(LibraryError::NameAlreadyTaken)
    ));

    assert!(!ctx.library().is_clip_name(clip2, "first"));

    assert_eq!(ctx.library().clip_with_name("first"), Some(clip1));
    assert!(ctx.library().is_clip_name(clip1, "first"));
}

#[test]
fn animations() {
    let mut ctx = Context::new();

    // Create a first animation

    let anim1 = ctx.library().new_animation(|_| {});

    assert!(!ctx.library().is_animation_name(anim1, "first"));
    assert_eq!(ctx.library().animation_with_name("first"), None);

    // Name it

    assert!(ctx.library().name_animation(anim1, "first").is_ok());

    assert!(ctx.library().is_animation_name(anim1, "first"));
    assert_eq!(ctx.library().animation_with_name("first"), Some(anim1));

    // Give it a second name, this is not forbidden

    assert!(ctx.library().name_animation(anim1, "first again").is_ok());

    assert!(ctx.library().is_animation_name(anim1, "first"));
    assert_eq!(ctx.library().animation_with_name("first"), Some(anim1));

    assert!(ctx.library().is_animation_name(anim1, "first again"));
    assert_eq!(
        ctx.library().animation_with_name("first again"),
        Some(anim1)
    );

    // Give it the same names again, this is a no-op

    assert!(ctx.library().name_animation(anim1, "first").is_ok());
    assert!(ctx.library().name_animation(anim1, "first again").is_ok());

    // Create another animation and reuse one of those names, this should not work

    let anim2 = ctx.library().new_animation(|_| {});

    assert!(matches!(
        ctx.library().name_animation(anim2, "first"),
        Err(LibraryError::NameAlreadyTaken)
    ));

    assert!(!ctx.library().is_animation_name(anim2, "first"));

    assert_eq!(ctx.library().animation_with_name("first"), Some(anim1));
    assert!(ctx.library().is_animation_name(anim1, "first"));
}

#[test]
fn markers() {
    let mut ctx = Context::new();

    // Create a first marker

    let marker1 = ctx.library().new_marker();

    assert!(!ctx.library().is_marker_name(marker1, "first"));
    assert_eq!(ctx.library().marker_with_name("first"), None);

    // Name it

    assert!(ctx.library().name_marker(marker1, "first").is_ok());

    assert!(ctx.library().is_marker_name(marker1, "first"));
    assert_eq!(ctx.library().marker_with_name("first"), Some(marker1));

    // Give it a second name, this is not forbidden

    assert!(ctx.library().name_marker(marker1, "first again").is_ok());

    assert!(ctx.library().is_marker_name(marker1, "first"));
    assert_eq!(ctx.library().marker_with_name("first"), Some(marker1));

    assert!(ctx.library().is_marker_name(marker1, "first again"));
    assert_eq!(ctx.library().marker_with_name("first again"), Some(marker1));

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
}
