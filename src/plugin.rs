use bevy::app::{App, Plugin, PostUpdate};

use crate::{
    animator::SpritesheetAnimator,
    events::AnimationEvent,
    library::SpritesheetLibrary,
    systems::{
        sprite3d::{
            add_mesh_and_material_to_3d_sprite, delete_outdated_uvs, sync_atlas_layout_uvs,
            sync_mesh_uvs_with_atlas_index, TextureAtlasLayoutUvs,
        },
        spritesheet_animation::play_animations,
    },
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

impl Plugin for SpritesheetAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpritesheetLibrary::new())
            .insert_resource(SpritesheetAnimator::new())
            .insert_resource(TextureAtlasLayoutUvs::default())
            .add_event::<AnimationEvent>()
            .add_systems(PostUpdate, play_animations)
            //
            .add_systems(PostUpdate, add_mesh_and_material_to_3d_sprite)
            .add_systems(PostUpdate, sync_atlas_layout_uvs)
            .add_systems(PostUpdate, sync_mesh_uvs_with_atlas_index)
            .add_systems(PostUpdate, delete_outdated_uvs);
    }
}
