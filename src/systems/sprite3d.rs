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
    },
    sprite::TextureAtlasLayout,
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
            Option<&Mesh3d>,
            Option<&MeshMaterial3d<StandardMaterial>>,
        ),
        Or<(Without<Mesh3d>, Without<MeshMaterial3d<StandardMaterial>>)>,
    >,
) {
    for (entity, sprite, maybe_mesh, maybe_material) in &sprites {
        // Add a mesh to the entity if it does not have one yet

        if maybe_mesh.is_none() {
            try_get_or_create_mesh(sprite, &images, &atlas_layouts, &mut meshes, &mut cache)
                .inspect(|mesh_handle| {
                    commands.entity(entity).insert(Mesh3d(mesh_handle.clone()));
                });
        }

        // Add a material to the entity if it does not have one yet

        if maybe_material.is_none() {
            let material = StandardMaterial {
                base_color_texture: Some(sprite.image.clone()),
                base_color: sprite.color,
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            };

            let material_handle = materials.add(material);

            commands
                .entity(entity)
                .insert(MeshMaterial3d(material_handle));
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
            &Mesh3d,
            &MeshMaterial3d<StandardMaterial>,
        ),
        Changed<Sprite3d>,
    >,
) {
    for (entity, sprite, mesh, material) in &sprites {
        // Update the mesh if it changed

        try_get_or_create_mesh(sprite, &images, &atlas_layouts, &mut meshes, &mut cache).inspect(
            |new_mesh_handle| {
                if mesh.0 != *new_mesh_handle {
                    commands.entity(entity).remove::<Mesh3d>();

                    commands
                        .entity(entity)
                        .insert(Mesh3d(new_mesh_handle.clone()));
                }
            },
        );
        // Update the material if it changed

        let new_material_handle = get_or_create_material(sprite, &mut materials, &mut cache);

        if material.0 != new_material_handle {
            commands
                .entity(entity)
                .remove::<MeshMaterial3d<StandardMaterial>>();

            commands
                .entity(entity)
                .insert(MeshMaterial3d(new_material_handle));
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
    sprites: Query<(Entity, &Sprite3d, &Mesh3d), Changed<Sprite3d>>,
) {
    for (entity, sprite, mesh) in &sprites {
        try_get_or_create_mesh(sprite, &images, &atlas_layouts, &mut meshes, &mut cache).inspect(
            |new_mesh_handle| {
                if mesh.0 != *new_mesh_handle {
                    commands.entity(entity).remove::<Mesh3d>();
                    commands
                        .entity(entity)
                        .insert(Mesh3d(new_mesh_handle.clone()));
                }
            },
        );
    }
}

// Retrieves a material from the cache or create a new one
fn get_or_create_material(
    sprite: &Sprite3d,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cache: &mut Cache,
) -> Handle<StandardMaterial> {
    let material_id = MaterialId::new(sprite, &sprite.image);

    cache
        .materials
        .get(&material_id)
        .cloned()
        .unwrap_or_else(|| {
            let material_handle: Handle<StandardMaterial> = materials.add(StandardMaterial {
                base_color_texture: Some(sprite.image.clone()),
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
fn try_get_or_create_mesh(
    sprite: &Sprite3d,
    images: &Res<Assets<Image>>,
    atlas_layouts: &Res<Assets<TextureAtlasLayout>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    cache: &mut Cache,
) -> Option<Handle<Mesh>> {
    // We have to wait for the image to be loaded to access its dimensions

    images.get(&sprite.image).map(|sprite_image| {
        sprite.texture_atlas.as_ref().map(|sprite_atlas| {
            let atlas_layout = atlas_layouts
                .get(&sprite_atlas.layout)
                .expect("cannot get 3D sprite's atlas layout");

            let atlas_rect = atlas_layout
                .textures
                .get(sprite_atlas.index)
                .expect("cannot get 3D sprite's atlas rect");

            let mesh_id = MeshId::new(sprite, sprite_image, atlas_rect);

            cache.meshes.get(&mesh_id).cloned().unwrap_or_else(|| {
                let mut mesh = Mesh::new(
                    PrimitiveTopology::TriangleList, // Needed to support raycasting
                    RenderAssetUsages::default(),
                );

                // Vertices

                let size = match sprite.custom_size {
                    Some(size) => size,
                    None => sprite_image.size_f32(),
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
                    (UVec2::new(atlas_rect.min.x, atlas_rect.max.y).as_vec2() / atlas_size)
                        .to_array(),
                    (UVec2::new(atlas_rect.max.x, atlas_rect.max.y).as_vec2() / atlas_size)
                        .to_array(),
                    (UVec2::new(atlas_rect.min.x, atlas_rect.min.y).as_vec2() / atlas_size)
                        .to_array(),
                    // Triangle 2
                    (UVec2::new(atlas_rect.max.x, atlas_rect.max.y).as_vec2() / atlas_size)
                        .to_array(),
                    (UVec2::new(atlas_rect.max.x, atlas_rect.min.y).as_vec2() / atlas_size)
                        .to_array(),
                    (UVec2::new(atlas_rect.min.x, atlas_rect.min.y).as_vec2() / atlas_size)
                        .to_array(),
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
        })
    })?
}

pub(crate) fn remove_dropped_standard_materials(
    mut cache: ResMut<Cache>,
    mut standard_material_events: EventReader<AssetEvent<StandardMaterial>>,
) {
    for event in standard_material_events.read() {
        if let AssetEvent::Removed { id } = event {
            cache.materials.retain(|_, handle| handle.id() != *id);
        }
    }
}
