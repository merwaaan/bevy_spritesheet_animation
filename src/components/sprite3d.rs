use bevy::{
    asset::Handle,
    color::Color,
    ecs::{bundle::Bundle, component::Component},
    math::{Rect, Vec2},
    render::{
        texture::Image,
        view::{InheritedVisibility, ViewVisibility, Visibility},
    },
    sprite::{Anchor, TextureAtlas, TextureAtlasLayout},
    transform::components::{GlobalTransform, Transform},
};

/// Same props as Bevy's Sprite (https://docs.rs/bevy/latest/bevy/sprite/struct.Sprite.html)
#[derive(Component, Default)]
pub struct Sprite3D {
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
    pub custom_size: Option<Vec2>,
    pub rect: Option<Rect>,
    pub anchor: Anchor,
}

#[derive(Bundle, Default)]
pub struct Sprite3DBundle {
    pub sprite: Sprite3D,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub atlas: TextureAtlas,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    //pub mesh: Handle<Mesh>, // TODO ???
    //pub material: Handle<StandardMaterial>, // TODO ???
}

#[derive(Clone)]
pub struct Sprite3DBuilder {
    texture: Handle<Image>,

    sprite_color: Color,
    sprite_flip_x: bool,
    sprite_flip_y: bool,
    sprite_custom_size: Option<Vec2>,
    sprite_rect: Option<Rect>,
    sprite_anchor: Anchor,

    atlas: Option<TextureAtlas>,
    transform: Option<Transform>,
}

impl Sprite3DBuilder {
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

    pub fn with_color(mut self, color: Color) -> Self {
        // TODO into
        self.sprite_color = color;
        self
    }

    pub fn with_flip(mut self, x: bool, y: bool) -> Self {
        self.sprite_flip_x = x;
        self.sprite_flip_y = y;
        self
    }

    pub fn with_custom_size(mut self, size: Vec2) -> Self {
        // TODO into
        self.sprite_custom_size = Some(size);
        self
    }

    pub fn with_rect(mut self, rect: Rect) -> Self {
        // TODO into
        self.sprite_rect = Some(rect);
        self
    }

    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        // TODO into
        self.sprite_anchor = anchor;
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

    pub fn build(self) -> Sprite3DBundle {
        Sprite3DBundle {
            sprite: Sprite3D {
                color: self.sprite_color,
                flip_x: self.sprite_flip_x,
                flip_y: self.sprite_flip_y,
                custom_size: self.sprite_custom_size,
                rect: self.sprite_rect,
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
