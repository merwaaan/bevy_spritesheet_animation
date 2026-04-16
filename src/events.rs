#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

use bevy::{ecs::system::SystemParam, prelude::*};

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
///     // ... omitted: create an animated sprite
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
///     // ... omitted: create an animated sprite
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

/// An animation marker has been hit
#[derive(EntityEvent, Message, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarkerHit {
    pub entity: Entity,
    pub marker: Marker,
    pub clip_id: ClipId,
    pub clip_repetition: usize,
    pub animation: Handle<Animation>,
    pub animation_repetition: usize,
}

/// A repetition of a clip has ended
#[derive(EntityEvent, Message, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClipRepetitionEnd {
    pub entity: Entity,
    pub clip_id: ClipId,
    pub clip_repetition: usize,
    pub animation: Handle<Animation>,
    // TODO add animation_repetition
}

/// A clip has ended
#[derive(EntityEvent, Message, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClipEnd {
    pub entity: Entity,
    pub clip_id: ClipId,
    pub animation: Handle<Animation>,
    // TODO add animation_repetition
}

/// A repetition of an animation has ended
#[derive(EntityEvent, Message, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimationRepetitionEnd {
    pub entity: Entity,
    pub animation: Handle<Animation>,
    pub animation_repetition: usize,
}

/// An animation has ended
#[derive(EntityEvent, Message, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimationEnd {
    pub entity: Entity,
    pub animation: Handle<Animation>,
}

#[derive(SystemParam)]
pub struct AnimationEventWriters<'w> {
    pub marker_hit_writer: MessageWriter<'w, MarkerHit>,
    pub clip_repitition_end_writer: MessageWriter<'w, ClipRepetitionEnd>,
    pub clip_end_writer: MessageWriter<'w, ClipEnd>,
    pub animation_repitition_end_writer: MessageWriter<'w, AnimationRepetitionEnd>,
    pub animation_end_writer: MessageWriter<'w, AnimationEnd>,
}

/// A marker that designates a point of interest in an animation.
///
/// [MarkerHit](AnimationEvent::MarkerHit) events containing this marker are emitted when the corresponding frame is played.
///
/// Add markers to a clip with [AnimationBuilder::add_clip_marker()](crate::prelude::AnimationBuilder::add_clip_marker).
#[derive(Clone, Copy, Eq, PartialEq, Hash, Reflect)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
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

pub struct AnimationEventsPlugin;

impl Plugin for AnimationEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MarkerHit>()
            .add_message::<ClipRepetitionEnd>()
            .add_message::<ClipEnd>()
            .add_message::<AnimationRepetitionEnd>()
            .add_message::<AnimationEnd>();
    }
}
