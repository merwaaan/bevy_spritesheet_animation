use std::{collections::HashMap, hash::Hash};

use bevy::{
    asset::{Assets, Handle},
    ecs::{
        entity::Entity,
        query::Changed,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    pbr::StandardMaterial,
    prelude::*,
    render::{
        alpha::AlphaMode,
        mesh::{Mesh, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        texture::Image,
    },
    sprite::{TextureAtlas, TextureAtlasLayout},
};

use crate::components::sprite3d::Sprite3d;

/// Cached data for the 3D sprites
#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource, Debug, Default)]
pub struct Cache {
    /// Materials used by 3D sprites.
    ///
    /// Shared when the image and color are the same.
    materials: HashMap<MaterialId, Handle<StandardMaterial>>,

    /// Meshes used by the 3D sprites.
    ///
    /// Shared when the size, flips and atlas are the same.
    meshes: HashMap<MeshId, Handle<Mesh>>,
}

/// Uniquely identifies a sprite material
#[derive(Debug, Hash, PartialEq, Eq, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
struct MaterialId {
    image: Handle<Image>,
    color: u32,
}

impl MaterialId {
    fn new(sprite: &Sprite3d, image_handle: &Handle<Image>) -> Self {
        Self {
            image: image_handle.clone_weak(),
            color: sprite.color.to_linear().as_u32(),
        }
    }
}

/// Uniquely identifies a sprite mesh
#[derive(Debug, Hash, PartialEq, Eq, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
struct MeshId {
    sprite_custom_size: [u32; 2],
    sprite_anchor: [u32; 2],
    sprite_flip_x: bool,
    sprite_flip_y: bool,
    image_size: UVec2,
    atlas_rect: URect,
}

impl MeshId {
    fn new(sprite: &Sprite3d, image: &Image, atlas_rect: &URect) -> Self {
        let sprite_custom_size = sprite
            .custom_size
            .map_or([0, 0], |size| [size.x.to_bits(), size.y.to_bits()]);

        let sprite_anchor_vec = sprite.anchor.as_vec();
        let sprite_anchor = [sprite_anchor_vec.x.to_bits(), sprite_anchor_vec.y.to_bits()];

        Self {
            sprite_custom_size,
            sprite_anchor,
            sprite_flip_x: sprite.flip_x,
            sprite_flip_y: sprite.flip_y,
            image_size: image.size(),
            atlas_rect: *atlas_rect,
        }
    }
}

/// Setups 3D sprites for rendering by attaching the 3D geometry and materials to display them.
pub fn setup_rendering(
    mut commands: Commands,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    images: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cache: ResMut<Cache>,
    sprites: Query<
        (
            Entity,
            &Sprite3d,
            &TextureAtlas,
            &Handle<Image>,
            Option<&Handle<Mesh>>,
            Option<&Handle<StandardMaterial>>,
        ),
        Or<(Without<Handle<Mesh>>, Without<Handle<StandardMaterial>>)>,
    >,
) {
    for (entity, sprite, atlas, image_handle, maybe_mesh_handle, maybe_material_handle) in &sprites
    {
        // Add a mesh to the entity if it does not have one yet

        if let Some(image) = images.get(image_handle) {
            // (we have to wait for the image to be loaded to access its dimensions)

            if maybe_mesh_handle.is_none() {
                let mesh_handle = get_or_create_mesh(
                    sprite,
                    image,
                    atlas,
                    &atlas_layouts,
                    &mut meshes,
                    &mut cache,
                );

                commands.entity(entity).insert(mesh_handle);
            }
        }

        // Add a material to the entity if it does not have one yet

        if maybe_material_handle.is_none() {
            // let material_handle =
            //     get_or_create_material(image_handle, &sprite.color, &mut cache, &mut materials);

            // commands.entity(entity).insert(material_handle);

            let material = StandardMaterial {
                base_color_texture: Some(image_handle.clone()),
                base_color: sprite.color,
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            };

            commands.entity(entity).insert(materials.add(material));
        }
    }
}

/// Synchronizes 3D sprites when their Sprite3D gets updated.
pub fn sync_when_sprites_change(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cache: ResMut<Cache>,
    sprites: Query<
        (
            Entity,
            &Sprite3d,
            &TextureAtlas,
            &Handle<Image>,
            &Handle<Mesh>,
            &Handle<StandardMaterial>,
        ),
        Changed<Sprite3d>,
    >,
) {
    for (entity, sprite, atlas, image_handle, mesh_handle, material_handle) in &sprites {
        // Update the mesh if it changed

        if let Some(image) = images.get(image_handle) {
            let new_mesh_handle = get_or_create_mesh(
                sprite,
                image,
                atlas,
                &atlas_layouts,
                &mut meshes,
                &mut cache,
            );

            if mesh_handle != &new_mesh_handle {
                commands.entity(entity).remove::<Handle<Mesh>>();
                commands.entity(entity).insert(new_mesh_handle);
            }
        }

        // Update the material if it changed

        let new_material_handle =
            get_or_create_material(sprite, image_handle, &mut materials, &mut cache);

        if material_handle != &new_material_handle {
            commands.entity(entity).remove::<Handle<StandardMaterial>>();
            commands.entity(entity).insert(new_material_handle);
        }
    }
}

/// Synchronizes 3D sprites when the index of their texture atlas changes.
pub fn sync_when_atlases_change(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cache: ResMut<Cache>,
    sprites: Query<
        (
            Entity,
            &Sprite3d,
            &TextureAtlas,
            &Handle<Image>,
            &Handle<Mesh>,
        ),
        Changed<TextureAtlas>,
    >,
) {
    for (entity, sprite, atlas, image_handle, mesh_handle) in &sprites {
        if let Some(image) = images.get(image_handle) {
            let new_mesh_handle = get_or_create_mesh(
                sprite,
                image,
                atlas,
                &atlas_layouts,
                &mut meshes,
                &mut cache,
            );

            if mesh_handle != &new_mesh_handle {
                commands.entity(entity).remove::<Handle<Mesh>>();
                commands.entity(entity).insert(new_mesh_handle);
            }
        }
    }
}

// Retrieves a material from the cache or create a new one
fn get_or_create_material(
    sprite: &Sprite3d,
    image_handle: &Handle<Image>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cache: &mut Cache,
) -> Handle<StandardMaterial> {
    let material_id = MaterialId::new(sprite, image_handle);

    cache
        .materials
        .get(&material_id)
        .cloned()
        .unwrap_or_else(|| {
            let material_handle = materials.add(StandardMaterial {
                base_color_texture: Some(image_handle.clone()),
                base_color: sprite.color,
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            });

            cache
                .materials
                .insert(material_id, material_handle.clone_weak());

            material_handle
        })
}

// Retrieves a mesh from the cache or create a new one
fn get_or_create_mesh(
    sprite: &Sprite3d,
    image: &Image,
    atlas: &TextureAtlas,
    atlas_layouts: &Res<Assets<TextureAtlasLayout>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    cache: &mut Cache,
) -> Handle<Mesh> {
    let atlas_layout = atlas_layouts
        .get(&atlas.layout)
        .expect("cannot get 3D sprite's atlas layout");

    let atlas_rect = atlas_layout
        .textures
        .get(atlas.index)
        .expect("cannot get 3D sprite's atlas rect");

    let mesh_id = MeshId::new(sprite, image, atlas_rect);

    cache.meshes.get(&mesh_id).cloned().unwrap_or_else(|| {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList, // Needed to support raycasting
            RenderAssetUsages::default(),
        );

        // Vertices

        let size = match sprite.custom_size {
            Some(size) => size,
            None => image.size_f32(),
        };

        let half = size / 2.0;

        let offset = sprite.anchor.as_vec() * size;

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                // Triangle 1
                [
                    // bottom left
                    -half.x - offset.x,
                    -half.y - offset.y,
                    0.0,
                ],
                [
                    // bottom right
                    half.x - offset.x,
                    -half.y - offset.y,
                    0.0,
                ],
                [
                    // top left
                    -half.x - offset.x,
                    half.y - offset.y,
                    0.0,
                ],
                // Triangle 2
                [
                    // bottom right
                    half.x - offset.x,
                    -half.y - offset.y,
                    0.0,
                ],
                [
                    // top right
                    half.x - offset.x,
                    half.y - offset.y,
                    0.0,
                ],
                [
                    // top left
                    -half.x - offset.x,
                    half.y - offset.y,
                    0.0,
                ],
            ],
        );

        // Texture coordinates

        let atlas_size = atlas_layout.size.as_vec2();

        let mut uvs = vec![
            // Triangle 1
            (UVec2::new(atlas_rect.min.x, atlas_rect.max.y).as_vec2() / atlas_size).to_array(),
            (UVec2::new(atlas_rect.max.x, atlas_rect.max.y).as_vec2() / atlas_size).to_array(),
            (UVec2::new(atlas_rect.min.x, atlas_rect.min.y).as_vec2() / atlas_size).to_array(),
            // Triangle 2
            (UVec2::new(atlas_rect.max.x, atlas_rect.max.y).as_vec2() / atlas_size).to_array(),
            (UVec2::new(atlas_rect.max.x, atlas_rect.min.y).as_vec2() / atlas_size).to_array(),
            (UVec2::new(atlas_rect.min.x, atlas_rect.min.y).as_vec2() / atlas_size).to_array(),
        ];

        if sprite.flip_x {
            uvs.swap(0, 1);
            uvs.swap(5, 4);
            uvs[2] = uvs[5];
            uvs[3] = uvs[1];
        }

        if sprite.flip_y {
            uvs.swap(0, 2);
            uvs.swap(3, 4);
            uvs[1] = uvs[3];
            uvs[5] = uvs[2];
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        let mesh_handle = meshes.add(mesh);

        cache.meshes.insert(mesh_id, mesh_handle.clone());

        mesh_handle
    })
}
