use bevy::{
    asset::Handle,
    ecs::{entity::Entity, message::Message},
};

use crate::{
    animation::Animation,
    clip::{ClipId, MarkerId},
};

// TODO rename message?

/// A Bevy event emitted when an animation reaches a point of interest
///
/// * when a clip repetition ends
/// * when a clip ends (if the clip repeats multiple times, this only occurs at the end of the last repetition)
/// * when an animation repetition ends
/// * when an animation ends (if the animation repeats multiple times, this only occurs at the end of the last repetition)
/// * when an [animation marker](crate::prelude::Clip::add_marker) is hit
///
/// # Example
///
/// You can use those events to be notified of a clip/animation ending.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// fn create_death_animation(
///     mut commands: Commands,
/// #   animation_handle: Handle<Animation>
/// ) {
///     // ... omitted: create a super scary death animation
///
///     // To use this animation from another system, you might want to keep it around.
///     // For example, you could store it in a resource.
///
///     commands.insert_resource(MyDeathAnimation(animation_handle));
/// }
///
/// #[derive(Resource)]
/// struct MyDeathAnimation(Handle<Animation>);
///
/// # fn explode() {}
/// fn explode_on_death(
///     mut messages: MessageReader<AnimationEvent>,
///     my_death_animation: Res<MyDeathAnimation>,
/// ) {
///     for message in messages.read() {
///         match message {
///             // Some animation just ended and it was the main character's death animation,
///             AnimationEvent::AnimationEnd { animation, .. } if *animation == my_death_animation.0 => {
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
/// fn create_animated_sprite(
///     mut commands: Commands,
/// ) {
///     // Let's create a marker to be notified when the exact frame
///     // where the character shoots their gun is played
///
///     let mut clip = Clip::from_frames([10, 11, 15, 16, 17]);
///
///     let marker_id = clip.add_marker(3);
///
///     // ... omitted: create an animation from that clip
///
///     // To use this marker from another system, you might want to keep it around.
///     // For example, you could store it in a resource.
///
///     commands.insert_resource(MyMarker(marker_id));
/// }
///
/// #[derive(Resource)]
/// struct MyMarker(MarkerId);
///
/// // We can watch events from any system and react to them
/// fn spawn_bullets(
///     mut messages: MessageReader<AnimationEvent>,
///     my_marker: Res<MyMarker>,
///) {
///     for message in messages.read() {
///         match message {
///             // Some marker was just hit and it was our "bullet goes out" marker, let's spawn a bullet
///             AnimationEvent::MarkerHit { marker_id, .. } if *marker_id == my_marker.0 => {
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
        marker_id: MarkerId,
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
