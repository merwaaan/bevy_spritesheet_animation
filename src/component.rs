use bevy::ecs::component::Component;

use crate::animation::AnimationId;

/// A Bevy component that enables spritesheet animations.
///
/// It contains an [AnimationId] that references an [Animation](crate::prelude::Animation) created with [SpritesheetLibrary::new_animation](crate::prelude::SpritesheetLibrary::new_animation).
///
/// The animation can be paused and resumed by toggling the `playing` field.
///
/// The speed of the animation can be tweaked with `speed_factor`.
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
///     mut library: ResMut<SpritesheetLibrary>,
///     # assets: Res<AssetServer>,
///     # mut layouts: ResMut<Assets<TextureAtlasLayout>>,
/// ) {
///     let animation_id = library.new_animation(|animation| {
///         // ...
///     });
///
///     // ... omitted: load a texture and an atlas layout ...
///     # let texture = assets.load("fake");
///     # let layout = layouts.add(TextureAtlasLayout::new_empty(UVec2::ONE));
///
///     commands.spawn((
///         SpriteSheetBundle {
///             texture: texture.clone(),
///             atlas: TextureAtlas {
///                 layout: layout.clone(),
///                 ..default()
///             },
///             ..default()
///         },
///         SpritesheetAnimation::from_id(animation_id),
///     ));
/// }
/// ```
#[derive(Component, Debug, Clone)]
pub struct SpritesheetAnimation {
    /// The ID of the animation to play
    pub animation_id: AnimationId,

    /// Is the animation currently playing?
    ///
    /// The animation can alternatively be stopped by removing the [SpritesheetAnimation] component from its entity entirely.
    /// However, re-inserting the component at a later stage will restart it from scratch whereas pausing/resuming the animation with `playing` keeps its progress.
    pub playing: bool,

    /// A speed multiplier for the animation, defaults to 1
    pub speed_factor: f32,

    /// Marks the animation to be reset by the animator on the next update
    pub(crate) reset_requested: bool,
}

impl SpritesheetAnimation {
    /// Creates a [SpritesheetAnimation] component from an [AnimationId] returned by [SpritesheetLibrary::new_animation](crate::prelude::SpritesheetLibrary::new_animation).
    ///
    /// # Arguments
    ///
    /// * `animation_id` - the ID of the animation to play
    pub fn from_id(animation_id: AnimationId) -> Self {
        Self {
            animation_id,
            playing: true,
            speed_factor: 1.0,
            reset_requested: false,
        }
    }

    /// Resets the animation to its initial state.
    pub fn reset(&mut self) {
        self.reset_requested = true;
    }
}
