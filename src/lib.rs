//! This crate is a [Bevy](https://bevyengine.org/) plugin for animating sprites that are backed by spritesheets.
//!
//!# Features
//!
//! - A single Bevy [component](crate::prelude::SpritesheetAnimation) to add to your entities to play animations.
//! - Tunable parameters: [duration](crate::prelude::AnimationDuration), [repetitions](crate::prelude::AnimationRepeat), [direction](crate::prelude::AnimationDirection), [easing](crate::prelude::Easing).
//! - [Composable animations](crate::prelude::Animation) from multiple clips.
//! - [Events](crate::prelude::AnimationEvent) to react to animations ending or reaching specific points.
//! - A [convenient API](crate::prelude::Spritesheet) to select frames in spritesheets.
//!
//! # Quick start
//!
//! 1. Add the [SpritesheetAnimationPlugin](crate::prelude::SpritesheetAnimationPlugin) to your app
//! 2. Use the [SpritesheetLibrary](crate::prelude::SpritesheetLibrary) resource to create new clips and animations
//! 3. Add a [SpritesheetAnimation](crate::prelude::SpritesheetAnimation) component to your entity
//!
//! ```
//! # use bevy::prelude::*;
//! # use bevy_spritesheet_animation::prelude::*;
//! fn main() {
//!     # return; // cannot actually execute this during CI builds as there are no displays
//!     let app = App::new()
//!         .add_plugins(DefaultPlugins)
//!         // Add the plugin to enable animations.
//!         // This makes the SpritesheetLibrary resource available to your systems.
//!         .add_plugins(SpritesheetAnimationPlugin)
//!         .add_systems(Startup, setup);
//!
//!     // ...
//! }
//!
//! fn setup(
//!     mut commands: Commands,
//!     mut library: ResMut<SpritesheetLibrary>,
//!     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
//!     assets: Res<AssetServer>,
//! ) {
//!     // Create an animation
//!
//!     let clip_id = library.new_clip(|clip| {
//!         // You can configure this clip here (duration, number of repetitions, etc...)
//!
//!         // This clip will use all the frames in row 3 of the spritesheet
//!         clip.push_frame_indices(Spritesheet::new(8, 8).row(3));
//!     });
//!
//!     let animation_id = library.new_animation(|animation| {
//!         // You can configure this animation here (duration, number of repetitions, etc...)
//!
//!         animation.add_stage(clip_id.into());
//!
//!         // This is a simple animation with a single clip but we can create more sophisticated
//!         // animations with multiple clips, each one having different parameters.
//!         //
//!         // See the `composition` example for more details.
//!     });
//!
//!     // Spawn a sprite using Bevy's built-in SpriteSheetBundle
//!
//!     let texture = assets.load("character.png");
//!
//!     let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
//!         UVec2::new(96, 96),
//!         8,
//!         8,
//!         None,
//!         None,
//!     ));
//!
//!     commands.spawn((
//!         SpriteSheetBundle {
//!             texture,
//!             atlas: TextureAtlas {
//!                 layout,
//!                 ..default()
//!             },
//!             ..default()
//!         },
//!         // Add a SpritesheetAnimation component that references our newly created animation
//!         SpritesheetAnimation::from_id(animation_id),
//!     ));
//!
//!     commands.spawn(Camera2dBundle::default());
//! }
//! ```

pub mod animation;
pub mod clip;
pub mod component;
pub mod easing;
pub mod events;
pub mod library;
pub mod plugin;
pub mod spritesheet;
pub mod stage;

mod animator;
mod systems;

pub mod prelude {
    pub use super::{
        animation::{
            Animation, AnimationDirection, AnimationDuration, AnimationId, AnimationRepeat,
        },
        clip::{AnimationClip, AnimationClipId},
        component::SpritesheetAnimation,
        easing::{Easing, EasingVariety},
        events::{AnimationEvent, AnimationMarkerId},
        library::SpritesheetLibrary,
        plugin::SpritesheetAnimationPlugin,
        spritesheet::Spritesheet,
        stage::AnimationStage,
    };
}

const CRATE_NAME: &str = "bevy_spritesheet_animation";
