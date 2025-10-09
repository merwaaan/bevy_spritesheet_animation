use bevy::prelude::*;

use crate::{
    animation::Animation,
    animator::Animator,
    components::{sprite3d::Sprite3d, spritesheet_animation::SpritesheetAnimation},
    events::AnimationEvent,
    systems::{sprite3d, spritesheet_animation},
};

/// Set for systems that update animations
#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub struct AnimationSystemSet;

/// Set for systems that manage 3D sprites
#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub struct Sprite3dSystemSet;

/// The spritesheet animation plugin to add to Bevy apps.
///
/// This plugin injects the systems required for running animations and inserts the `Assets<Animation>` resource through which you can create new animations.
///
/// # Examples
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// fn create_app() {
///     let app = App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(SpritesheetAnimationPlugin);
///
///     // ...
/// }
///
/// fn create_animated_sprite(
///     assets: Res<AssetServer>,
///     mut animations: ResMut<Assets<Animation>>
/// ) {
///     let image = assets.load("character.png");
///
///     let animation = Spritesheet::new(&image, 8, 4)
///         .create_animation()
///         .add_indices([1, 2, 3])
///         .build();
///
///     let animation_handle = animations.add(animation);
///
///     // ...
/// }
/// ```
#[derive(Default)]
pub struct SpritesheetAnimationPlugin;

impl Plugin for SpritesheetAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register animation assets
            // TODO other types?
            .init_asset::<Animation>()
            .register_type::<Animation>()
            .register_type::<SpritesheetAnimation>()
            // Main animation system
            .init_resource::<Animator>()
            .register_type::<Animator>()
            .add_systems(
                PostUpdate,
                spritesheet_animation::play_animations.in_set(AnimationSystemSet),
            )
            // Animations events
            .add_message::<AnimationEvent>()
            // 3D sprites
            .init_resource::<sprite3d::Cache>()
            .register_type::<sprite3d::Cache>()
            .register_type::<Sprite3d>()
            .add_systems(
                PostUpdate,
                (
                    sprite3d::setup_rendering,
                    sprite3d::sync_when_sprites_change,
                    sprite3d::sync_when_atlases_change,
                )
                    .in_set(Sprite3dSystemSet)
                    .after(AnimationSystemSet),
            );
    }
}
