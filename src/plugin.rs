use bevy::{
    app::{App, Plugin, PostUpdate},
    prelude::{IntoSystemConfigs, SystemSet},
};

use crate::{
    animator::Animator,
    events::AnimationEvent,
    library::AnimationLibrary,
    systems::{sprite3d, spritesheet_animation},
};

/// Set for systems that update the animation state.
#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub struct AnimationSystemSet;

/// Set for systems that update the sprite state.
#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub struct Sprite3dSystemSet;

/// The spritesheet animation plugin to add to Bevy apps.
///
/// This plugin injects the systems required for running animations and inserts the [AnimationLibrary] resource with which you can create new clips and animations.
///
/// # Examples
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// # return; // cannot actually execute this during CI builds as there are no displays
/// let app = App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(SpritesheetAnimationPlugin::default());
///
/// // ...
/// ```
///
/// Adding the plugin to a Bevy app makes the [AnimationLibrary] available as a resource:
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// fn my_system(mut library: ResMut<AnimationLibrary>) {
///     let clip = Clip::from_frames([1, 2, 3]);
///     let clip_id = library.register_clip(clip);
///
///     let animation = Animation::from_clip(clip_id);
///     let animation_id = library.register_animation(animation);
///
///     // ...
/// }
/// ```
pub struct SpritesheetAnimationPlugin {
    /// Determines whether to run 3D-related systems.
    ///
    /// This allows using the plugin without `bevy_render`, for example in a headless environment with `MinimalPlugin`.
    pub enable_3d: bool,
}

impl Plugin for SpritesheetAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            // The animation library, for creating clips, animations and markers
            .init_resource::<AnimationLibrary>()
            .register_type::<AnimationLibrary>()
            // The animator responsible for running animations
            .init_resource::<Animator>()
            .register_type::<Animator>()
            // Animations events
            .add_event::<AnimationEvent>()
            // Systems
            .add_systems(
                PostUpdate,
                // Main animation system
                spritesheet_animation::play_animations.in_set(AnimationSystemSet),
            );

        if self.enable_3d {
            app
                // Cache for 3D sprites
                .init_resource::<sprite3d::Cache>()
                .register_type::<sprite3d::Cache>()
                // 3D sprite systems
                .add_systems(
                    PostUpdate,
                    (
                        sprite3d::setup_rendering,
                        sprite3d::sync_when_sprites_change,
                        sprite3d::sync_when_atlases_change,
                        sprite3d::remove_dropped_standard_materials,
                    )
                        .in_set(Sprite3dSystemSet)
                        .after(AnimationSystemSet),
                );
        }
    }
}

impl Default for SpritesheetAnimationPlugin {
    fn default() -> Self {
        Self { enable_3d: true }
    }
}
