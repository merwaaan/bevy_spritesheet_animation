use bevy::{prelude::*, sprite::Anchor};

/// A 3D sprite, animated or not.
///
/// This component is intended to be used as a drop-in replacement for Bevy's standard [Sprite](https://docs.rs/bevy/latest/bevy/sprite/struct.Sprite.html).
///
/// # Example
///
/// Use the [Spritesheet's sprite3d()](crate::prelude::ComponentGenerator::sprite3d) to create an animation-ready 3D sprite.
///
/// Optionally insert a [SpritesheetAnimation](crate::prelude::SpritesheetAnimation) component to animate the 3D sprite.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// fn create_animated_sprite(
///     mut commands: Commands,
///     mut animations: ResMut<Assets<Animation>>,
///     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
///     # spritesheet: &Spritesheet,
///     # animation: Handle<Animation>
/// ) {
///     // ...omitted: create a spritesheet and an animation
///     //
///     // The animation is optional, static 3D sprite are supported too
///
///     let sprite3d = spritesheet
///         .with_size_hint(600, 400)
///         .sprite3d(&mut atlas_layouts)
///         .with_color(LinearRgba::RED)
///         .with_flip(false, true)
///         .with_double_sided(true);
///
///     commands.spawn((
///         sprite3d,
///         SpritesheetAnimation::new(animation),
///     ));
/// }
/// ```
///
/// # Required components
///
/// The geometry and material required for rendering a 3D sprite will be automatically added by the plugin.
///
/// It requires the sprite's image to be loaded before setting everything up:
/// - If the image has already been loaded (for example, at a separate loading stage), the sprite will immediately appear.
/// - Otherwise, rendering will be delayed and the sprite will not be visible during a few frames.
#[derive(Component, Debug, Reflect)]
#[require(Transform, Visibility)]
#[reflect(Component, Debug)]
pub struct Sprite3d {
    /// The sprite's image (should be a spritesheet when animated)
    pub image: Handle<Image>,

    /// The texture atlas used to render only part of the image
    pub texture_atlas: Option<TextureAtlas>,

    /// A color to tint the sprite with (default = `Color::WHITE`).
    pub color: Color,

    /// Flips the sprite horizontally (default = `false`).
    pub flip_x: bool,

    /// Flips the sprite vertically (default = `false`).
    pub flip_y: bool,

    /// The size of the sprite (default = `None`).
    ///
    /// If not specified, the dimensions of the sprite's image will be used.
    pub custom_size: Option<Vec2>,

    /// The position of the sprite's origin (default = `Anchor::CENTER`).
    pub anchor: Anchor,

    /// The sprite's alpha mode (default = `AlphaMode::Mask(0.5)`).
    ///
    /// Use other modes to achieve custom blending effects.
    /// A common choice is `AlphaMode::Blend`, which makes pixels partially transparent depending on the color's alpha.
    pub alpha_mode: AlphaMode,

    /// Whether the sprite should be unlit (default = `true`).
    pub unlit: bool, // TODO change to lit?

    /// An emissive color, if the sprite emits light (default = `Color::BLACK`).
    pub emissive: LinearRgba,

    /// Whether the sprite should be rendered as double-sided (default = `false`).
    pub double_sided: bool,
}

impl Default for Sprite3d {
    fn default() -> Self {
        Self {
            image: default(),
            texture_atlas: default(),
            color: default(),
            flip_x: default(),
            flip_y: default(),
            custom_size: default(),
            anchor: default(),
            alpha_mode: AlphaMode::Mask(0.5),
            unlit: true,
            emissive: Color::BLACK.into(),
            double_sided: false,
        }
    }
}

impl Sprite3d {
    pub fn from_image(image: Handle<Image>) -> Self {
        Self { image, ..default() }
    }

    pub fn from_atlas_image(image: Handle<Image>, atlas: TextureAtlas) -> Self {
        Self {
            image,
            texture_atlas: Some(atlas),
            ..default()
        }
    }

    pub fn with_alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.alpha_mode = alpha_mode;
        self
    }

    pub fn with_unlit(mut self, unlit: bool) -> Self {
        self.unlit = unlit;
        self
    }

    pub fn with_emissive(mut self, emissive: LinearRgba) -> Self {
        self.emissive = emissive;
        self
    }

    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }

    pub fn with_flip(mut self, x: bool, y: bool) -> Self {
        self.flip_x = x;
        self.flip_y = y;
        self
    }

    pub fn with_custom_size(mut self, size: impl Into<Vec2>) -> Self {
        self.custom_size = Some(size.into());
        self
    }

    pub fn with_anchor(mut self, anchor: impl Into<Anchor>) -> Self {
        self.anchor = anchor.into();
        self
    }

    pub fn with_double_sided(mut self, ds: bool) -> Self {
        self.double_sided = ds;
        self
    }
}
