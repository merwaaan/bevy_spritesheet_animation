use bevy::prelude::*;

use crate::{builder::AnimationBuilder, components::generator::ComponentGenerator};

/// A spritesheet image that is split into cells, each one representing an animation frame.
///
/// With a [Spritesheet], you can [create new animations](Spritesheet::create_animation) with layout queries:
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// fn create_animated_sprite(
///     mut commands: Commands,
///     assets: Res<AssetServer>,
/// ) {
///     // We're working with a spritesheet with 8 columns and 4 rows
///
///     let image = assets.load("character.png");
///
///     let animation = Spritesheet::new(&image, 8, 4)
///         .create_animation()
///         // Use all the frames in row 2
///         .add_row(2)
///         // Add a few extra frames
///         .add_cell(4, 0)
///         .add_cell(4, 3)
///         // Get the final animation
///         .build();
///
///     // ... omitted: create a sprite that uses this animation
/// }
/// ```
///
/// A [Spritesheet] also provides helpers to generate Bevy components like Sprites and TextureAtlases that match the source image.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// fn create_animated_sprite(
///     mut commands: Commands,
///     assets: Res<AssetServer>,
///     images: ResMut<Assets<Image>>,
///     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
///     # spritesheet: &mut Spritesheet,
/// ) {
///     // ... omitted: create an animation from the spritesheet
///     # let animation = Handle::default();
///
///     // Create a standard Bevy sprite using the spritesheet's image and layout
///
///     let sprite = spritesheet
///         .with_loaded_image(&images)
///         .expect("the image is not loaded")
///         .sprite(&mut atlas_layouts);
///
///     commands.spawn((
///         // This is a regular Bevy sprite
///         sprite,
///         // This is the component that animates the sprite
///         SpritesheetAnimation::new(animation),
///     ));
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Spritesheet {
    /// The spritesheet image
    image: Handle<Image>,

    /// The number of columns in the spritesheet
    columns: usize,

    /// The number of rows in the spritesheet
    rows: usize,
}

impl Spritesheet {
    /// Creates a Spritesheet.
    ///
    /// # Arguments
    ///
    /// - `image` - the spritesheet's image
    /// - `columns` - the number of columns in the spritesheet
    /// - `rows` - the number of rows in the spritesheet
    pub fn new(image: &Handle<Image>, columns: usize, rows: usize) -> Self {
        Self {
            image: image.clone(),
            columns,
            rows,
        }
    }

    pub fn image(&self) -> &Handle<Image> {
        &self.image
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Creates a new animation that uses that spritesheet.
    ///
    /// This returns an [AnimationBuilder] with which you can extract frames from the spritesheet and set playback parameters.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// fn create_animated_sprite(
    ///     mut commands: Commands,
    ///     assets: Res<AssetServer>,
    /// ) {
    ///     let image = assets.load("character.png");
    ///
    ///     let spritesheet = Spritesheet::new(&image, 8, 8)
    ///         .create_animation()
    ///         .add_column(2)
    ///         .set_repetitions(AnimationRepeat::Times(3))
    ///         .build();
    ///
    ///     // ...
    /// }
    /// ```
    pub fn create_animation(&self) -> AnimationBuilder {
        AnimationBuilder::new(self.clone())
    }

    /// Creates a component generator if the spritesheet's image has been loaded and is ready to use.
    ///
    /// If the image is not loaded, this returns `None`.
    ///
    /// If you don't want to wait for the image to load, you might prefer using [Spritesheet::with_size_hint].
    ///
    /// # Arguments
    ///
    /// - `assets` - the Bevy asset server
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// fn create_animated_sprite(
    ///     mut commands: Commands,
    ///     assets: Res<AssetServer>,
    ///     images: ResMut<Assets<Image>>,
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
    pub fn with_loaded_image(&self, images: &Assets<Image>) -> Option<ComponentGenerator> {
        images
            .get(&self.image)
            .map(|image| ComponentGenerator::new(self, image.width(), image.height()))
    }

    /// Creates a component generator using an explicit image size.
    ///
    /// It's safer to use [Spritesheet::with_loaded_image] to avoid mismatches between the loaded image and the size hint.
    ///
    /// However, this is a convenient alternative if you don't want to wait for the image to load.
    ///
    /// # Arguments
    ///
    /// - `image_width` - the width of the spritesheet image
    /// - `image_height` - the height of the spritesheet image
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// fn create_animated_sprite(
    ///     mut commands: Commands,
    ///     assets: Res<AssetServer>,
    ///     images: ResMut<Assets<Image>>,
    ///     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ///     # spritesheet: Spritesheet,
    ///     # animation: Handle<Animation>,
    /// ) {
    ///     // ... omitted: create a spritesheet and an animation
    ///
    ///     let sprite = spritesheet
    ///         .with_size_hint(800, 400)
    ///         .sprite(&mut atlas_layouts);
    ///
    ///     commands.spawn((
    ///         sprite,
    ///         SpritesheetAnimation::new(animation),
    ///     ));
    /// }
    /// ```
    pub fn with_size_hint(&self, image_width: u32, image_height: u32) -> ComponentGenerator {
        ComponentGenerator::new(self, image_width, image_height)
    }
}
