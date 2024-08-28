use std::collections::{HashMap, HashSet};

use bevy::prelude::Resource;

use crate::{
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
#[derive(Resource)]
pub struct AnimationLibrary {
    /// All the clips
    clips: HashMap<ClipId, Clip>,

    /// Name to ID lookup for clips
    clip_name_lookup: HashMap<String, ClipId>,

    /// All the animations
    animations: HashMap<AnimationId, Animation>,

    /// Name to ID lookup for animations
    animation_name_lookup: HashMap<String, AnimationId>,

    /// All the markers
    markers: HashSet<AnimationMarkerId>,

    /// Name to ID lookup for markers
    marker_name_lookup: HashMap<String, AnimationMarkerId>,
}

impl AnimationLibrary {
    #[cfg_attr(feature = "integration-tests", visibility::make(pub))]
    pub(crate) fn new() -> Self {
        Self {
            clips: HashMap::new(),
            clip_name_lookup: HashMap::new(),
            animations: HashMap::new(),
            animation_name_lookup: HashMap::new(),
            markers: HashSet::new(),
            marker_name_lookup: HashMap::new(),
        }
    }

    /// Registers a [Clip] and returns its ID.
    ///
    /// The clip can then be referenced in one or several [Animation]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::new();
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
    /// # let mut library = AnimationLibrary::new();
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

        if let Some(named_clip_id) = self.clip_name_lookup.get(&name) {
            // The clip already has this name: no-op
            if *named_clip_id == clip_id {
                Ok(())
            } else {
                Err(LibraryError::NameAlreadyTaken)
            }
        } else {
            self.clip_name_lookup.insert(name, clip_id);
            Ok(())
        }
    }

    /// Returns the ID of the clip with the given name if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - the clip name
    pub fn clip_with_name(&self, name: impl Into<String>) -> Option<ClipId> {
        self.clip_name_lookup.get(&name.into()).copied()
    }

    /// Returns true if a clip has the given name.
    ///
    /// # Arguments
    ///
    /// * `clip_id` - the ID of the clip to check the name of
    /// * `name` - the name to check
    pub fn is_clip_name(&self, clip_id: ClipId, name: impl Into<String>) -> bool {
        self.clip_name_lookup
            .get(&name.into())
            .map(|id| *id == clip_id)
            .unwrap_or(false)
    }

    /// Returns all the clips registered in the library.
    pub fn clips(&self) -> &HashMap<ClipId, Clip> {
        &self.clips
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
    /// # let mut library = AnimationLibrary::new();
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

        if let Some(named_animation_id) = self.animation_name_lookup.get(&name) {
            // The animation already has this name: no-op
            if *named_animation_id == animation_id {
                Ok(())
            } else {
                Err(LibraryError::NameAlreadyTaken)
            }
        } else {
            self.animation_name_lookup.insert(name, animation_id);
            Ok(())
        }
    }

    /// Returns the ID of the animation with the given name if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - the animation name
    pub fn animation_with_name(&self, name: impl Into<String>) -> Option<AnimationId> {
        self.animation_name_lookup.get(&name.into()).copied()
    }

    /// Returns true if an animation has the given name.
    ///
    /// # Arguments
    ///
    /// * `animation_id` - the ID of the animation to check the name of
    /// * `name` - the name to check
    pub fn is_animation_name(&self, animation_id: AnimationId, name: impl Into<String>) -> bool {
        self.animation_name_lookup
            .get(&name.into())
            .map(|id| *id == animation_id)
            .unwrap_or(false)
    }

    /// Returns all the animations registered in the library.
    pub fn animations(&self) -> &HashMap<AnimationId, Animation> {
        &self.animations
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
    /// # let mut library = AnimationLibrary::new();
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
    /// # let mut library = AnimationLibrary::new();
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

        if let Some(named_marker_id) = self.marker_name_lookup.get(&name) {
            // The marker already has this name: no-op
            if *named_marker_id == marker_id {
                Ok(())
            } else {
                Err(LibraryError::NameAlreadyTaken)
            }
        } else {
            self.marker_name_lookup.insert(name, marker_id);
            Ok(())
        }
    }

    /// Returns the ID of the marker with the given name if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - the marker name
    pub fn marker_with_name(&self, name: impl Into<String>) -> Option<AnimationMarkerId> {
        self.marker_name_lookup.get(&name.into()).copied()
    }

    /// Returns true if an animation marker has the given name.
    ///
    /// # Arguments
    ///
    /// * `marker_id` - the ID of the marker to check the name of
    /// * `name` - the name to check
    pub fn is_marker_name(&self, marker_id: AnimationMarkerId, name: impl Into<String>) -> bool {
        self.marker_name_lookup
            .get(&name.into())
            .map(|id| *id == marker_id)
            .unwrap_or(false)
    }

    /// Returns all the animation markers registered in the library.
    pub fn markers(&self) -> &HashSet<AnimationMarkerId> {
        &self.markers
    }
}
