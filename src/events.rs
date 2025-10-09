use std::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

use bevy::prelude::*;

use crate::{animation::Animation, clip::ClipId};

/// A Bevy event emitted when an animation reaches a point of interest:
/// - When a clip repetition ends
/// - When a clip ends (if the clip repeats multiple times, this only occurs at the end of the last repetition)
/// - When an animation repetition ends
/// - When an animation ends (if the animation repeats multiple times, this only occurs at the end of the last repetition)
/// - When an [animation marker](Marker) is hit
///
/// # Example
///
/// You can use those events to be notified of a clip/animation ending.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// #[derive(Resource)]
/// struct MyDeathAnimation(Handle<Animation>);
///
/// fn create_death_animation(
///     mut commands: Commands,
///     # animation_handle: Handle<Animation>
/// ) {
///     // ... omitted: create an animation
///
///     // To use this animation from another system, you might want to keep it around.
///     // For example, you could store it in a resource.
///
///     commands.insert_resource(MyDeathAnimation(animation_handle));
/// }
///
/// fn explode_on_death(
///     mut messages: MessageReader<AnimationEvent>,
///     my_death_animation: Res<MyDeathAnimation>,
/// ) {
///     for message in messages.read() {
///         match message {
///             // Some animation just ended and it was the main character's death animation
///             AnimationEvent::AnimationEnd { animation, .. } if *animation == my_death_animation.0 => {
///                 # fn explode() {}
///                 explode();
///             }
///
///             // Ignore other events
///             _ => (),
///         }
///     }
/// }
/// ```
///
/// # Example
///
/// You can also add markers to specific frames of a clip to be notified of an animation reaching points of interest.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// #[derive(Resource)]
/// struct ShootMarker(Marker);
///
/// fn create_animated_sprite(
///     mut commands: Commands,
///     assets: Res<AssetServer>,
/// ) {
///     let image = assets.load("character.png");
///
///     let spritesheet = Spritesheet::new(&image, 8, 4);
///
///     // Let's create a marker to be notified when the exact frame (5) where the character shoots their gun is played
///
///     let bullet_goes_out_marker = Marker::new();
///
///     let animation = spritesheet
///         .create_animation()
///         .add_row(2)
///         .add_clip_marker(bullet_goes_out_marker, 5)
///         .build();
///
///     // To use this marker from another system, you might want to keep it around.
///     // For example, you could store it in a resource.
///
///     commands.insert_resource(ShootMarker(bullet_goes_out_marker));
///
///     // ...
/// }
///
/// #[derive(Resource)]
/// struct MyMarker(Marker);
///
/// // We can watch events from any system and react to them
/// fn spawn_bullets(
///     mut messages: MessageReader<AnimationEvent>,
///     shoot_marker: Res<ShootMarker>,
///) {
///     for message in messages.read() {
///         match message {
///             // Some marker was just hit and it was our "bullet goes out" marker
///             AnimationEvent::MarkerHit { marker, .. } if *marker == shoot_marker.0 => {
///                 // ... omitted: spawn a bullet entity
///             }
///
///             // Ignore other events
///             _ => (),
///         }
///     }
/// }
/// ```
#[derive(Message, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnimationEvent {
    /// An animation marker has been hit
    MarkerHit {
        entity: Entity,
        marker: Marker,
        clip_id: ClipId,
        clip_repetition: usize,
        animation: Handle<Animation>,
        animation_repetition: usize,
    },
    /// A repetition of a clip has ended
    ClipRepetitionEnd {
        entity: Entity,
        clip_id: ClipId,
        clip_repetition: usize,
        animation: Handle<Animation>,
    },
    /// A clip has ended
    ClipEnd {
        entity: Entity,
        clip_id: ClipId,
        animation: Handle<Animation>,
    },
    /// A repetition of an animation has ended
    AnimationRepetitionEnd {
        entity: Entity,
        animation: Handle<Animation>,
        animation_repetition: usize,
    },
    /// An animation has ended
    AnimationEnd {
        entity: Entity,
        animation: Handle<Animation>,
    },
}

/// A marker that designates a point of interest in an animation.
///
/// [MarkerHit](AnimationEvent::MarkerHit) events containing this marker are emitted when the corresponding frame is played.
///
/// Add markers to a clip with [AnimationBuilder::add_clip_marker()](crate::prelude::AnimationBuilder::add_clip_marker).
#[derive(Clone, Copy, Eq, PartialEq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub struct Marker {
    pub(crate) value: usize,
}

static NEXT_MARKER: AtomicUsize = AtomicUsize::new(0);

impl Marker {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Marker {
            value: NEXT_MARKER.fetch_add(1, Ordering::Relaxed),
        }
    }
}

impl fmt::Debug for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "marker{}", self.value)
    }
}
