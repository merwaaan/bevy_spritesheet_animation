//! This crate is a [Bevy](https://bevyengine.org/) plugin for animating sprites.
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
//! 2. Use the `Assets<[Animation](crate::prelude::Animation)>` resource to register new animations
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
//!         // This makes the Assets<Animation> resource available to your systems.
//!         .add_plugins(SpritesheetAnimationPlugin)
//!         .add_systems(Startup, setup);
//!
//!     // ...
//! }
//!
//! fn setup(
//!     mut commands: Commands,
//!     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
//!     mut animations: ResMut<Assets<Animation>>,
//!     assets: Res<AssetServer>,
//! ) {
//!     // Create a clip from a row of an 8x8 spritesheet
//!
//!     let spritesheet = Spritesheet::new(8, 8);
//!
//!     let clip = Clip::from_frames(spritesheet.row(3))
//!         .with_duration(AnimationDuration::PerFrame(150));
//!
//!     // Create an animation from this clip
//!     //
//!     // This is a simple animation made of a single clip but we can create more sophisticated
//!     // animations with multiple clips, each one having different parameters.
//!     //
//!     // See the `composition` example for more details.
//!
//!     let animation = Animation::from_clip(clip);
//!
//!     let animation_handle = animations.add(animation);
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
//!         // This is Bevy's built-in sprite
//!         Sprite::from_atlas_image(image, atlas),
//!
//!         // This is the component provided by this crate that will animate the sprite
//!         SpritesheetAnimation::new(animation_handle),
//!     ));
//!
//!     commands.spawn(Camera2d);
//! }
//! ```

pub mod animation;
pub mod clip;
pub mod components;
pub mod easing;
pub mod events;
pub mod plugin;
pub mod spritesheet;

pub mod prelude {
    pub use crate::{
        animation::{Animation, AnimationDirection, AnimationDuration, AnimationRepeat},
        animation_set,
        clip::{Clip, ClipId, MarkerId},
        components::{sprite3d::Sprite3d, spritesheet_animation::SpritesheetAnimation},
        easing::{Easing, EasingVariety},
        events::AnimationEvent,
        plugin::SpritesheetAnimationPlugin,
        spritesheet::Spritesheet,
    };
}

mod animator;
mod macros;
mod systems;

const CRATE_NAME: &str = "bevy_spritesheet_animation";
