use std::{collections::HashMap, fmt};

use crate::{
    animation::{AnimationDirection, AnimationDuration},
    easing::Easing,
    events::AnimationMarkerId,
};

/// An opaque identifier that references an [AnimationClip].
///
/// Returned by [SpritesheetLibrary::new_clip](crate::prelude::SpritesheetLibrary::new_clip).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct AnimationClipId {
    pub(crate) value: usize,
}

impl fmt::Display for AnimationClipId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "clip:{}", self.value)
    }
}

/// An [AnimationClip] is a reusable sequence of frames.
/// It is the most basic building block for creating animations.
///
/// The "frames" of an animation clip actually are TextureAtlas entries, referred to by their indices.
/// At runtime, they will be automatically assigned to our entities' [TextureAtlas](bevy::prelude::TextureAtlas) component to make things move.
///
/// Default parameters like duration, repeat and direction can be specified.
/// The [AnimationStage](crate::prelude::AnimationStage)s that reference an [AnimationClip] will inherit its parameters if they don't specify their own.
///
/// An animation clip can also contain markers to identify frames of interest.
/// When an animation reaches such a frame, a [MarkerHit](crate::prelude::AnimationEvent::MarkerHit) event will be emitted.
/// See the documentation of [AnimationEvent](crate::prelude::AnimationEvent) for more details.
///
/// # Example
///
/// ```
/// # use bevy_spritesheet_animation::prelude::*;
/// # let mut library = SpritesheetLibrary::new();
/// let spritesheet = Spritesheet::new(8, 4);
///
/// let clip_id = library.new_clip(|clip| {
///     clip
///         // Use all the frames in row 4
///         .push_frame_indices(spritesheet.row(4))
///         // Set a default duration for this clip
///         .set_default_duration(AnimationDuration::PerCycle(500));
/// });
///
/// // For simple animations, just pass the clip to an animation with into()
///
/// let animation1_id = library.new_animation(|animation| {
///     animation.add_stage(clip_id.into());
/// });
///
/// // If you want more flexibility, explicitly use animation stages
///
/// let animation2_id = library.new_animation(|animation| {
///     // An animation with two stages:
///     // - clip played once, slowly
///     // - the same clip played twice, with a higher speed
///
///     let mut slow_stage = AnimationStage::from_clip(clip_id);
///     slow_stage.set_duration(AnimationDuration::PerCycle(500));
///
///     let mut fast_stage = AnimationStage::from_clip(clip_id);
///     fast_stage
///         .set_duration(AnimationDuration::PerCycle(300))
///         .set_repeat(2);
///
///     animation
///         .add_stage(slow_stage)
///         .add_stage(fast_stage);
/// });
/// ```
#[derive(Debug, Clone)]
pub struct AnimationClip {
    /// Indices into the layout of a TextureAtlas component
    atlas_indices: Vec<usize>,

    /// The default duration of this clip, will be inherited by the [AnimationStage] using this clip
    default_duration: Option<AnimationDuration>,
    /// The default repetitions of this clip, will be inherited by the [AnimationStage] using this clip
    default_repeat: Option<u32>,
    /// The default direction of this clip, will be inherited by the [AnimationStage] using this clip
    default_direction: Option<AnimationDirection>,
    /// The default easing of this clip, will be inherited by the [AnimationStage] using this clip
    default_easing: Option<Easing>,

    /// Markers that will generate [MarkerHit](crate::prelude::AnimationEvent::MarkerHit) events when played by an animation
    markers: HashMap<usize, Vec<AnimationMarkerId>>,
}

impl AnimationClip {
    /// Creates a new animation clip
    pub(crate) fn new() -> Self {
        Self {
            atlas_indices: Vec::new(),
            default_duration: None,
            default_repeat: None,
            default_direction: None,
            default_easing: None,
            markers: HashMap::new(),
        }
    }

    /// Pushes new frames into the clip.
    ///
    /// # Arguments
    ///
    /// `indices`: the indices of the frames in the sprite's texture atlas
    ///
    /// # Examples
    ///
    /// You can  specify frames with raw indices:
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = SpritesheetLibrary::new();
    /// let clip_id = library.new_clip(|clip| {
    ///     clip.push_frame_indices([0, 1, 7, 8]);
    /// });
    /// ```
    ///
    /// Alternatively, you can use a [Spritesheet](crate::prelude::Spritesheet) helper:
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = SpritesheetLibrary::new();
    /// let spritesheet = Spritesheet::new(3, 4);
    ///
    /// let clip_id = library.new_clip(|clip| {
    ///     clip.push_frame_indices(spritesheet.row(2));
    /// });
    /// ```
    pub fn push_frame_indices<I: IntoIterator<Item = usize>>(&mut self, indices: I) -> &mut Self {
        self.atlas_indices.extend(indices);
        self
    }
    /// Adds a marker to a frame of this clip.
    ///
    /// # Arguments
    ///
    /// * `marker_id` - the ID of the marker
    /// * `frame_index` - the index of the frame to attach the marker to
    ///
    /// # Example
    ///
    /// See the documentation of [AnimationEvent](crate::prelude::AnimationEvent) for more details.
    pub fn add_marker(&mut self, marker_id: AnimationMarkerId, frame_index: usize) -> &mut Self {
        let frame_markers = self.markers.entry(frame_index).or_default();
        frame_markers.push(marker_id);
        self
    }

    pub fn markers(&self) -> &HashMap<usize, Vec<AnimationMarkerId>> {
        &self.markers
    }

    pub fn frame_indices(&self) -> &[usize] {
        &self.atlas_indices
    }

    pub fn frame_count(&self) -> usize {
        self.atlas_indices.len()
    }

    pub fn set_default_duration(&mut self, duration: AnimationDuration) -> &mut Self {
        self.default_duration = Some(duration);
        self
    }

    pub fn default_duration(&self) -> &Option<AnimationDuration> {
        &self.default_duration
    }

    pub fn set_default_repeat(&mut self, repeat: u32) -> &mut Self {
        self.default_repeat = Some(repeat);
        self
    }

    pub fn default_repeat(&self) -> &Option<u32> {
        &self.default_repeat
    }

    pub fn set_default_direction(&mut self, direction: AnimationDirection) -> &mut Self {
        self.default_direction = Some(direction);
        self
    }

    pub fn default_direction(&self) -> &Option<AnimationDirection> {
        &self.default_direction
    }

    pub fn set_default_easing(&mut self, easing: Easing) -> &mut Self {
        self.default_easing = Some(easing);
        self
    }

    pub fn default_easing(&self) -> &Option<Easing> {
        &self.default_easing
    }
}
