//! This crate provides a [Bevy](https://bevyengine.org/) plugin for animating sprites.
//!
//! # Features
//!
//! - Animate 2D sprites, [3D sprites](crate::prelude::Sprite3d), UI images and cursors! 🎉
//! - [Easily build](crate::prelude::AnimationBuilder) animations from [spritesheets](crate::prelude::Spritesheet) with custom parameters like [duration](crate::prelude::AnimationDuration), [repetitions](crate::prelude::AnimationRepeat), [direction](crate::prelude::AnimationDirection), [easing](crate::prelude::Easing).
//! - Trigger [events](crate::prelude::AnimationEvent) when animations end or reach specific points.
//!
//! # Examples
//!
//! Please check out the [examples](https://github.com/merwaaan/bevy_spritesheet_animation/tree/main/examples) showcasing those different features.
//!
//! # Quick start
//!
//! 1. Add the [SpritesheetAnimationPlugin](crate::prelude::SpritesheetAnimationPlugin) to your app.
//! 2. Create a [Spritesheet](crate::prelude::Spritesheet) and build animations.
//! 3. Register the animations in the `Assets<Animation>` resource.
//! 4. Add a [SpritesheetAnimation](crate::prelude::SpritesheetAnimation) component to your entity.
//!
//! ```no_run (cannot actually execute during CI builds as there are no displays)
#![doc = include_str!("../examples/basic.rs")]
//! ```

pub mod animation;
pub mod builder;
pub mod clip;
pub mod components;
pub mod easing;
pub mod events;
pub mod plugin;
pub mod spritesheet;

pub mod prelude {
    pub use crate::{
        animation::{Animation, AnimationDirection, AnimationDuration, AnimationRepeat},
        builder::AnimationBuilder,
        clip::{Clip, ClipId},
        components::{
            generator::ComponentGenerator,
            sprite3d::Sprite3d,
            spritesheet_animation::{AnimationProgress, SpritesheetAnimation},
        },
        easing::{Easing, EasingVariety},
        events::AnimationEvent,
        events::Marker,
        plugin::SpritesheetAnimationPlugin,
        spritesheet::Spritesheet,
    };
}

mod animator;
mod systems;

const CRATE_NAME: &str = "bevy_spritesheet_animation";
