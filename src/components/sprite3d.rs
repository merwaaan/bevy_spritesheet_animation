use bevy::{
    asset::Handle,
    color::Color,
    ecs::prelude::*,
    math::{Rect, Vec2},
    reflect::prelude::*,
    render::{
        texture::Image,
        view::{InheritedVisibility, ViewVisibility, Visibility},
    },
    sprite::{Anchor, TextureAtlas, TextureAtlasLayout},
    transform::components::{GlobalTransform, Transform},
};

/// Specifies the rendering properties of a 3D sprite.
///
/// This contains similar fields as Bevy's [Sprite](bevy::sprite::Sprite).
///
/// This is commonly used as a component within [Sprite3dBundle].
#[derive(Component, Debug, Reflect)]
#[reflect(Component, Debug)]
pub struct Sprite3d {
    /// A color to tint the sprite with.
    ///
    /// The default color is white, which does not tint the sprite.
    pub color: Color,

    /// Flips the sprite horizontally.
    pub flip_x: bool,

    /// Flips the sprite vertically.
    pub flip_y: bool,

    /// The size of the sprite.
    ///
    /// If undefined, the dimensions of the sprite's image will be used.
    pub custom_size: Option<Vec2>,

    /// The position of the sprite's origin
    pub anchor: Anchor,
}

impl Default for Sprite3d {
    fn default() -> Self {
        Sprite3d {
            color: Color::WHITE,
            flip_x: false,
            flip_y: false,
            custom_size: None,
            anchor: Anchor::Center,
        }
    }
}

/// A Bundle of components for drawing a 3D sprite.
///
/// This contains similar fields as Bevy's [Sprite](bevy::sprite::SpriteBundle).
#[derive(Bundle, Default)]
pub struct Sprite3dBundle {
    pub sprite: Sprite3d,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub atlas: TextureAtlas,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

/// A builder to easily instantiate [Sprite3dBundle]s
///
/// # Note
///
/// The geometry and material required for rendering a 3D sprite will be automatically added by the library in an internal system.
///
/// The library requires the sprite's texture to be loaded before setting everything up.
/// If the texture has already been loaded (for example, in a loading stage), the sprite will appear on the next update.
/// Otherwise, the actual rendering will be delayed and the sprite will not be visible during a few frames.
///
/// # Example
///
/// ```
/// # use bevy::{prelude::*, sprite::Anchor};
/// # use bevy_spritesheet_animation::prelude::*;
/// # fn f(mut commands: Commands, texture: Handle<Image>, atlas_layout: Handle<TextureAtlasLayout>) {
/// commands.spawn(
///     Sprite3dBuilder::from_image(texture.clone())
///         .with_atlas(atlas_layout)
///         .with_anchor(Anchor::BottomRight)
///         .build()
/// );
/// # }
/// ```
#[derive(Clone)]
pub struct Sprite3dBuilder {
    sprite_color: Color,
    sprite_flip_x: bool,
    sprite_flip_y: bool,
    sprite_custom_size: Option<Vec2>,
    sprite_rect: Option<Rect>,
    sprite_anchor: Anchor,

    texture: Handle<Image>,
    atlas: Option<TextureAtlas>,
    transform: Option<Transform>,
}

impl Sprite3dBuilder {
    pub fn from_image(texture: Handle<Image>) -> Self {
        Self {
            texture,
            sprite_color: Color::WHITE,
            sprite_flip_x: false,
            sprite_flip_y: false,
            sprite_custom_size: None,
            sprite_rect: None,
            sprite_anchor: Anchor::default(),
            atlas: None,
            transform: None,
        }
    }

    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.sprite_color = color.into();
        self
    }

    pub fn with_flip(mut self, x: bool, y: bool) -> Self {
        self.sprite_flip_x = x;
        self.sprite_flip_y = y;
        self
    }

    pub fn with_custom_size(mut self, size: impl Into<Vec2>) -> Self {
        self.sprite_custom_size = Some(size.into());
        self
    }

    pub fn with_rect(mut self, rect: impl Into<Rect>) -> Self {
        self.sprite_rect = Some(rect.into());
        self
    }

    pub fn with_anchor(mut self, anchor: impl Into<Anchor>) -> Self {
        self.sprite_anchor = anchor.into();
        self
    }

    pub fn with_atlas(mut self, handle: Handle<TextureAtlasLayout>) -> Self {
        self.atlas = Some(TextureAtlas {
            layout: handle,
            index: 0,
        });
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = Some(transform);
        self
    }

    pub fn build(self) -> Sprite3dBundle {
        Sprite3dBundle {
            sprite: Sprite3d {
                color: self.sprite_color,
                flip_x: self.sprite_flip_x,
                flip_y: self.sprite_flip_y,
                custom_size: self.sprite_custom_size,
                anchor: self.sprite_anchor,
            },
            texture: self.texture,
            atlas: self.atlas.unwrap_or_default(),
            transform: self.transform.unwrap_or_default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}
