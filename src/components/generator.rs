#[cfg(feature = "custom_cursor")]
use bevy::window::{CursorIcon, CustomCursor, CustomCursorImage};
use bevy::{
    asset::Assets,
    image::{TextureAtlas, TextureAtlasLayout},
    math::UVec2,
    sprite::Sprite,
    ui::widget::ImageNode,
    utils::default,
};

use crate::{components::sprite3d::Sprite3d, spritesheet::Spritesheet};

/// A helper to generate animation-ready components such as sprites, texture atlases, UI images and cursors.
///
/// Create a component generator with:
/// - [Spritesheet::with_loaded_image()](Spritesheet::with_loaded_image) if the sprite's image has already been loaded
/// - [Spritesheet::with_size_hint()](Spritesheet::with_size_hint) with an explicit size if you don't want to deal with the sprite's image loading asynchronously
pub struct ComponentGenerator {
    spritesheet: Spritesheet,
    image_width: u32,
    image_height: u32,
}

impl ComponentGenerator {
    pub(crate) fn new(spritesheet: &Spritesheet, image_width: u32, image_height: u32) -> Self {
        Self {
            spritesheet: spritesheet.clone(),
            image_width,
            image_height,
        }
    }

    /// Creates an animation-ready Bevy [Sprite](https://docs.rs/bevy/latest/bevy/sprite/struct.Sprite.html).
    ///
    /// # Arguments
    ///
    /// - `atlas_layouts` - the atlas layouts of the Bevy app
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// fn create_animated_sprite(
    ///     mut commands: Commands,
    ///     assets: Res<AssetServer>,
    ///     images: Res<Assets<Image>>,
    ///     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ///     # spritesheet: Spritesheet,
    ///     # animation: Handle<Animation>,
    /// ) {
    ///     // ... omitted: create a spritesheet and an animation
    ///
    ///     let sprite = spritesheet
    ///         .with_loaded_image(&images)
    ///         .expect("the image is not loaded")
    ///         .sprite(&mut atlas_layouts);
    ///
    ///     commands.spawn((
    ///         sprite,
    ///         SpritesheetAnimation::new(animation),
    ///     ));
    /// }
    /// ```
    pub fn sprite(&self, atlas_layouts: &mut Assets<TextureAtlasLayout>) -> Sprite {
        Sprite::from_atlas_image(self.spritesheet.image().clone(), self.atlas(atlas_layouts))
    }

    /// Creates an animation-ready [Sprite3d].
    ///
    /// # Arguments
    ///
    /// - `atlas_layouts` - the atlas layouts of the Bevy app
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::{prelude::*, sprite::Anchor};
    /// # use bevy_spritesheet_animation::prelude::*;
    /// fn create_animated_3d_sprite(
    ///     mut commands: Commands,
    ///     assets: Res<AssetServer>,
    ///     images: Res<Assets<Image>>,
    ///     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ///     # spritesheet: Spritesheet,
    ///     # animation: Handle<Animation>,
    /// ) {
    ///     // ... omitted: create a spritesheet and an animation
    ///
    ///     let sprite = spritesheet
    ///         .with_loaded_image(&images)
    ///         .expect("the image is not loaded")
    ///         .sprite3d(&mut atlas_layouts)
    ///         // Configure the sprite if needed
    ///         .with_anchor(Anchor::BOTTOM_CENTER)
    ///         .with_flip(true, false);
    ///     
    ///     commands.spawn((
    ///         sprite,
    ///         SpritesheetAnimation::new(animation),
    ///     ));
    /// }
    /// ```
    pub fn sprite3d(&self, atlas_layouts: &mut Assets<TextureAtlasLayout>) -> Sprite3d {
        Sprite3d::from_atlas_image(self.spritesheet.image().clone(), self.atlas(atlas_layouts))
    }

