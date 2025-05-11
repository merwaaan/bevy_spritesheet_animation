use std::{
    collections::{HashMap, HashSet},
    sync::{
        // TODO: Use bevy_platform when updated to Bevy 0.16.
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use bevy::{ecs::reflect::*, prelude::Resource, reflect::prelude::*};

use crate::{
    animator::cache::AnimationCache,
    clip::{Clip, ClipId},
    events::AnimationMarkerId,
    prelude::{Animation, AnimationId},
};

/// Error type returned by some [AnimationLibrary] methods.
#[derive(Debug)]
pub enum LibraryError {
    /// The name given to a clip/animation/marker is already in use
    NameAlreadyTaken,
}

/// The animation library is the global store for clips and animations.
///
/// When the [SpritesheetAnimationPlugin](crate::prelude::SpritesheetAnimationPlugin) is added to the app, the [AnimationLibrary] becomes available as a resource.
///
/// You can then use it to register new clips and animations, and create new markers.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// fn my_system(mut library: ResMut<AnimationLibrary>) {
///     // Create a marker
///
///     let marker_id = library.new_marker();
///
///     // Create a clip and attach the marker created above
///
///     let clip = Clip::from_frames([0, 1, 2, 3])
///         .with_marker(marker_id, 5);
///
///     let clip_id = library.register_clip(clip);
///
///     // Create an animation that uses the clip
///
///     let animation = Animation::from_clip(clip_id);
///
///     let animation_id = library.register_animation(animation);
///
///     // ... Assign the animation to a SpritesheetAnimation component ...
/// }
/// ```
#[derive(Resource, Default, Reflect)]
#[reflect(Resource, Default)]
pub struct AnimationLibrary {
    /// All the clips
    clips: HashMap<ClipId, Clip>,

    /// Optional clip names
    clip_names: HashMap<ClipId, String>,

    /// All the animations
    animations: HashMap<AnimationId, Animation>,

    /// Optional animation names
    animation_names: HashMap<AnimationId, String>,

    /// All the markers
    markers: HashSet<AnimationMarkerId>,

    /// Optional marker names
    marker_names: HashMap<AnimationMarkerId, String>,

    /// Animation caches, one for each animation.
    /// They contain all the data required to play an animation.
    animation_caches: HashMap<AnimationId, Arc<AnimationCache>>,

    next_id: AtomicUsize,
}

impl AnimationLibrary {
    /// Registers a [Clip] and returns its ID.
    ///
    /// The clip can then be referenced in one or several [Animation]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// let clip = Clip::from_frames([4, 5, 6]).with_repetitions(10);
    ///
    /// let clip_id = library.register_clip(clip);
    ///
    /// let animation = Animation::from_clip(clip_id);
    ///
    /// // ...
    /// ```
    pub fn register_clip(&mut self, clip: Clip) -> ClipId {
        let id = ClipId {
            value: self.next_id.fetch_add(1, Ordering::Relaxed),
        };

        self.clips.insert(id, clip);

        id
    }

    /// Deregisters a clip from the library.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// let clip = Clip::from_frames([1, 2, 3]);
    ///
    /// let clip_id = library.register_clip(clip);
    ///
    /// library.deregister_clip(clip_id);
    ///
    /// assert!(library.get_clip(clip_id).is_none());
    /// ```
    pub fn deregister_clip(&mut self, clip_id: ClipId) {
        self.clips.remove(&clip_id);
        self.clip_names.remove(&clip_id);
    }

    /// Associates a unique name to a clip.
    ///
    /// The clip ID can then later be queried from that name with [AnimationLibrary::clip_with_name].
    ///
    /// Returns a [LibraryError::NameAlreadyTaken] error if the name is already in use.
    ///
    /// # Arguments
    ///
    /// * `clip_id` - the ID of the clip to name
    /// * `name` - the name to assign
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// let clip = Clip::from_frames([1, 2, 3]);
    /// let clip_id = library.register_clip(clip);
    ///
    /// library.name_clip(clip_id, "jump");
    ///
    /// assert_eq!(library.clip_with_name("jump"), Some(clip_id));
    /// assert!(library.is_clip_name(clip_id, "jump"));
    /// ```
    pub fn name_clip(
        &mut self,
        clip_id: ClipId,
        name: impl Into<String>,
    ) -> Result<(), LibraryError> {
        let name = name.into();

        if let Some(existing_clip_id) = self.clip_with_name(&name) {
            // The clip already has this name: no-op
            if existing_clip_id == clip_id {
                Ok(())
            } else {
                Err(LibraryError::NameAlreadyTaken)
            }
        } else {
            self.clip_names.insert(clip_id, name);
            Ok(())
        }
    }

    /// Returns all the clip names registered in the library.
    pub fn clip_names(&self) -> &HashMap<ClipId, String> {
        &self.clip_names
    }

    /// Returns the ID of the clip with the given name if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - the clip name
    pub fn clip_with_name(&self, name: impl AsRef<str>) -> Option<ClipId> {
        self.clip_names.iter().find_map(|(k, v)| {
            if v.as_str() == name.as_ref() {
                Some(*k)
            } else {
                None
            }
        })
    }

    /// Returns the name of the clip with the given ID if it exists.
    ///
    /// # Arguments
    ///
    /// * `clip_id` - the clip id
    pub fn get_clip_name(&self, clip_id: ClipId) -> Option<&str> {
        self.clip_names.iter().find_map(|(k, v)| {
            if k == &clip_id {
                Some(v.as_str())
            } else {
                None
            }
        })
    }

    /// Returns true if a clip has the given name.
    ///
    /// # Arguments
    ///
    /// * `clip_id` - the ID of the clip to check the name of
    /// * `name` - the name to check
    pub fn is_clip_name(&self, clip_id: ClipId, name: impl AsRef<str>) -> bool {
        self.clip_names
            .get(&clip_id)
            .map(|v| v == name.as_ref())
            .unwrap_or(false)
    }

    /// Returns all the clips registered in the library.
    pub fn clips(&self) -> &HashMap<ClipId, Clip> {
        &self.clips
    }

    /// Returns a clip registered in the library.
    pub fn get_clip(&self, clip_id: ClipId) -> &Clip {
        // In practice, this cannot fail as the library is the sole creator of IDs
        self.clips.get(&clip_id).unwrap()
    }

    /// Registers an new [Animation] and returns its ID.
    ///
    /// The animation can then be referenced in [SpritesheetAnimation](crate::prelude::SpritesheetAnimation) components.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// fn f(
    ///     mut commands: Commands,
    ///     mut library: AnimationLibrary,
    ///     # image: Handle<Image>,
    ///     # atlas: TextureAtlas
    ///     // ...
    /// ) {
    ///     let clip = Clip::from_frames([4, 5, 6]);
    ///
    ///     let clip_id = library.register_clip(clip);
    ///
    ///     let animation = Animation::from_clip(clip_id)
    ///         .with_duration(AnimationDuration::PerRepetition(1500));
    ///
    ///     let animation_id = library.register_animation(animation);
    ///
    ///     // The animation can then be assigned to an entity
    ///
    ///     // ... omitted: load an image and create a texture atlas for the sprite ...
    ///
    ///     commands.spawn((
    ///         Sprite::from_atlas_image(image, atlas),
    ///         SpritesheetAnimation::from_id(animation_id)
    ///     ));
    /// }
    /// ```
    pub fn register_animation(&mut self, animation: Animation) -> AnimationId {
        let id = AnimationId {
            value: self.next_id.fetch_add(1, Ordering::Relaxed),
        };

        self.animations.insert(id, animation);

        self.animation_caches
            .insert(id, Arc::new(AnimationCache::new(id, self)));

        id
    }

    /// Deregisters an animation from the library.
    ///
    /// This also deregisters all the clips associated with the animation.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// fn f(
    ///     mut commands: Commands,
    ///     mut library: AnimationLibrary,
    ///     # image: Handle<Image>,
    ///     # atlas: TextureAtlas
    ///     // ...
    /// ) {
    ///     let clip = Clip::from_frames([4, 5, 6]);
    ///
    ///     let clip_id = library.register_clip(clip);
    ///
    ///     let animation = Animation::from_clip(clip_id)
    ///         .with_duration(AnimationDuration::PerRepetition(1500));
    ///
    ///     let animation_id = library.register_animation(animation);
    ///
    ///     // Later, when finished with the animation, deregister it.
    ///
    ///     library.deregister_animation(animation_id);
    ///
    ///     // The animation and its clips are no longer available in the library.
    ///     assert!(library.get_animation(animation_id).is_none());
    ///     assert!(library.get_clip(clip_id).is_none());
    /// }
    /// ```
    pub fn deregister_animation(&mut self, animation_id: AnimationId) {
        let maybe_animation = self.animations.remove(&animation_id);

        if let Some(animation) = maybe_animation {
            for clip_id in animation.clip_ids() {
                self.deregister_clip(*clip_id);
            }
        }

        self.animation_caches.remove(&animation_id);
        self.animation_names.remove(&animation_id);
    }

    /// Associates a unique name to an animation.
    ///
    /// The animation ID can then later be queried from that name with [AnimationLibrary::animation_with_name].
    ///
    /// Returns a [LibraryError::NameAlreadyTaken] error if the name is already in use.
    ///
    /// # Arguments
    ///
    /// * `animation_id` - the ID of the animation to name
    /// * `name` - the name to assign
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// # let clip = Clip::from_frames([]);
    /// # let clip_id = library.register_clip(clip);
    /// let animation = Animation::from_clip(clip_id);
    ///
    /// let animation_id = library.register_animation(animation);
    ///
    /// library.name_animation(animation_id, "crouch");
    ///
    /// assert_eq!(library.animation_with_name("crouch"), Some(animation_id));
    /// assert!(library.is_animation_name(animation_id, "crouch"));
    /// ```
    pub fn name_animation(
        &mut self,
        animation_id: AnimationId,
        name: impl Into<String>,
    ) -> Result<(), LibraryError> {
        let name = name.into();

        if let Some(existing_animation_id) = self.animation_with_name(&name) {
            // The animation already has this name: no-op
            if existing_animation_id == animation_id {
                Ok(())
            } else {
                Err(LibraryError::NameAlreadyTaken)
            }
        } else {
            self.animation_names.insert(animation_id, name);
            Ok(())
        }
    }

    /// Returns all the animation names registered in the library.
    pub fn animation_names(&self) -> &HashMap<AnimationId, String> {
        &self.animation_names
    }

    /// Returns the ID of the animation with the given name if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - the animation name
    pub fn animation_with_name(&self, name: impl AsRef<str>) -> Option<AnimationId> {
        self.animation_names.iter().find_map(
            |(k, v)| {
                if v == name.as_ref() {
                    Some(*k)
                } else {
                    None
                }
            },
        )
    }

    /// Returns the name of the animation with the given ID if it exists.
    ///
    /// # Arguments
    ///
    /// * `animation_id` - the animation id
    pub fn get_animation_name(&self, animation_id: AnimationId) -> Option<&str> {
        self.animation_names.iter().find_map(|(k, v)| {
            if k == &animation_id {
                Some(v.as_str())
            } else {
                None
            }
        })
    }

    /// Returns true if an animation has the given name.
    ///
    /// # Arguments
    ///
    /// * `animation_id` - the ID of the animation to check the name of
    /// * `name` - the name to check
    pub fn is_animation_name(&self, animation_id: AnimationId, name: impl AsRef<str>) -> bool {
        self.animation_names
            .get(&animation_id)
            .map(|v| v == name.as_ref())
            .unwrap_or(false)
    }

    /// Returns all the animations registered in the library.
    pub fn animations(&self) -> &HashMap<AnimationId, Animation> {
        &self.animations
    }

    /// Returns an animation registered in the library.
    pub fn get_animation(&self, animation_id: AnimationId) -> &Animation {
        // In practice, this cannot fail as the library is the sole creator of IDs
        self.animations.get(&animation_id).unwrap()
    }

    /// Creates a new animation marker and returns a unique ID to refer to it.
    ///
    /// The marker can then be inserted into [Clip]s and an [AnimationEvent::MarkerHit](crate::prelude::AnimationEvent::MarkerHit) event
    /// will be emitted whenever an animation reaches it.
    ///
    /// For more details, see the documentation of [AnimationEvent](crate::prelude::AnimationEvent).
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// let marker = library.new_marker();
    ///
    /// let clip = Clip::from_frames([7, 8, 9, 10, 11, 12])
    ///     .with_marker(marker, 3);
    /// ```
    pub fn new_marker(&mut self) -> AnimationMarkerId {
        let id = AnimationMarkerId {
            value: self.next_id.fetch_add(1, Ordering::Relaxed),
        };

        self.markers.insert(id);

        id
    }

    /// Associates a unique name to an animation marker.
    ///
    /// The marker ID can then later be queried from that name with [AnimationLibrary::marker_with_name].
    ///
    /// Returns a [LibraryError::NameAlreadyTaken] error if the name is already in use.
    ///
    /// # Arguments
    ///
    /// * `marker_id` - the ID of the marker to name
    /// * `name` - the name to assign
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// let marker_id = library.new_marker();
    ///
    /// library.name_marker(marker_id, "raise sword");
    ///
    /// assert_eq!(library.marker_with_name("raise sword"), Some(marker_id));
    /// assert!(library.is_marker_name(marker_id, "raise sword"));
    /// ```
    pub fn name_marker(
        &mut self,
        marker_id: AnimationMarkerId,
        name: impl Into<String>,
    ) -> Result<(), LibraryError> {
        let name = name.into();

        if let Some(existing_marker_id) = self.marker_with_name(&name) {
            // The marker already has this name: no-op
            if existing_marker_id == marker_id {
                Ok(())
            } else {
                Err(LibraryError::NameAlreadyTaken)
            }
        } else {
            self.marker_names.insert(marker_id, name);
            Ok(())
        }
    }

    /// Returns all the marker names registered in the library.
    pub fn marker_names(&self) -> &HashMap<AnimationMarkerId, String> {
        &self.marker_names
    }

    /// Returns the ID of the marker with the given name if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - the marker name
    pub fn marker_with_name(&self, name: impl AsRef<str>) -> Option<AnimationMarkerId> {
        self.marker_names.iter().find_map(|(k, v)| {
            if v.as_str() == name.as_ref() {
                Some(*k)
            } else {
                None
            }
        })
    }

    /// Returns the name of the marker with the given ID if it exists.
    ///
    /// # Arguments
    ///
    /// * `marker_id` - the marker id
    pub fn get_marker_name(&self, marker_id: AnimationMarkerId) -> Option<&str> {
        self.marker_names.iter().find_map(|(k, v)| {
            if k == &marker_id {
                Some(v.as_str())
            } else {
                None
            }
        })
    }

    /// Returns true if an animation marker has the given name.
    ///
    /// # Arguments
    ///
    /// * `marker_id` - the ID of the marker to check the name of
    /// * `name` - the name to check
    pub fn is_marker_name(&self, marker_id: AnimationMarkerId, name: impl AsRef<str>) -> bool {
        self.marker_names
            .get(&marker_id)
            .map(|v| v == name.as_ref())
            .unwrap_or(false)
    }

    /// Returns all the animation markers registered in the library.
    pub fn markers(&self) -> &HashSet<AnimationMarkerId> {
        &self.markers
    }

    /// Returns the cache for an animation registered in the library
    pub(crate) fn get_animation_cache(&self, animation_id: AnimationId) -> Arc<AnimationCache> {
        // In practice, this cannot fail as the library is the sole creator of IDs
        // and the cache is created when registering the animation
        self.animation_caches.get(&animation_id).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animation_ids_should_be_unique_after_deregistration() {
        let mut library = AnimationLibrary::default();

        // First animation registration.
        let clip1 = Clip::from_frames(vec![0]);
        let clip_id1 = library.register_clip(clip1);
        let anim1 = Animation::from_clip(clip_id1);
        let anim_id1 = library.register_animation(anim1);

        // Deregister the animation and its clips.
        library.deregister_animation(anim_id1);

        // Second animation registration should get a unique ID.
        let clip2 = Clip::from_frames(vec![1]);
        let clip_id2 = library.register_clip(clip2);
        let anim2 = Animation::from_clip(clip_id2);
        let anim_id2 = library.register_animation(anim2);

        assert_ne!(
            anim_id1, anim_id2,
            "Each animation should receive a unique ID even after deregistration"
        );
    }

    #[test]
    fn clip_ids_should_be_unique_after_deregistration() {
        let mut library = AnimationLibrary::default();

        // First clip registration.
        let clip1 = Clip::from_frames(vec![0]);
        let clip_id1 = library.register_clip(clip1);

        // Deregister the clip.
        library.deregister_clip(clip_id1);

        // Second clip registration should get a unique ID.
        let clip2 = Clip::from_frames(vec![1]);
        let clip_id2 = library.register_clip(clip2);

        assert_ne!(
            clip_id1, clip_id2,
            "Each clip should receive a unique ID even after deregistration"
        );
    }
}
