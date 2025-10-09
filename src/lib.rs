//! This crate is a [Bevy](https://bevyengine.org/) plugin for animating sprites that are backed by spritesheets.
//!
//!# Features
//!
//! - Animate 2D and [3D sprites](crate::prelude::Sprite3d)! 🎉
//! - A single Bevy [component](crate::prelude::SpritesheetAnimation) to add to your entities to play animations.
//! - Tunable parameters: [duration](crate::prelude::AnimationDuration), [repetitions](crate::prelude::AnimationRepeat), [direction](crate::prelude::AnimationDirection), [easing](crate::prelude::Easing).
//! - [Composable animations](crate::prelude::Animation) from multiple clips.
//! - [Events](crate::prelude::AnimationEvent) to react to animations ending or reaching specific points.
//! - A [convenient API](crate::prelude::Spritesheet) to select frames in spritesheets.
//!
//! # Quick start
//!
//! 1. Add the [SpritesheetAnimationPlugin](crate::prelude::SpritesheetAnimationPlugin) to your app
//! 2. Use the [AnimationLibrary](crate::prelude::AnimationLibrary) resource to create new clips and animations
//! 3. Add a [SpritesheetAnimation](crate::prelude::SpritesheetAnimation) component to your entity
//!
//! ```no_run (cannot actually execute this during CI builds as there are no displays)
//! use bevy::prelude::*;
//! use bevy_spritesheet_animation::prelude::*;
//!
//! fn main() {
//!     let app = App::new()
//!         .add_plugins(DefaultPlugins)
//!         // Add the plugin to enable animations.
//!         // This makes the AnimationLibrary resource available to your systems.
//!         .add_plugins(SpritesheetAnimationPlugin)
//!         .add_systems(Startup, setup);
//!
//!     // ...
//! }
//!
//! fn setup(
//!     mut commands: Commands,
//!     mut library: ResMut<AnimationLibrary>,
//!     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
//!     assets: Res<AssetServer>,
//! ) {
//!     // Create a clip that references some frames from a spritesheet
//!
//!     let spritesheet = Spritesheet::new(8, 8);
//!
//!     let clip = Clip::from_frames(spritesheet.row(3));
//!
//!     let clip_id = library.register_clip(clip);
//!
//!     // Create an animation that uses the clip
//!
//!     let animation = Animation::from_clip(clip_id);
//!
//!     let animation_id = library.register_animation(animation);
//!
//!     // This is a simple animation made of a single clip but we can create more sophisticated
//!     // animations with multiple clips, each one having different parameters.
//!     //
//!     // See the `composition` example for more details.
//!
//!     // Spawn an animated sprite with a SpritesheetAnimation component that references our animation
//!
//!     let image = assets.load("character.png");
//!
//!     let atlas = TextureAtlas {
//!         layout:atlas_layouts.add(spritesheet.atlas_layout(96, 96)),
//!         ..default()
//!     };
//!
//!     commands.spawn((
//!         Sprite::from_atlas_image(image, atlas),
//!         SpritesheetAnimation::from_id(animation_id),
//!     ));
//!
//!     commands.spawn(Camera2d);
//! }
//! ```

pub mod animation;
pub mod animator;
pub mod clip;
pub mod components;
pub mod easing;
pub mod events;
pub mod library;
pub mod plugin;
pub mod spritesheet;

mod systems;

pub mod prelude {
    pub use super::{
        animation::{
            Animation, AnimationDirection, AnimationDuration, AnimationId, AnimationRepeat,
        },
        clip::{Clip, ClipId},
        components::{sprite3d::Sprite3d, spritesheet_animation::SpritesheetAnimation},
        easing::{Easing, EasingVariety},
        events::{AnimationEvent, AnimationMarkerId},
        library::{AnimationLibrary, LibraryError},
        plugin::SpritesheetAnimationPlugin,
        spritesheet::Spritesheet,
    };
}

const CRATE_NAME: &str = "bevy_spritesheet_animation";
