use std::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

use bevy::{platform::collections::HashMap, reflect::prelude::*};

use crate::{
    animation::{AnimationDirection, AnimationDuration},
    easing::Easing,
};

/// An opaque identifier that references a [Clip]
///
/// Clip-related [AnimationEvent](crate::prelude::AnimationEvent)s will contain this ID.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub struct ClipId {
    pub(crate) value: usize,
}

impl fmt::Display for ClipId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "clip{}", self.value)
    }
}

static NEXT_CLIP_ID: AtomicUsize = AtomicUsize::new(0);

/// An opaque identifier that references an animation marker
///
/// Marker-related [AnimationEvent](crate::prelude::AnimationEvent)s will contain this ID.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub struct MarkerId {
    pub(crate) value: usize,
}

impl fmt::Display for MarkerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "marker{}", self.value)
    }
}

static NEXT_MARKER_ID: AtomicUsize = AtomicUsize::new(0);

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
/// At runtime, they will be automatically assigned to the entities' [TextureAtlas](bevy::prelude::TextureAtlas) component to make things move.
///
/// # Example
///
/// ```
/// # use bevy_spritesheet_animation::prelude::*;
/// // Create a clip
///
/// let spritesheet = Spritesheet::new(8, 4);
///
/// let clip = Clip::from_frames(spritesheet.row(3))
///     .with_repetitions(5)
///     .with_duration(AnimationDuration::PerRepetition(1000));
///
/// // For simple cases, just add a single clip to an animation
///
/// let simple_animation = Animation::from_clip(clip.clone());
///
/// // You can also compose animations from multiple clips
/// //
/// // Here, an animation made of two clips:
/// // - first, the clip played once, slowly
/// // - then, the same clip played twice, faster
///
/// let slow_clip = clip
///     .clone()
///     .with_duration(AnimationDuration::PerRepetition(5000));
///
/// let fast_clip = clip
///     .clone()
///     .with_repetitions(2)
///     .with_duration(AnimationDuration::PerRepetition(200));
///
/// let composite_animation = Animation::from_clips([slow_clip, fast_clip]);
/// ```
#[derive(Debug, Clone, Reflect)]
#[reflect(Debug)]
pub struct Clip {
    /// Unique ID of this clip
    ///
    /// Marker-related [AnimationEvent](crate::prelude::AnimationEvent)s will contain this ID.
    id: ClipId,

    /// Indices into the layout of a TextureAtlas component
    atlas_indices: Vec<usize>,

    /// The optional duration of this clip
    duration: Option<AnimationDuration>,

    /// The optional number of repetitions of this clip
    repetitions: Option<usize>,

    /// The optional direction of this clip
    direction: Option<AnimationDirection>,

    /// The optional easing of this clip
    easing: Option<Easing>,

    /// Markers that will trigger [MarkerHit](crate::prelude::AnimationEvent::MarkerHit) events when the corresponding frame is played
    ///
    /// The key is the frame index.
    /// Multiple markers can be associated to the same frame.
    markers: HashMap<usize, Vec<MarkerId>>,
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
            id: ClipId {
                value: NEXT_CLIP_ID.fetch_add(1, Ordering::Relaxed),
            },
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

    pub fn markers(&self) -> &HashMap<usize, Vec<MarkerId>> {
        &self.markers
    }

    pub fn add_marker(&mut self, frame_index: usize) -> MarkerId {
        let marker_id = MarkerId {
            value: NEXT_MARKER_ID.fetch_add(1, Ordering::Relaxed),
        };

        let frame_markers = self.markers.entry(frame_index).or_default();

        frame_markers.push(marker_id);

        marker_id
    }

    pub fn id(&self) -> ClipId {
        self.id
    }
}