    /// Creates an animation-ready Bevy UI [ImageNode](https://docs.rs/bevy/latest/bevy/ui/prelude/struct.ImageNode.html).
    ///
    /// # Arguments
    ///
    /// - `atlas_layouts` - the atlas layouts of the Bevy app
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// fn create_animated_ui_image(
    ///     mut commands: Commands,
    ///     assets: Res<AssetServer>,
    ///     images: Res<Assets<Image>>,
    ///     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ///     # spritesheet: Spritesheet,
    ///     # animation: Handle<Animation>,
    /// ) {
    ///     // ... omitted: create a spritesheet and an animation
    ///
    ///     let image_node = spritesheet
    ///         .with_loaded_image(&images)
    ///         .expect("the image is not loaded")
    ///         .image_node(&mut atlas_layouts);
    ///
    ///      commands
    ///         .spawn(Node {
    ///             width: Val::Percent(100.0),
    ///             height: Val::Percent(100.0),
    ///             justify_content: JustifyContent::Center,
    ///             align_items: AlignItems::Center,
    ///             ..default()
    ///         })
    ///         .with_child((
    ///             image_node,
    ///             SpritesheetAnimation::new(animation),
    ///             UiTransform::from_scale(Vec2::splat(10.0)),
    ///         ));
    /// }
    /// ```
    pub fn image_node(&self, atlas_layouts: &mut Assets<TextureAtlasLayout>) -> ImageNode {
        ImageNode::from_atlas_image(self.spritesheet.image().clone(), self.atlas(atlas_layouts))
    }

    /// Creates an animation-ready Bevy [CursorIcon](https://docs.rs/bevy/latest/bevy/window/enum.CursorIcon.html).
    ///
    /// # Arguments
    ///
    /// - `atlas_layouts` - the atlas layouts of the Bevy app
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// fn create_animated_cursor(
    ///     window: Single<Entity, With<Window>>,
    ///     mut commands: Commands,
    ///     assets: Res<AssetServer>,
    ///     images: Res<Assets<Image>>,
    ///     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ///     # spritesheet: Spritesheet,
    ///     # animation: Handle<Animation>,
    /// ) {
    ///     // ... omitted: create a spritesheet and an animation
    ///
    ///     let cursor_icon = spritesheet
    ///         .with_loaded_image(&images)
    ///         .expect("the image is not loaded")
    ///         .cursor_icon(&mut atlas_layouts);
    ///
    ///     commands.entity(*window).insert((
    ///         cursor_icon,
    ///         SpritesheetAnimation::new(animation),
    ///     ));
    /// }
    /// ```
    #[cfg(feature = "custom_cursor")]
    pub fn cursor_icon(&self, atlas_layouts: &mut Assets<TextureAtlasLayout>) -> CursorIcon {
        CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            handle: self.spritesheet.image().clone(),
            texture_atlas: Some(self.atlas(atlas_layouts)),
            ..default()
        }))
    }

    /// Creates a Bevy [TextureAtlas](https://docs.rs/bevy/latest/bevy/image/struct.TextureAtlas.html) that matches the spritesheet.
    ///
    /// This can be useful if you need lower-level access to the Bevy Sprite, for instance to set some attributes like its color.
    ///
    /// # Arguments
    ///
    /// - `atlas_layouts` - the atlas layouts of the Bevy app
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// fn create_animated_sprite(
    ///     mut commands: Commands,
    ///     assets: Res<AssetServer>,
    ///     images: Res<Assets<Image>>,
    ///     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ///     # image: Handle<Image>,
    ///     # spritesheet: Spritesheet,
    ///     # animation: Handle<Animation>,
    /// ) {
    ///     // ... omitted: create a spritesheet and an animation
    ///
    ///     let atlas = spritesheet
    ///         .with_loaded_image(&images)
    ///         .expect("the image is not loaded")
    ///         .atlas(&mut atlas_layouts);
    ///
    ///     commands.spawn((
    ///         Sprite {
    ///             image,
    ///             texture_atlas: Some(atlas),
    ///             color: Color::linear_rgb(1.0, 0.0, 0.0),
    ///             ..default()
    ///         },
    ///         SpritesheetAnimation::new(animation),
    ///     ));
    /// }
    /// ```
    pub fn atlas(&self, atlas_layouts: &mut Assets<TextureAtlasLayout>) -> TextureAtlas {
        let cell_width = self.image_width / (self.spritesheet.columns() as u32);
        let cell_height = self.image_height / (self.spritesheet.rows() as u32);

        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(cell_width, cell_height),
            self.spritesheet.columns() as u32,
            self.spritesheet.rows() as u32,
            None,
            None,
        );

        let layout_handle = atlas_layouts.add(layout);

        TextureAtlas {
            layout: layout_handle,
            ..default()
        }
    }
}
