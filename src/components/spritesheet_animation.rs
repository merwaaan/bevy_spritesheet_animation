use bevy::prelude::*;

use crate::animation::Animation;

/// The progress of an animation being played.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Default, PartialEq, Hash)]
pub struct AnimationProgress {
    /// The index of the current frame of the animation.
    ///
    /// This is an absolute index within the whole animation and it's unrelated to the clips that compose it internally.
    ///
    /// This value wraps around for each repetition of the animation.
    ///
    /// For instance, a 3-frame animation repeated twice will give:
    /// - `frame`     : 0 → 1 → 2 → 0 → 1 → 2
    pub frame: usize,

    /// The current repetition of the animation.
    ///
    /// For instance, a 3-frame animation repeated twice will give:
    /// - `repetition`: 0 → 0 → 0 → 1 → 1 → 1
    pub repetition: usize,
}

impl AnimationProgress {
    pub fn with_frame(frame: usize) -> Self {
        Self {
            frame,
            repetition: 0,
        }
    }

    pub fn with_frame_repetition(frame: usize, repetition: usize) -> Self {
        Self { frame, repetition }
    }
}

/// A Bevy component that enables spritesheet animations.
///
/// It references an [Animation] and contains playback-related attributes.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// fn create_animated_sprite(
///     mut commands: Commands,
///     mut animations: ResMut<Assets<Animation>>,
///     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
///     # spritesheet: &Spritesheet,
///     # animation: Handle<Animation>
/// ) {
///     // ...omitted: create a spritesheet and an animation
///
///     let sprite = spritesheet
///         .with_size_hint(600, 400)
///         .sprite(&mut atlas_layouts);
///
///     commands.spawn((
///         sprite,
///         SpritesheetAnimation::new(animation),
///     ));
/// }
/// ```
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Debug)]
pub struct SpritesheetAnimation {
    /// The animation to play
    ///
    /// To change the current animation, it's best to call [SpritesheetAnimation::switch], which will set a new `animation` and also reset the `frame` and `repetition` indices, as changing the animation without adjusting the indices can lead to unexpected results if the animations' lengths don't match.
    ///
    /// However, directly updating the `animation` field can be useful in specific cases, such as when working with animation variants that must resume from the same frame.
    pub animation: Handle<Animation>,

    /// The current progress of the animation
    ///
    /// This can be both read from and written to to control the animation.
    pub progress: AnimationProgress,

    /// Is the animation currently playing?
    ///
    /// Alternatively, the animation can be stopped by removing the [SpritesheetAnimation] component from its entity entirely.
    /// However, re-inserting the component at a later time will restart it from scratch whereas pausing/resuming the animation with `playing` keeps its progress.
    pub playing: bool,

    /// A speed multiplier for the animation (default = `1`)
    pub speed_factor: f32,
}

impl SpritesheetAnimation {
    /// Creates a [SpritesheetAnimation] component.
    ///
    /// # Arguments
    ///
    /// - `animation` - the handle of the animation to play
    pub fn new(animation: Handle<Animation>) -> Self {
        Self {
            animation,
            progress: AnimationProgress {
                frame: 0,
                repetition: 0,
            },
            playing: true,
            speed_factor: 1.0,
        }
    }

    pub fn with_progress(mut self, progress: AnimationProgress) -> Self {
        self.progress = progress;
        self
    }

    pub fn with_playing(mut self, playing: bool) -> Self {
        self.playing = playing;
        self
    }

    pub fn with_speed_factor(mut self, speed_factor: f32) -> Self {
        self.speed_factor = speed_factor;
        self
    }

    /// Resumes the animation.
    pub fn play(&mut self) {
        self.playing = true;
    }

    /// Pauses the animation.
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// Resets the animation to its first frame.
    pub fn reset(&mut self) {
        self.progress.frame = 0;
        self.progress.repetition = 0;
    }

    /// Switches to a different animation.
    pub fn switch(&mut self, animation: Handle<Animation>) {
        self.animation = animation;
        self.reset();
    }
}
