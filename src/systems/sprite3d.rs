use std::hash::Hash;

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    platform::collections::HashMap,
    prelude::*,
    render::render_resource::Face,
};

use crate::prelude::Sprite3d;

/// Cached data for the 3D sprites
#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource, Debug, Default)]
pub(crate) struct Cache {
    /// Materials used by 3D sprites
    materials: HashMap<MaterialId, Handle<StandardMaterial>>,

    /// Meshes used by the 3D sprites
    meshes: HashMap<MeshId, Handle<Mesh>>,
}

/// Uniquely identifies a sprite material
#[derive(Debug, Hash, PartialEq, Eq, Reflect)]
#[reflect(Debug, Hash, PartialEq)]
struct MaterialId {
    image_id: AssetId<Image>,
    color: u32,
    alpha_mode: HashableAlphaMode,
    unlit: bool,
    emissive: HashableLinearRgba,
}

#[derive(Eq, PartialEq, Debug, Reflect)]
struct HashableAlphaMode(AlphaMode);

impl Hash for HashableAlphaMode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.0 {
            AlphaMode::Opaque => 0.hash(state),
            AlphaMode::Mask(cutoff) => {
                1.hash(state);
                cutoff.to_bits().hash(state);
            }
            AlphaMode::Blend => 2.hash(state),
            AlphaMode::Premultiplied => 3.hash(state),
            AlphaMode::Add => 4.hash(state),
            AlphaMode::Multiply => 5.hash(state),
            AlphaMode::AlphaToCoverage => 6.hash(state),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Reflect)]
struct HashableLinearRgba(u32);

impl HashableLinearRgba {
    fn new(color: LinearRgba) -> Self {
        Self(color.as_u32())
    }
}

impl MaterialId {
    fn new(sprite: &Sprite3d, image_handle: &Handle<Image>) -> Self {
        Self {
            image_id: image_handle.id(),
            color: sprite.color.to_linear().as_u32(),
            alpha_mode: HashableAlphaMode(sprite.alpha_mode),
            unlit: sprite.unlit,
            emissive: HashableLinearRgba::new(sprite.emissive),
        }
    }
}

/// Uniquely identifies a sprite mesh
///
/// We use separate meshes with different UVs for each animation frame.
/// Because this is cached, the animation frames using the same attributes share the same mesh.
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

/// Setups 3D sprites for rendering by attaching meshes and materials to render them.
pub fn setup_rendering(
    mut commands: Commands,
    atlas_layouts: Option<Res<Assets<TextureAtlasLayout>>>,
    images: Option<Res<Assets<Image>>>,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
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
    if let (Some(images), Some(atlas_layouts), Some(meshes), Some(materials)) =
        (images, atlas_layouts, &mut meshes, &mut materials)
    {
        for (entity, sprite, maybe_mesh, maybe_material) in &sprites {
            // Add a mesh to the entity if it does not have one yet

            if maybe_mesh.is_none()
                && let Some(mesh_handle) =
                    try_get_or_create_mesh(sprite, &images, &atlas_layouts, meshes, &mut cache)
            {
                commands.entity(entity).insert(Mesh3d(mesh_handle));
            }

            // Add a material to the entity if it does not have one yet

            if maybe_material.is_none() {
                let material_handle = get_or_create_material(sprite, materials, &mut cache);

                commands
                    .entity(entity)
                    .insert(MeshMaterial3d(material_handle));
            }
        }
    }
}

/// Synchronizes 3D sprites when their Sprite3D gets updated.
pub fn sync_when_sprites_change(
    mut commands: Commands,
    images: Option<Res<Assets<Image>>>,
    atlas_layouts: Option<Res<Assets<TextureAtlasLayout>>>,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
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
    if let (Some(images), Some(atlas_layouts), Some(meshes), Some(materials)) =
        (images, atlas_layouts, &mut meshes, &mut materials)
    {
        for (entity, sprite, mesh, material) in &sprites {
            // Update the mesh if it changed

            if let Some(mesh_handle) =
                try_get_or_create_mesh(sprite, &images, &atlas_layouts, meshes, &mut cache)
                && mesh.0 != mesh_handle
            {
                commands.entity(entity).insert(Mesh3d(mesh_handle));
            }

            // Update the material if it changed

            let material_handle = get_or_create_material(sprite, materials, &mut cache);

            if material.0 != material_handle {
                commands
                    .entity(entity)
                    .insert(MeshMaterial3d(material_handle));
            }
        }
    }
}

