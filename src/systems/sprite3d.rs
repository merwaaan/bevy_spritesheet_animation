use std::collections::HashMap;

use bevy::{
    asset::{AssetId, Assets, Handle},
    ecs::{
        change_detection::DetectChanges,
        entity::Entity,
        query::{Changed, With, Without},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    math::Vec2,
    pbr::{AlphaMode, StandardMaterial},
    prelude::default,
    render::{
        mesh::{Mesh, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        texture::Image,
    },
    sprite::{TextureAtlas, TextureAtlasLayout},
};

use crate::components::sprite3d::Sprite3D;

pub type QuadUvs = Vec<[f32; 2]>;

// UV coordinates for all the 3D sprites' atlas layouts
#[derive(Resource, Default)]
pub struct TextureAtlasLayoutUvs {
    layouts: HashMap<Handle<TextureAtlasLayout>, Vec<QuadUvs>>,
}

pub fn sync_atlas_layout_uvs(
    atlases: Res<Assets<TextureAtlasLayout>>,
    mut atlases_uvs: ResMut<TextureAtlasLayoutUvs>,
    sprites: Query<&TextureAtlas, With<Sprite3D>>,
) {
    for atlas in &sprites {
        if let Some(layout) = atlases.get(&atlas.layout) {
            // TODO on change too!
            if !atlases_uvs.layouts.contains_key(&atlas.layout) {
                atlases_uvs.layouts.insert(
                    atlas.layout.clone_weak(),
                    create_uvs_from_atlas_layout(layout),
                );
            }
        }
    }
}

// Generates UV coordinates from a texture atlas layout
fn create_uvs_from_atlas_layout(atlas_layout: &TextureAtlasLayout) -> Vec<QuadUvs> {
    atlas_layout
        .textures
        .iter()
        .map(|texture| {
            vec![
                (Vec2::new(texture.min.x, texture.max.y) / atlas_layout.size).to_array(),
                (Vec2::new(texture.max.x, texture.max.y) / atlas_layout.size).to_array(),
                (Vec2::new(texture.min.x, texture.min.y) / atlas_layout.size).to_array(),
                (Vec2::new(texture.max.x, texture.min.y) / atlas_layout.size).to_array(),
            ]
        })
        .collect()
}

/// Adds a mesh and a material to 3D sprites that don't have those yet
/// (Requires the image to be loaded)
pub fn add_mesh_and_material_to_3d_sprite(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sprites_without_meshes: Query<(Entity, &Sprite3D, &Handle<Image>), Without<Handle<Mesh>>>, // TODO without mat too
) {
    for (entity, sprite, image_handle) in &sprites_without_meshes {
        // Is the image loaded?

        if let Some(texture) = images.get(image_handle) {
            let size = match sprite.custom_size {
                Some(size) => size,
                None => texture.size_f32(),
            };

            let mesh = create_mesh(size);

            // No atlas: just use the whole texture

            // if maybe_atlas.is_none() {
            //     mesh.insert_attribute(
            //         Mesh::ATTRIBUTE_UV_0,
            //         vec![[0.0, 1.0], [1.0, 1.0], [0.0, 0.0], [1.0, 0.0]],
            //     );
            // }

            let material = StandardMaterial {
                base_color_texture: Some(image_handle.clone()),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            };

            commands
                .entity(entity)
                .insert(meshes.add(mesh))
                .insert(materials.add(material));
        }
    }
}

// Creates a quad mesh to display a sprite
fn create_mesh(size: Vec2) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleStrip,
        RenderAssetUsages::default(),
    );

    update_mesh_vertices(&mut mesh, size);
    //update_mesh_uvs(&mut mesh, maybe_uvs);

    mesh
}

fn update_mesh_vertices(mesh: &mut Mesh, size: Vec2) {
    let half_size = size / 2.0;

    // TODO origin

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-half_size.x, -half_size.y, 0.0],
            [half_size.x, -half_size.y, 0.0],
            [-half_size.x, half_size.y, 0.0],
            [half_size.x, half_size.y, 0.0],
        ],
    );
}

// fn update_mesh_uvs(mesh: &mut Mesh, maybe_uvs: Option<&QuadUvs>) {
//     let uvs = match maybe_uvs {
//         Some(uvs) => uvs.clone(),
//         None => vec![[0.0, 1.0], [1.0, 1.0], [0.0, 0.0], [1.0, 0.0]],
//     };

//     mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
// }

// // TODO
// pub fn resize_mesh_when_sprite_size_changes(
//     mut params: Sprites3dParams,
//     changed_sprites: Query<
//         (&Sprite3D, &Handle<StandardMaterial>, &mut Handle<Mesh>),
//         (Changed<Sprite3D>, With<Handle<Mesh>>),
//     >,
// ) {
//     for (sprite, material_handle, mesh_handle) in &changed_sprites {
//         if let Some(StandardMaterial {
//             base_color_texture: Some(texture_handle),
//             ..
//         }) = params.materials.get(material_handle)
//         {
//             if let Some(image) = params.images.get(texture_handle) {
//                 if let Some(mesh) = params.meshes.get_mut(mesh_handle) {
//                     let size_info = match sprite.size {
//                         SpriteSize::Fixed(size) => SizeInfo::FixedSize(size),
//                         SpriteSize::Scaled(size) => SizeInfo::ScaledSizeAndImage(size, image),
//                     };

//                     update_mesh_vertices(mesh, size_info);
//                 }
//             }
//         }
//     }
// }

/// Synchronizes the UV coordinates of the sprites' meshes whenever the index of their texture atlas changes
pub fn sync_mesh_uvs_with_atlas_index(
    atlas_uvs: Res<TextureAtlasLayoutUvs>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sprites_with_changed_atlas: Query<
        (&TextureAtlas, &mut Handle<Mesh>),
        (With<Sprite3D>, Changed<TextureAtlas>),
    >,
) {
    for (atlas, mesh_handle) in &mut sprites_with_changed_atlas {
        if let Some(mesh) = meshes.get_mut(mesh_handle.id()) {
            println!("{:?}", atlas);
            if atlas.layout.is_weak() {
                panic!();
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_UV_0,
                    vec![[0.0, 1.0], [1.0, 1.0], [0.0, 0.0], [1.0, 0.0]],
                );
            } else if let Some(atlas_uvs) = atlas_uvs.layouts.get(&atlas.layout) {
                if let Some(current_atlas_uvs) = atlas_uvs.get(atlas.index) {
                    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, current_atlas_uvs.clone());
                }
            }
        }
    }
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

/// Deletes outdated UV associated to atlas layouts that do not exist anymore
pub fn delete_outdated_uvs(
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    mut atlas_layout_uvs: ResMut<TextureAtlasLayoutUvs>,
) {
    // TODO check ref count instead

    if atlas_layouts.is_changed() {
        atlas_layout_uvs
            .layouts
            .retain(|layout, _| atlas_layouts.contains(layout));
    }
}
