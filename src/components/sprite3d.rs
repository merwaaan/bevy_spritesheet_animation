use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, component::Component},
    math::{Rect, Vec2},
    render::{
        color::Color,
        texture::Image,
        view::{InheritedVisibility, ViewVisibility, Visibility},
    },
    sprite::{Anchor, TextureAtlas, TextureAtlasLayout},
    transform::components::{GlobalTransform, Transform},
};

/// props from https://docs.rs/bevy/latest/bevy/sprite/struct.Sprite.html
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
    pub image: Handle<Image>,
    pub atlas: TextureAtlas,
    pub transform: Transform,
    //pub mesh: Handle<Mesh>, // TODO ???
    //pub material: Handle<StandardMaterial>, // TODO ???
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

pub struct Sprite3DBuilder {
    image: Handle<Image>,

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
    pub fn from_image(image: Handle<Image>) -> Self {
        Self {
            image,
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
        self.sprite_anchor = anchor;
        self
    }

    pub fn with_atlas(mut self, layout_handle: Handle<TextureAtlasLayout>) -> Self {
        // TODO index? opt?
        self.atlas = Some(TextureAtlas {
            layout: layout_handle,
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
            image: self.image,
            atlas: self.atlas.unwrap_or_default(),
            transform: self.transform.unwrap_or_default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

// pub struct Sprite3DBundleBuilder {
//     image: Handle<Image>,
//     size: Option<SpriteSize>,
//     transform: Option<Transform>,
//     atlas_layout: Option<Handle<TextureAtlasLayout>>,
// }

// impl Sprite3DBundleBuilder {
//     pub fn from_image(image: Handle<Image>) -> Self {
//         Self {
//             image,
//             size: None,
//             transform: None,
//             atlas_layout: None,
//         }
//     }

//     // pub fn with_size(mut self, size: SpriteSize) -> Self {
//     //     self.size = Some(size);
//     //     self
//     // }

//     // pub fn with_transform(mut self, transform: Transform) -> Self {
//     //     self.transform = Some(transform);
//     //     self
//     // }

//     // pub fn with_atlas_layout(mut self, layout: Handle<TextureAtlasLayout>) -> Self {
//     //     self.atlas_layout = Some(layout);
//     //     self
//     // }

//     // pub fn spawn<'a>(
//     //     self,
//     //     commands: &'a mut Commands,
//     //     params: &mut Sprites3dParams,
//     // ) -> EntityCommands<'a> {
//     //     // Use the sprite's image as the material's texture

//     //     let material = params.materials.add(StandardMaterial {
//     //         base_color_texture: Some(self.image.clone()),
//     //         unlit: true,
//     //         alpha_mode: self.alpha_mode.unwrap_or(AlphaMode::Blend),
//     //         ..default()
//     //     });

//     //     // Spawn the entity

//     //     let size = self.size.unwrap_or(SpriteSize::Scaled(1.0));

//     //     let mut entity_commands = commands.spawn((
//     //         // Marker
//     //         Sprite3D { size },
//     //         // Same as PBRBundle but we don't use this bundle directly to be able to not add a mesh yet if we're waiting for the image to load
//     //         material,
//     //         self.transform.unwrap_or(Transform::default()),
//     //         GlobalTransform::default(),
//     //         Visibility::default(),
//     //         InheritedVisibility::default(),
//     //         ViewVisibility::default(),
//     //     ));

//     //     // Add a texture atlas if a layout has been provided

//     //     let mut maybe_uvs: Option<Vec<QuadUvs>> = None;

//     //     if let Some(atlas_layout_handle) = &self.atlas_layout {
//     //         if let Some(atlas_layout) = params.atlas_layouts.get(atlas_layout_handle) {
//     //             // Generate the UV coordinates and cache them

//     //             let mesh_uvs = create_uvs_from_atlas_layout(&atlas_layout); // TODO reuse?

//     //             params
//     //                 .atlas_layout_uvs
//     //                 .add_uvs(atlas_layout_handle.clone_weak(), mesh_uvs.clone());

//     //             // Add a texture atlas to the entity

//     //             entity_commands.insert(TextureAtlas {
//     //                 layout: atlas_layout_handle.clone(),
//     //                 index: 0,
//     //             });

//     //             maybe_uvs = Some(mesh_uvs);
//     //         }
//     //     }

//     //     // Immediately add a quad mesh if the image is already loaded or the sprite does not require it.
//     //     // Otherwise, it will be done by a system when the image becomes available.

//     //     let size_info_if_ready = match size {
//     //         SpriteSize::Fixed(size) => Some(SizeInfo::FixedSize(size)),
//     //         SpriteSize::Scaled(size) => {
//     //             if let Some(image) = params.images.get(self.image) {
//     //                 Some(SizeInfo::ScaledSizeAndImage(size, image))
//     //             } else {
//     //                 None
//     //             }
//     //         }
//     //     };

//     //     if let Some(size_info) = size_info_if_ready {
//     //         let maybe_first_uvs = maybe_uvs.as_ref().and_then(|quads| quads.get(0));

//     //         let mesh = create_mesh(size_info, maybe_first_uvs);

//     //         entity_commands.insert(params.meshes.add(mesh));
//     //     }

//     //     entity_commands
//     // }
// }
