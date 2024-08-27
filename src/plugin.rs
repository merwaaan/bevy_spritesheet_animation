use bevy::{
    app::{App, Plugin, PostUpdate},
    prelude::{IntoSystemConfigs, SystemSet},
};

use crate::{
    animator::SpritesheetAnimator,
    events::AnimationEvent,
    library::SpritesheetLibrary,
    systems::{sprite3d, spritesheet_animation},
};

/// The spritesheet animation plugin to add to Bevy apps.
///
/// This plugin injects the systems required for running animations and inserts the [SpritesheetLibrary] resource with which you can create new clips and animations.
///
/// # Examples
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// # return; // cannot actually execute this during CI builds as there are no displays
/// let app = App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(SpritesheetAnimationPlugin);
///
/// // ...
/// ```
///
/// Adding the plugin to a Bevy app makes the [SpritesheetLibrary] available as a resource:
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// fn my_system(mut library: ResMut<SpritesheetLibrary>) {
///     let clip_id = library.new_clip(|clip| {
///         // ...
///     });
///
///     let animation_id = library.new_animation(|animation| {
///         // ...
///     });
///
///     // ...
/// }
/// ```
pub struct SpritesheetAnimationPlugin;

/// Label for systems that update the animation state.
#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub struct AnimationSystem;

/// Label for systems that update the sprite state.
#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub struct Sprite3dSystem;

impl Plugin for SpritesheetAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            // The spritesheet library, for creating clips, animations and markers
            .insert_resource(SpritesheetLibrary::new())
            // The animator responsible for running animations
            .insert_resource(SpritesheetAnimator::new())
            // Atlas UVs for 3D sprites
            .insert_resource(sprite3d::TextureAtlasLayoutUvs::default())
            // Animations events
            .add_event::<AnimationEvent>()
            // Systems
            .add_systems(
                PostUpdate,
                (
                    // Main animation system
                    spritesheet_animation::play_animations.in_set(AnimationSystem),
                    // 3D sprite systems
                    (
                        sprite3d::setup_rendering,
                        sprite3d::sync_sprites_with_component,
                        sprite3d::sync_sprites_with_atlas,
                    )
                        .in_set(Sprite3dSystem)
                        .after(AnimationSystem),
                ),
            );
    }
}
