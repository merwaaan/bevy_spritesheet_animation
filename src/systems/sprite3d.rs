use std::collections::HashMap;

use bevy::{
    asset::{Assets, Handle},
    ecs::{
        entity::Entity,
        query::Changed,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    math::UVec2,
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

pub(crate) type QuadUvs = Vec<[f32; 2]>;

/// A resource that stores the UV coordinates for all the atlas layouts used by 3D sprites.
#[derive(Resource, Default)]
pub(crate) struct TextureAtlasLayoutUvs {
    data: HashMap<Handle<TextureAtlasLayout>, Vec<QuadUvs>>,
}

/// Setups 3D sprites for rendering by creating the 3D geometry and materials to display them.
pub(crate) fn setup_rendering(
    mut commands: Commands,
    atlases: Res<Assets<TextureAtlasLayout>>,
    mut atlases_uvs: ResMut<TextureAtlasLayoutUvs>,
    images: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        // We have to wait for the image to be loaded to access its dimensions

        if let Some(texture) = images.get(image_handle) {
            // Generate UVs for this sprite's layout if needed

            let uvs = atlases.get(&atlas.layout).and_then(|layout| {
                let atlas_uvs = atlases_uvs
                    .data
                    .entry(atlas.layout.clone_weak())
                    .or_insert_with(|| create_uvs_from_atlas_layout(layout));

                atlas_uvs.get(atlas.index)
            });

            // Add a mesh to the entity if it does not have one yet

            if maybe_mesh_handle.is_none() {
                let mut mesh = Mesh::new(
                    PrimitiveTopology::TriangleList, // Needed to support raycasting
                    RenderAssetUsages::default(),
                );

                update_mesh_vertices(&mut mesh, sprite, texture);
                update_mesh_uvs(&mut mesh, sprite, uvs);

                commands.entity(entity).insert(meshes.add(mesh));
            }

            // Add a material to the entity if it does not have one yet

            if maybe_material_handle.is_none() {
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
}

/// Synchronizes 3D sprites with the data from their Sprite3D component.
pub(crate) fn sync_sprites_with_component(
    atlases_uvs: Res<TextureAtlasLayoutUvs>,
    images: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sprites: Query<
        (
            &Sprite3d,
            &TextureAtlas,
            &Handle<Image>,
            &Handle<Mesh>,
            &Handle<StandardMaterial>,
        ),
        Changed<Sprite3d>,
    >,
) {
    for (sprite, atlas, image_handle, mesh_handle, material_handle) in &sprites {
        // Update the mesh

        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            // We need to wait for the image to be loaded to get its size

            if let Some(texture) = images.get(image_handle) {
                update_mesh_vertices(mesh, sprite, texture);

                if let Some(uvs) = atlases_uvs.data.get(&atlas.layout) {
                    update_mesh_uvs(mesh, sprite, uvs.get(atlas.index));
                }
            }
        }

        // Update the material

        if let Some(material) = materials.get_mut(material_handle) {
            material.base_color = sprite.color;
        }
    }
}

/// Synchronizes the UV coordinates of the sprites' meshes when the index of their texture atlas changes.
pub(crate) fn sync_sprites_with_atlas(
    atlases_uvs: Res<TextureAtlasLayoutUvs>,
    mut meshes: ResMut<Assets<Mesh>>,
    sprites: Query<(&Sprite3d, &TextureAtlas, &mut Handle<Mesh>), Changed<TextureAtlas>>,
) {
    for (sprite, atlas, mesh_handle) in &sprites {
        if let Some(mesh) = meshes.get_mut(mesh_handle.id()) {
            if let Some(uvs) = atlases_uvs.data.get(&atlas.layout) {
                update_mesh_uvs(mesh, sprite, uvs.get(atlas.index));
            }
        }
    }
}

/// Generates UV coordinates from a texture atlas layout.
fn create_uvs_from_atlas_layout(atlas: &TextureAtlasLayout) -> Vec<QuadUvs> {
    let atlas_layout_size = atlas.size.as_vec2();

    atlas
        .textures
        .iter()
        .map(|texture| {
            vec![
                // Triangle 1
                (UVec2::new(texture.min.x, texture.max.y).as_vec2() / atlas_layout_size).to_array(),
                (UVec2::new(texture.max.x, texture.max.y).as_vec2() / atlas_layout_size).to_array(),
                (UVec2::new(texture.min.x, texture.min.y).as_vec2() / atlas_layout_size).to_array(),
                // Triangle 2
                (UVec2::new(texture.max.x, texture.max.y).as_vec2() / atlas_layout_size).to_array(),
                (UVec2::new(texture.max.x, texture.min.y).as_vec2() / atlas_layout_size).to_array(),
                (UVec2::new(texture.min.x, texture.min.y).as_vec2() / atlas_layout_size).to_array(),
            ]
        })
        .collect()
}

fn update_mesh_vertices(mesh: &mut Mesh, sprite: &Sprite3d, texture: &Image) {
    let size = match sprite.custom_size {
        Some(size) => size,
        None => texture.size_f32(),
    };

    let half_size = size / 2.0;

    let anchor_offset = sprite.anchor.as_vec() * size;

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // Triangle 1
            [
                // bottom left
                -half_size.x - anchor_offset.x,
                -half_size.y - anchor_offset.y,
                0.0,
            ],
            [
                // bottom right
                half_size.x - anchor_offset.x,
                -half_size.y - anchor_offset.y,
                0.0,
            ],
            [
                // top left
                -half_size.x - anchor_offset.x,
                half_size.y - anchor_offset.y,
                0.0,
            ],
            // Triangle 2
            [
                // bottom right
                half_size.x - anchor_offset.x,
                -half_size.y - anchor_offset.y,
                0.0,
            ],
            [
                // top right
                half_size.x - anchor_offset.x,
                half_size.y - anchor_offset.y,
                0.0,
            ],
            [
                // top left
                -half_size.x - anchor_offset.x,
                half_size.y - anchor_offset.y,
                0.0,
            ],
        ],
    );
}

fn update_mesh_uvs(mesh: &mut Mesh, sprite: &Sprite3d, maybe_uvs: Option<&QuadUvs>) {
    static DEFAULT_UVS: [[f32; 2]; 6] = [
        // Triangle 1
        [0.0, 1.0],
        [1.0, 1.0],
        [0.0, 0.0],
        // Triangle 2
        [1.0, 1.0],
        [1.0, 0.0],
        [0.0, 0.0],
    ];

    let mut uvs = match maybe_uvs {
        Some(uvs) => uvs.clone(),
        None => DEFAULT_UVS.to_vec(),
    };

    // Flipping

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
}
