use bevy::{asset::Handle, ecs::prelude::*, reflect::prelude::*};

use crate::animation::Animation;

/// The progress of an animation being played
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Default, PartialEq, Hash)]
pub struct AnimationProgress {
    /// The index of the active frame of the animation
    ///
    /// This is an absolute index within the whole animation and it's unrelated to the clips that compose it internally.
    ///
    /// This value wraps around for each repetition of the animation.
    /// For instance, a 3-frame animation repeated twice will give:
    /// `frame`     : 0 → 1 → 2 → 0 → 1 → 2
    /// `repetition`: 0 → 0 → 0 → 1 → 1 → 1
    pub frame: usize,

    /// The current repetition of the animation
    pub repetition: usize,
}

/// A Bevy component that enables spritesheet animations.
///
/// It references an [Animation](crate::prelude::Animation) and contains a few playback-related attributes.
///
/// # Note
///
/// For this component to take effect, the entity must also have a Bevy [TextureAtlas](bevy::prelude::TextureAtlas) component.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// fn my_system(
///     mut commands: Commands,
///     # assets: Res<AssetServer>,
///     # mut layouts: ResMut<Assets<TextureAtlasLayout>>,
///     mut animations: ResMut<Assets<Animation>>,
/// ) {
///     let clip = Clip::from_frames([1, 2, 3]);
///
///     let animation = Animation::from_clip(clip);
///
///     let animation_handle = animations.add(animation);
///
///     // ... omitted: load a texture and an atlas layout ...
///     # let image = assets.load("");
///     # let atlas = TextureAtlas {
///     #    layout: layouts.add(TextureAtlasLayout::new_empty(UVec2::ONE)),
///     #    ..default()
///     # };
///
///     commands.spawn((
///         Sprite::from_atlas_image(image, atlas),
///         SpritesheetAnimation::new(animation_handle),
///     ));
/// }
/// ```
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Debug)]
pub struct SpritesheetAnimation {
    /// The animation to play
    ///
    /// # Note
    ///
    /// In most cases, it's best to call [SpritesheetAnimation::switch], which will set a new `animation` and also reset the `frame` and `repetition` indices, as changing the animation without adjusting the indices can lead to unexpected results.
    ///
    /// However, only updating the `animation` can be useful in specific cases, such as when working with animation variants that must resume from the same frame.
    pub animation: Handle<Animation>,

    /// The current progress of the animation
    ///
    /// This can be both read from and written to.
    pub progress: AnimationProgress,

    /// Is the animation currently playing?
    ///
    /// Alternatively, the animation can be stopped by removing the [SpritesheetAnimation] component from its entity entirely.
    /// However, re-inserting the component at a later time will restart it from scratch whereas pausing/resuming the animation with `playing` keeps its progress.
    pub playing: bool,

    /// A speed multiplier for the animation, defaults to 1
    pub speed_factor: f32,
}

impl SpritesheetAnimation {
    /// Creates a [SpritesheetAnimation] component.
    ///
    /// # Arguments
    ///
    /// * `animation` - the handle of the animation to play
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

    /// Switches to a different animation.
    ///
    /// # Note
    ///
    /// To change the animation while keeping the current `frame` and `repetition` indices, directly update the `animation` field instead.
    pub fn switch(&mut self, animation: Handle<Animation>) {
        self.animation = animation;
        self.reset();
    }

    /// Resets the animation to its initial state.
    pub fn reset(&mut self) {
        self.progress.frame = 0;
        self.progress.repetition = 0;
    }
}