/// Synchronizes 3D sprites when the index of their texture atlas changes.
pub fn sync_when_atlases_change(
    mut commands: Commands,
    images: Option<Res<Assets<Image>>>,
    atlas_layouts: Option<Res<Assets<TextureAtlasLayout>>>,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut cache: ResMut<Cache>,
    sprites: Query<(Entity, &Sprite3d, &Mesh3d), Changed<Sprite3d>>,
) {
    if let (Some(images), Some(atlas_layouts), Some(meshes)) = (images, atlas_layouts, &mut meshes)
    {
        for (entity, sprite, mesh) in &sprites {
            if let Some(mesh_handle) =
                try_get_or_create_mesh(sprite, &images, &atlas_layouts, meshes, &mut cache)
                && mesh.0 != mesh_handle
            {
                commands.entity(entity).insert(Mesh3d(mesh_handle));
            }
        }
    }
}

// Retrieves a material from the cache or creates a new one
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
            let material_handle = materials.add(StandardMaterial {
                base_color_texture: Some(sprite.image.clone()),
                base_color: sprite.color,
                cull_mode: Some(Face::Back),
                unlit: sprite.unlit,
                alpha_mode: sprite.alpha_mode,
                emissive: sprite.emissive,

                // these are sensible values for 3d rendering,
                // but could be extended to public API
                perceptual_roughness: 0.5,
                reflectance: 0.15,
                ..default()
            });

            cache.materials.insert(material_id, material_handle.clone());

            material_handle
        })
}

// Retrieves a mesh from the cache or creates a new one
//
// We need the image to be loaded to access its dimensions so this returns None if the image is not ready yet
fn try_get_or_create_mesh(
    sprite: &Sprite3d,
    images: &Res<Assets<Image>>,
    atlas_layouts: &Res<Assets<TextureAtlasLayout>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    cache: &mut Cache,
) -> Option<Handle<Mesh>> {
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
                        // Rectangle 1
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
                        // top right
                        [half.x - offset.x, half.y - offset.y, 0.0],
                        // Rectangle 2
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
                        // top right
                        [half.x - offset.x, half.y - offset.y, 0.0],
                    ],
                );

                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_NORMAL,
                    vec![
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, -1.0],
                        [0.0, 0.0, -1.0],
                        [0.0, 0.0, -1.0],
                        [0.0, 0.0, -1.0],
                    ],
                );

                // Texture coordinates

                let atlas_size = atlas_layout.size.as_vec2();

                let mut uvs = vec![
                    // Rectangle 1
                    (UVec2::new(atlas_rect.min.x, atlas_rect.max.y).as_vec2() / atlas_size)
                        .to_array(),
                    (UVec2::new(atlas_rect.max.x, atlas_rect.max.y).as_vec2() / atlas_size)
                        .to_array(),
                    (UVec2::new(atlas_rect.min.x, atlas_rect.min.y).as_vec2() / atlas_size)
                        .to_array(),
                    (UVec2::new(atlas_rect.max.x, atlas_rect.min.y).as_vec2() / atlas_size)
                        .to_array(),
                    // Rectangle 2
                    (UVec2::new(atlas_rect.min.x, atlas_rect.max.y).as_vec2() / atlas_size)
                        .to_array(),
                    (UVec2::new(atlas_rect.max.x, atlas_rect.max.y).as_vec2() / atlas_size)
                        .to_array(),
                    (UVec2::new(atlas_rect.min.x, atlas_rect.min.y).as_vec2() / atlas_size)
                        .to_array(),
                    (UVec2::new(atlas_rect.max.x, atlas_rect.min.y).as_vec2() / atlas_size)
                        .to_array(),
                ];

                if sprite.flip_x {
                    uvs.swap(0, 1);
                    uvs.swap(2, 3);
                    uvs.swap(4, 5);
                    uvs.swap(6, 7);
                }

                if sprite.flip_y {
                    uvs.swap(0, 3);
                    uvs.swap(2, 1);
                    uvs.swap(4, 6);
                    uvs.swap(5, 7);
                }

                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

                mesh.insert_indices(Indices::U32(if sprite.double_sided {
                    vec![0, 1, 2, 1, 3, 2, 5, 4, 6, 7, 5, 6]
                } else {
                    vec![0, 1, 2, 1, 3, 2]
                }));

                let mesh_handle = meshes.add(mesh);

                cache.meshes.insert(mesh_id, mesh_handle.clone());

                mesh_handle
            })
        })
    })?
}
