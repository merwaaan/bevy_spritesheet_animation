use std::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    animation::{AnimationDirection, AnimationDuration},
    easing::Easing,
    events::Marker,
};

/// An opaque identifier for a [Clip]
///
/// Clip-related [AnimationEvents](crate::prelude::AnimationEvent) will contain this ID.
///
/// Wen creating animations, use [AnimationBuilder::get_current_clip_id()](crate::prelude::AnimationBuilder::get_current_clip_id) to retrieve a clip's ID.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Reflect)]
#[reflect(Debug, PartialEq, Hash)]
pub struct ClipId {
    pub(crate) value: usize,
}

static NEXT_CLIP_ID: AtomicUsize = AtomicUsize::new(0);

impl fmt::Debug for ClipId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "clip{}", self.value)
    }
}

impl ClipId {
    /// Private constructor, only the crate can internally attribute new IDs.
    pub(crate) fn new() -> Self {
        Self {
            value: NEXT_CLIP_ID.fetch_add(1, Ordering::Relaxed),
        }
    }

    /// Creates a dummy clip ID that can be written to with [AnimationBuilder::get_current_clip_id()](crate::prelude::AnimationBuilder::get_current_clip_id).
    pub fn dummy() -> Self {
        Self { value: usize::MAX }
    }
}

/// A [Clip] is a sequence of frames.
///
/// It is the most basic building block for creating [Animations](crate::prelude::Animation).
/// Simple animations may contain a single clip while more sophisticated animations may contain a sequence of clips.
///
/// The "frames" of a clip actually are [TextureAtlas](https://docs.rs/bevy/latest/bevy/prelude/struct.TextureAtlas.html) entries, referred to by their indices.
/// During playback, they will be automatically assigned to the entities' texture atlas to animate the sprite.
///
/// Create clips with [AnimationBuilder::start_clip()](crate::prelude::AnimationBuilder::start_clip) and [AnimationBuilder::copy_clip()](crate::prelude::AnimationBuilder::copy_clip).
///
/// # Parameters
///
/// Playback parameters like [duration](crate::prelude::AnimationDuration), repetitions, [direction](crate::prelude::AnimationDirection) and [easing](crate::prelude::Easing) can be specified.
///
/// Those clip-level parameters will be combined with the parameters of the parent animation.
///
/// # Markers
///
/// A clip can contain markers to identify frames of interest.
///
/// When an animation reaches such a frame, a [MarkerHit](crate::prelude::AnimationEvent::MarkerHit) event will be emitted.
///
/// See [AnimationEvent](crate::prelude::AnimationEvent) for more details.
#[derive(Debug, Clone, Reflect)]
#[reflect(Debug)]
pub struct Clip {
    id: ClipId,

    pub(crate) atlas_indices: Vec<usize>,

    pub(crate) duration: Option<AnimationDuration>,
    pub(crate) repetitions: Option<usize>,
    pub(crate) direction: Option<AnimationDirection>,
    pub(crate) easing: Option<Easing>,

    pub(crate) markers: HashMap<usize, Vec<Marker>>,
}

impl Clip {
    /// Private empty clip constructor.
    ///
    /// Users should not create empty clip.
    /// Our animation builder starts with an empty clip.
    pub(crate) fn empty() -> Self {
        Clip {
            id: ClipId::new(),
            atlas_indices: Vec::new(),
            duration: None,
            repetitions: None,
            direction: None,
            easing: None,
            markers: HashMap::new(),
        }
    }

    /// Unique ID of this clip
    ///
    /// Marker-related [AnimationEvents](crate::prelude::AnimationEvent) will contain this ID.
    pub fn id(&self) -> ClipId {
        self.id
    }

    /// Indices into the layout of a TextureAtlas component
    pub fn atlas_indices(&self) -> &[usize] {
        &self.atlas_indices
    }

    /// The optional duration of this clip
    pub fn duration(&self) -> &Option<AnimationDuration> {
        &self.duration
    }

    /// The optional number of repetitions of this clip
    pub fn repetitions(&self) -> &Option<usize> {
        &self.repetitions
    }

    /// The optional direction of this clip
    pub fn direction(&self) -> &Option<AnimationDirection> {
        &self.direction
    }

    /// The optional easing of this clip
    pub fn easing(&self) -> &Option<Easing> {
        &self.easing
    }

    /// Markers that will trigger [MarkerHit](crate::prelude::AnimationEvent::MarkerHit) events when the corresponding frame is played
    ///
    /// The key is the frame index.
    /// Multiple markers can be associated to the same frame.
    pub fn markers(&self) -> &HashMap<usize, Vec<Marker>> {
        &self.markers
    }
}
