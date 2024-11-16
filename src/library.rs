use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
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

    /// Name to ID lookup for clips
    clip_name_lookup: HashMap<ClipId, String>,

    /// All the animations
    animations: HashMap<AnimationId, Animation>,

    /// Name to ID lookup for animations
    animation_name_lookup: HashMap<AnimationId, String>,

    /// All the markers
    markers: HashSet<AnimationMarkerId>,

    /// Name to ID lookup for markers
    marker_name_lookup: HashMap<AnimationMarkerId, String>,

    /// Animation caches, one for each animation.
    /// They contain all the data required to play an animation.
    animation_caches: HashMap<AnimationId, Arc<AnimationCache>>,
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
            value: self.clips.len(),
        };

        self.clips.insert(id, clip);

        id
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
        name: impl AsRef<str>,
    ) -> Result<(), LibraryError> {
        let name = name.as_ref();

        if let Some(existing_clip_id) = self.clip_with_name(name) {
            // The clip already has this name: no-op
            if existing_clip_id == clip_id {
                Ok(())
            } else {
                Err(LibraryError::NameAlreadyTaken)
            }
        } else {
            self.clip_name_lookup.insert(clip_id, name.to_string());
            Ok(())
        }
    }

    /// Returns all the clip names registered in the library.
    pub fn clip_names(&self) -> &HashMap<ClipId, String> {
        &self.clip_name_lookup
    }

    /// Returns the ID of the clip with the given name if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - the clip name
    pub fn clip_with_name(&self, name: impl AsRef<str>) -> Option<ClipId> {
        self.clip_name_lookup.iter().find_map(|(k, v)| {
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
        self.clip_name_lookup.iter().find_map(|(k, v)| {
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
        self.clip_name_lookup
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
    ///     # texture: Handle<Image>,
    ///     # layout: Handle<TextureAtlasLayout>
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
    ///     // ... omitted: load a texture and create an atlas layout for the sprite ...
    ///
    ///     commands.spawn((
    ///         SpriteBundle {
    ///             texture: texture.clone(),
    ///             ..default()
    ///         },
    ///         TextureAtlas {
    ///             layout: layout.clone(),
    ///            ..default()
    ///         },
    ///         SpritesheetAnimation::from_id(animation_id)
    ///     ));
    /// }
    /// ```
    pub fn register_animation(&mut self, animation: Animation) -> AnimationId {
        let id = AnimationId {
            value: self.animations.len(),
        };

        self.animations.insert(id, animation);

        self.animation_caches
            .insert(id, Arc::new(AnimationCache::new(id, self)));

        id
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
        name: impl AsRef<str>,
    ) -> Result<(), LibraryError> {
        let name = name.as_ref();

        if let Some(existing_animation_id) = self.animation_with_name(name) {
            // The animation already has this name: no-op
            if existing_animation_id == animation_id {
                Ok(())
            } else {
                Err(LibraryError::NameAlreadyTaken)
            }
        } else {
            self.animation_name_lookup
                .insert(animation_id, name.to_string());
            Ok(())
        }
    }

    /// Returns all the animation names registered in the library.
    pub fn animation_names(&self) -> &HashMap<AnimationId, String> {
        &self.animation_name_lookup
    }

    /// Returns the ID of the animation with the given name if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - the animation name
    pub fn animation_with_name(&self, name: impl AsRef<str>) -> Option<AnimationId> {
        self.animation_name_lookup.iter().find_map(
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
        self.animation_name_lookup.iter().find_map(|(k, v)| {
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
        self.animation_name_lookup
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
            value: self.markers.len(),
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
        name: impl AsRef<str>,
    ) -> Result<(), LibraryError> {
        let name = name.as_ref();

        if let Some(existing_marker_id) = self.marker_with_name(name) {
            // The marker already has this name: no-op
            if existing_marker_id == marker_id {
                Ok(())
            } else {
                Err(LibraryError::NameAlreadyTaken)
            }
        } else {
            self.marker_name_lookup.insert(marker_id, name.to_string());
            Ok(())
        }
    }

    /// Returns all the marker names registered in the library.
    pub fn marker_names(&self) -> &HashMap<AnimationMarkerId, String> {
        &self.marker_name_lookup
    }

    /// Returns the ID of the marker with the given name if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - the marker name
    pub fn marker_with_name(&self, name: impl AsRef<str>) -> Option<AnimationMarkerId> {
        self.marker_name_lookup.iter().find_map(|(k, v)| {
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
        self.marker_name_lookup.iter().find_map(|(k, v)| {
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
        self.marker_name_lookup
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
