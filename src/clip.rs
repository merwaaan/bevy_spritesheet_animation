use std::{collections::HashMap, fmt};

use crate::{
    animation::{AnimationDirection, AnimationDuration},
    easing::Easing,
    events::AnimationMarkerId,
};

/// An opaque identifier that references a [Clip].
///
/// Returned by [AnimationLibrary::register_clip](crate::prelude::AnimationLibrary::register_clip).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ClipId {
    pub(crate) value: usize,
}

impl fmt::Display for ClipId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "clip{}", self.value)
    }
}

/// A [Clip] is a sequence of frames.
///
/// It is the most basic building block for creating animations.
/// Simple animations may contain a single clip while more complex animations may contain a sequence of clips.
///
/// Parameters like duration, repetitions, direction and easing can be specified.
///
/// A clip can also contain markers to identify frames of interest.
/// When an animation reaches such a frame, a [MarkerHit](crate::prelude::AnimationEvent::MarkerHit) event will be emitted.
/// See the documentation of [AnimationEvent](crate::prelude::AnimationEvent) for more details.
///
/// The "frames" of a clip actually are TextureAtlas entries, referred to by their indices.
/// At runtime, they will be automatically assigned to your entities' [TextureAtlas](bevy::prelude::TextureAtlas) component to make things move.
///
/// # Example
///
/// ```
/// # use bevy_spritesheet_animation::prelude::*;
/// # let mut library = AnimationLibrary::new();
/// // Create a clip
///
/// let spritesheet = Spritesheet::new(8, 4);
///
/// let clip = Clip::from_frames(spritesheet.row(3))
///     .with_repetitions(5)
///     .with_duration(AnimationDuration::PerRepetition(1000));
///
/// let clip_id = library.register_clip(clip.clone());
///
/// // For simple cases, just add a single clip to an animation
///
/// let simple_animation = Animation::from_clip(clip_id);
///
/// // You can also compose animations from multiple clips
/// //
/// // Here, an animation made of two clips:
/// // - first, the clip played once, slowly
/// // - then, the clip played twice, faster
///
/// let slow_clip = clip.clone()
///     .with_duration(AnimationDuration::PerRepetition(5000));
///
/// let slow_clip_id = library.register_clip(slow_clip);
///
/// let fast_clip = clip.clone()
///     .with_duration(AnimationDuration::PerRepetition(200))
///     .with_repetitions(2);
///
/// let fast_clip_id = library.register_clip(fast_clip);
///
/// let composite_animation = Animation::from_clips([slow_clip_id, fast_clip_id]);
/// ```
#[derive(Debug, Clone)]
pub struct Clip {
    /// Indices into the layout of a TextureAtlas component
    atlas_indices: Vec<usize>,

    /// The optional duration of this animation
    duration: Option<AnimationDuration>,

    /// The optional number of repetitions of this animation
    repetitions: Option<usize>,

    /// The optional direction of this animation
    direction: Option<AnimationDirection>,

    /// The optional easing of this animation
    easing: Option<Easing>,

    /// Markers that will generate [MarkerHit](crate::prelude::AnimationEvent::MarkerHit) events when played by an animation
    markers: HashMap<usize, Vec<AnimationMarkerId>>,
}

impl Clip {
    /// Creates a new clip from frame indices.
    ///
    /// You can provide raw indices:
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// let clip = Clip::from_frames([1, 2, 3]);
    /// ```
    ///
    /// Alternatively, you can use the [Spritesheet](crate::prelude::Spritesheet) helper to extract indices from a spritesheet:
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// let spritesheet = Spritesheet::new(8, 4);
    ///
    /// let clip1 = Clip::from_frames(spritesheet.row(2));
    ///
    /// let clip2 = Clip::from_frames(spritesheet.column(3));
    /// ```
    pub fn from_frames(atlas_indices: impl IntoIterator<Item = usize>) -> Self {
        Self {
            atlas_indices: atlas_indices.into_iter().collect(),
            duration: None,
            repetitions: None,
            direction: None,
            easing: None,
            markers: HashMap::new(),
        }
    }

    pub fn frames(&self) -> &[usize] {
        &self.atlas_indices
    }

    pub fn markers(&self) -> &HashMap<usize, Vec<AnimationMarkerId>> {
        &self.markers
    }

    pub fn with_marker(&self, marker_id: AnimationMarkerId, frame_index: usize) -> Self {
        let mut other = self.clone();

        let frame_markers = other.markers.entry(frame_index).or_default();
        frame_markers.push(marker_id);

        other
    }

    pub fn add_marker(&mut self, marker_id: AnimationMarkerId, frame_index: usize) -> &mut Self {
        let frame_markers = self.markers.entry(frame_index).or_default();
        frame_markers.push(marker_id);
        self
    }

    pub fn duration(&self) -> &Option<AnimationDuration> {
        &self.duration
    }

    pub fn with_duration(&self, duration: AnimationDuration) -> Self {
        Self {
            duration: Some(duration),
            ..self.clone()
        }
    }

    pub fn set_duration(&mut self, duration: AnimationDuration) -> &mut Self {
        self.duration = Some(duration);
        self
    }

    pub fn repetitions(&self) -> &Option<usize> {
        &self.repetitions
    }

    pub fn with_repetitions(&self, repetitions: usize) -> Self {
        Self {
            repetitions: Some(repetitions),
            ..self.clone()
        }
    }

    pub fn set_repetitions(&mut self, repetitions: usize) -> &mut Self {
        self.repetitions = Some(repetitions);
        self
    }

    pub fn direction(&self) -> &Option<AnimationDirection> {
        &self.direction
    }

    pub fn with_direction(&self, direction: AnimationDirection) -> Self {
        Self {
            direction: Some(direction),
            ..self.clone()
        }
    }

    pub fn set_direction(&mut self, direction: AnimationDirection) -> &mut Self {
        self.direction = Some(direction);
        self
    }

    pub fn easing(&self) -> &Option<Easing> {
        &self.easing
    }

    pub fn with_easing(&self, easing: Easing) -> Self {
        Self {
            easing: Some(easing),
            ..self.clone()
        }
    }

    pub fn set_easing(&mut self, easing: Easing) -> &mut Self {
        self.easing = Some(easing);
        self
    }
}
