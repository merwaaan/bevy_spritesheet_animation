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
    prelude::{default, DetectChanges, Or, Without},
    render::{
        alpha::AlphaMode,
        mesh::{Mesh, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        texture::Image,
    },
    sprite::{TextureAtlas, TextureAtlasLayout},
};

use crate::components::sprite3d::Sprite3D;

pub type QuadUvs = Vec<[f32; 2]>;

/// UV coordinates for all the 3D sprites' atlas layouts
#[derive(Resource, Default)]
pub struct AtlasUvs {
    layouts: HashMap<Handle<TextureAtlasLayout>, Vec<QuadUvs>>,
}

pub fn setup_rendering(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    atlases: Res<Assets<TextureAtlasLayout>>,
    mut atlases_uvs: ResMut<AtlasUvs>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sprites: Query<
        (
            Entity,
            &Sprite3D,
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
        // Generate UVs for this sprite's layout if needed

        if let Some(layout) = atlases.get(&atlas.layout) {
            if !atlases_uvs.layouts.contains_key(&atlas.layout) {
                println!("add uvs");

                atlases_uvs.layouts.insert(
                    atlas.layout.clone_weak(), // Keep a weak reference, will be deleted by another system when TODO
                    create_uvs_from_atlas_layout(layout),
                );
            }
        }

        //

        if let Some(texture) = images.get(image_handle) {
            // Add a mesh to the entity if it does not have one yet

            if maybe_mesh_handle.is_none() {
                println!("sprite add mesh");

                let mut mesh = Mesh::new(
                    PrimitiveTopology::TriangleStrip,
                    RenderAssetUsages::default(),
                );

                update_mesh_vertices(&mut mesh, sprite, texture);
                if let Some(layout) = atlases_uvs.layouts.get(&atlas.layout) {
                    update_mesh_uvs(&mut mesh, sprite, layout.get(atlas.index));
                }

                commands.entity(entity).insert(meshes.add(mesh));
            }

            // Add a material to the entity if it does not have one yet

            if maybe_material_handle.is_none() {
                println!("sprite add material");

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

    // Deletes outdated UVs associated to atlas layouts that do not exist anymore

    if atlases.is_changed() {
        // TODO not working atlases_uvs.layouts.retain(|layout, _| layout.is_strong());
    }
}

///
pub fn sync_sprites_with_component(
    images: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    atlases_uvs: Res<AtlasUvs>,
    sprites: Query<
        (
            &Sprite3D,
            &TextureAtlas,
            &Handle<Image>,
            &Handle<Mesh>,
            &Handle<StandardMaterial>,
        ),
        Changed<Sprite3D>,
    >,
) {
    for (sprite, atlas, image_handle, mesh_handle, material_handle) in &sprites {
        // Update the mesh

        // TODO finer granularity?

        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            // We need to wait for the image to be loaded to get its size

            if let Some(texture) = images.get(image_handle) {
                update_mesh_vertices(mesh, sprite, texture);

                if let Some(layout) = atlases_uvs.layouts.get(&atlas.layout) {
                    update_mesh_uvs(mesh, sprite, layout.get(atlas.index));
                }
            }
        }

        // Update the material

        if let Some(material) = materials.get_mut(material_handle) {
            material.base_color = sprite.color;
        }
    }
}

/// Synchronizes the UV coordinates of the sprites' meshes when the index of their texture atlas changes
pub fn sync_sprites_with_atlas(
    atlases_uvs: Res<AtlasUvs>,
    mut meshes: ResMut<Assets<Mesh>>,
    sprites: Query<(&Sprite3D, &TextureAtlas, &mut Handle<Mesh>), Changed<TextureAtlas>>,
) {
    for (sprite, atlas, mesh_handle) in &sprites {
        // Update the mesh's UVs to match the current atlas index

        if let Some(mesh) = meshes.get_mut(mesh_handle.id()) {
            if let Some(layout) = atlases_uvs.layouts.get(&atlas.layout) {
                update_mesh_uvs(mesh, sprite, layout.get(atlas.index));
            }
        }
    }
}

/// Generates UV coordinates from a texture atlas layout
fn create_uvs_from_atlas_layout(atlas: &TextureAtlasLayout) -> Vec<QuadUvs> {
    let atlas_layout_size = atlas.size.as_vec2();

    atlas
        .textures
        .iter()
        .map(|texture| {
            vec![
                (UVec2::new(texture.min.x, texture.max.y).as_vec2() / atlas_layout_size).to_array(),
                (UVec2::new(texture.max.x, texture.max.y).as_vec2() / atlas_layout_size).to_array(),
                (UVec2::new(texture.min.x, texture.min.y).as_vec2() / atlas_layout_size).to_array(),
                (UVec2::new(texture.max.x, texture.min.y).as_vec2() / atlas_layout_size).to_array(),
            ]
        })
        .collect()
}

// TODO system to warn about invalid configs (already have mesh etc...)

// TODO merge geom & uvs
fn update_mesh_vertices(mesh: &mut Mesh, sprite: &Sprite3D, texture: &Image) {
    let size = match sprite.custom_size {
        Some(size) => size,
        None => texture.size_f32(),
    };

    let half_size = size / 2.0;

    let anchor_offset = sprite.anchor.as_vec() * size;

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [
                -half_size.x - anchor_offset.x,
                -half_size.y - anchor_offset.y,
                0.0,
            ],
            [
                half_size.x - anchor_offset.x,
                -half_size.y - anchor_offset.y,
                0.0,
            ],
            [
                -half_size.x - anchor_offset.x,
                half_size.y - anchor_offset.y,
                0.0,
            ],
            [
                half_size.x - anchor_offset.x,
                half_size.y - anchor_offset.y,
                0.0,
            ],
        ],
    );
}

fn update_mesh_uvs(mesh: &mut Mesh, sprite: &Sprite3D, maybe_uvs: Option<&QuadUvs>) {
    static DEFAULT_UVS: [[f32; 2]; 4] = [[0.0, 1.0], [1.0, 1.0], [0.0, 0.0], [1.0, 0.0]];

    let mut uvs = match maybe_uvs {
        Some(uvs) => uvs.clone(),
        None => DEFAULT_UVS.to_vec(),
    };

    // Flipping

    if sprite.flip_x {
        uvs.swap(0, 1);
        uvs.swap(2, 3);
    }

    if sprite.flip_y {
        uvs.swap(0, 2);
        uvs.swap(1, 3);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
}

// // Updates outdated UV coordinates when the atlas layouts that they reference have changed
// pub fn update_uvs_when_atlas_layout_changes(
//     atlas_layouts: Res<Assets<TextureAtlasLayout>>,
//     mut atlas_layout_uvs: ResMut<TextureAtlasLayoutUvs>,
// ) {
//     if atlas_layouts.is_changed() {
//         for (layout_handle, layout_uvs) in atlas_layout_uvs.layouts_mut() {
//             if let Some(xxx) = atlas_layouts.get(layout_handle) {
//                 *layout_uvs = create_uvs_from_atlas_layout(&xxx);
//             }
//         }
//     }

//     // NOTE: this recomputes ALL the sprites' UVs when any layout changes, which is not great.
//     // It might be better to track individual layouts.
// }
