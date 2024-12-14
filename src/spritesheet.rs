use std::ops::RangeBounds;

use bevy::{log::warn, math::UVec2, sprite::TextureAtlasLayout};

use crate::CRATE_NAME;

/// An helper to obtain frame indices from a spritesheet.
///
/// When creating a clip, you might specify its frames by using raw indices:
///
/// ```
/// # use bevy_spritesheet_animation::prelude::*;
/// # let mut library = AnimationLibrary::default();
/// let clip = Clip::from_frames([6, 7, 8, 9, 10, 11]);
/// ```
///
/// However, a clearer and less error-prone approach is to use a [Spritesheet] to retrieve indices with layout queries:
///
/// ```
/// # use bevy_spritesheet_animation::prelude::*;
/// # let mut library = AnimationLibrary::default();
/// // We're working with a spritesheet with 8 columns and 4 rows
///
/// let spritesheet = Spritesheet::new(8, 4);
///
/// // Create a clip from all the frames in row 2
///
/// let clip1 = Clip::from_frames(spritesheet.row(2));
///
/// // Create another clip with the vertical strip starting at (0, 1)
/// // (will wrap to the next columns)
///
/// let clip2 = Clip::from_frames(spritesheet.vertical_strip(0, 1, 12));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Spritesheet {
    /// The number of columns in the spritesheet
    columns: usize,

    /// The number of rows in the spritesheet
    rows: usize,
}

impl Spritesheet {
    /// Creates a new spritesheet helper with the given layout.
    ///
    /// # Arguments
    ///
    /// * `columns` - the number of columns in the spritesheet
    /// * `rows` - the number of rows in the spritesheet
    pub fn new(columns: usize, rows: usize) -> Self {
        Self { columns, rows }
    }

    /// Returns the frame indices for all of the spritesheet.
    ///
    /// This is convenient if the whole spritesheet represents a single animation.
    ///
    /// # Example
    ///
    /// ```
    /// // ┌───┐
    /// // │A B│
    /// // │C D│
    /// // └───┘
    ///
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// let spritesheet = Spritesheet::new(2, 2);
    ///
    /// let clip = Clip::from_frames(spritesheet.all());
    ///
    /// // This clip will play frames A → B → C → D
    ///
    /// assert_eq!(clip.frames(), vec![0, 1, 2, 3]);
    /// ```
    pub fn all(&self) -> Vec<usize> {
        (0..(self.columns * self.rows)).collect()
    }

    /// Returns the frame indices corresponding to the given positions in the spritesheet.
    ///
    /// This is convenient if the frames that you're interested in are scattered all over the spritesheet.
    ///
    /// # Arguments
    ///
    /// * `positions` - the list of (x, y) positions for each frame
    ///
    /// # Example
    ///
    /// ```
    /// // ┌───┐
    /// // │A B│
    /// // │C D│
    /// // └───┘
    ///
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// let spritesheet = Spritesheet::new(2, 2);
    ///
    /// let clip = Clip::from_frames(spritesheet.positions([(1, 0), (0, 1)]));
    ///
    /// // This clip will play frames B → C
    ///
    /// assert_eq!(clip.frames(), vec![1, 2]);
    /// ```
    pub fn positions(&self, positions: impl IntoIterator<Item = (usize, usize)>) -> Vec<usize> {
        let mut indices = Vec::new();

        for (x, y) in positions {
            let index = y * self.columns + x;

            if index >= self.columns * self.rows {
                warn!(
                    "{CRATE_NAME}: position ({x}, {y}) exceeds the spritesheet size ({}, {})",
                    self.columns, self.rows
                );
            } else {
                indices.push(index)
            }
        }

        indices
    }

    /// Returns the frame indices for a whole row of the spritesheet.
    ///
    /// This is convenient if some spritesheet row contains a single animation.
    ///
    /// # Arguments
    ///
    /// * `row` - the index of the spritesheet row to add frames for
    ///
    /// # Example
    ///
    /// ```
    /// // ┌─────┐
    /// // │A B C│
    /// // │D E F│
    /// // └─────┘
    ///
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// let spritesheet = Spritesheet::new(3, 2);
    ///
    /// let clip = Clip::from_frames(spritesheet.row(1));
    ///
    /// // This clip will play frames D → E → F
    ///
    /// assert_eq!(clip.frames(), vec![3, 4, 5]);
    /// ```
    pub fn row(&self, row: usize) -> Vec<usize> {
        if row < self.rows {
            let first_index = row * self.columns;

            (first_index..first_index + self.columns).collect()
        } else {
            warn!(
                "{CRATE_NAME}: row {row} exceeds the spritesheet size ({}, {})",
                self.columns, self.rows
            );

            Vec::new()
        }
    }

    /// Returns the frame indices for a section of a row of the spritesheet.
    ///
    /// This is convenient if some spritesheet row contains an animation next to other unrelated frames.
    ///
    ///
    /// # Arguments
    ///
    /// * `row` - the index of the spritesheet row to add frames for
    /// * `column_range` - the range of columns to add frames for
    ///
    /// # Example
    ///
    /// ```
    /// // ┌─────────┐
    /// // │A B C D E│
    /// // │F G H I J│
    /// // └─────────┘
    ///
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// let spritesheet = Spritesheet::new(5, 2);
    ///
    /// // This clip will play frames G → H → I
    ///
    /// let clip1 = Clip::from_frames(spritesheet.row_partial(1, 1..=3));
    ///
    /// assert_eq!(clip1.frames(), vec![6, 7, 8]);
    ///
    /// // This clip will play frames C → D → E
    ///
    /// let clip2 = Clip::from_frames(spritesheet.row_partial(0, 2..));
    ///
    /// assert_eq!(clip2.frames(), vec![2, 3, 4]);
    ///
    /// // This clip will play frames F → G → H → I
    ///
    /// let clip3 = Clip::from_frames(spritesheet.row_partial(1, ..4));
    ///
    /// assert_eq!(clip3.frames(), vec![5, 6, 7, 8]);
    /// ```
    pub fn row_partial<R: RangeBounds<usize>>(&self, row: usize, column_range: R) -> Vec<usize> {
        if row >= self.rows {
            warn!(
                "{CRATE_NAME}: row {row} exceeds the spritesheet size ({}, {})",
                self.columns, self.rows
            );

            Vec::new()
        } else {
            let first_column = match column_range.start_bound() {
                std::ops::Bound::Included(index) => *index,
                std::ops::Bound::Excluded(_index) => unreachable!(),
                std::ops::Bound::Unbounded => 0,
            };

            let end_column = match column_range.end_bound() {
                std::ops::Bound::Included(index) => (*index).saturating_add(1),
                std::ops::Bound::Excluded(index) => *index,
                std::ops::Bound::Unbounded => self.columns,
            };

            if first_column >= self.columns || end_column > self.columns {
                warn!(
                    "{CRATE_NAME}: range ({:?}, {:?}) exceeds the spritesheet size ({}, {})",
                    column_range.start_bound(),
                    column_range.end_bound(),
                    self.columns,
                    self.rows
                );
            }

            let first_index =
                row * self.columns + first_column.clamp(0, self.columns.saturating_sub(1));

            let end_index = row * self.columns + end_column.clamp(0, self.columns);

            (first_index..end_index).collect()
        }
    }

    /// Returns the frame indices for a whole column of the spritesheet.
    ///
    /// This is convenient if some spritesheet column contains a single animation.
    ///
    /// # Arguments
    ///
    /// * `column` - the index of the spritesheet column to add frames for
    ///
    /// # Example
    ///
    /// ```
    /// // ┌─────┐
    /// // │A B C│
    /// // │D E F│
    /// // └─────┘
    ///
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// let spritesheet = Spritesheet::new(3, 2);
    ///
    /// let clip = Clip::from_frames(spritesheet.column(1));
    ///
    /// // This clip will play frames B → E
    ///
    /// assert_eq!(clip.frames(), vec![1, 4]);
    /// ```
    pub fn column(&self, column: usize) -> Vec<usize> {
        if column < self.columns {
            ((0..self.rows).map(|current_row| column + current_row * self.columns)).collect()
        } else {
            warn!(
                "{CRATE_NAME}: column {column} exceeds the spritesheet size ({}, {})",
                self.columns, self.rows
            );

            Vec::new()
        }
    }

    /// Returns the frame indices for a section of a column of the spritesheet.
    ///
    /// This is convenient if some spritesheet column contains an animation among other unrelated frames.
    ///
    /// # Arguments
    ///
    /// * `column` - the index of the spritesheet column to add frames for
    /// * `row_range` - the range of rows to add frames for
    ///
    /// # Example
    ///
    /// ```
    /// // ┌─────┐
    /// // │A B C│
    /// // │D E F│
    /// // │G H I│
    /// // │J K L│
    /// // └─────┘
    ///
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// let spritesheet = Spritesheet::new(3, 4);
    ///
    /// let clip = Clip::from_frames(spritesheet.column_partial(1, 1..));
    ///
    /// // This clip will play frames E → H → K
    ///
    /// assert_eq!(clip.frames(), vec![4, 7, 10]);
    /// ```
    pub fn column_partial<R: RangeBounds<usize>>(&self, column: usize, row_range: R) -> Vec<usize> {
        if column >= self.columns {
            warn!(
                "{CRATE_NAME}: column {column} exceeds the spritesheet size ({}, {})",
                self.columns, self.rows
            );

            Vec::new()
        } else {
            let mut first_row = match row_range.start_bound() {
                std::ops::Bound::Included(index) => *index,
                std::ops::Bound::Excluded(_index) => unreachable!(),
                std::ops::Bound::Unbounded => 0,
            };

            let mut end_row = match row_range.end_bound() {
                std::ops::Bound::Included(index) => (*index).saturating_add(1),
                std::ops::Bound::Excluded(index) => *index,
                std::ops::Bound::Unbounded => self.rows,
            };

            if first_row >= self.rows || end_row > self.rows {
                warn!(
                    "{CRATE_NAME}: range ({:?}, {:?}) exceeds the spritesheet size ({}, {})",
                    row_range.start_bound(),
                    row_range.end_bound(),
                    self.columns,
                    self.rows
                );
            }

            first_row = first_row.clamp(0, self.rows.saturating_sub(1));

            end_row = end_row.clamp(0, self.rows);

            (first_row..end_row)
                .map(|row| row * self.columns + column)
                .collect()
        }
    }

    /// Returns the frame indices for an horizontal strip in the spritesheet, wrapping from row to row.
    ///
    /// This is convenient if some animations span several rows of a spritesheet.
    ///
    /// # Arguments
    ///
    /// * `x` - the x position of the beginning of the strip
    /// * `y` - the y position of the beginning of the strip
    /// * `count` - the number of frames to add
    ///
    /// # Example
    ///
    /// ```
    /// // ┌─────┐
    /// // │A B C│
    /// // │D E F│
    /// // └─────┘
    ///
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// let spritesheet = Spritesheet::new(3, 2);
    ///
    /// let clip = Clip::from_frames(spritesheet.horizontal_strip(2, 0, 3));
    ///
    /// // This clip will play frames C → D → E
    ///
    /// assert_eq!(clip.frames(), vec![2, 3, 4]);
    /// ```
    pub fn horizontal_strip(&self, x: usize, y: usize, count: usize) -> Vec<usize> {
        let first_index = y * self.columns + x;

        let last_index = (first_index + count).min(self.columns * self.rows);

        let frames = (first_index..last_index).collect();

        if last_index != first_index + count {
            warn!(
                "{CRATE_NAME}: horizontal strip from {x}/{y} with {count} entries exceeds the spritesheet size ({}, {})",
                self.columns, self.rows
            );
        }

        frames
    }

    /// Returns the frame indices for a vertical strip in the spritesheet, wrapping from column to column.
    ///
    /// This is convenient if some animations span several columns of a spritesheet.
    ///
    /// # Arguments
    ///
    /// * `x` - the x position of the beginning of the strip
    /// * `y` - the y position of the beginning of the strip
    /// * `count` - the number of frames to add
    ///
    /// # Example
    ///
    /// ```
    /// // ┌─────┐
    /// // │A B C│
    /// // │D E F│
    /// // └─────┘
    ///
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let mut library = AnimationLibrary::default();
    /// let spritesheet = Spritesheet::new(3, 2);
    ///
    /// let clip = Clip::from_frames(spritesheet.vertical_strip(1, 0, 3));
    ///
    /// // This clip will play frames B → E → C
    ///
    /// assert_eq!(clip.frames(), vec![1, 4, 2]);
    /// ```
    pub fn vertical_strip(&self, x: usize, y: usize, count: usize) -> Vec<usize> {
        let available_count = (self.columns - (x + 1)) * self.rows + self.rows - y;

        let clamped_count = count.min(available_count);

        let frames = (0..clamped_count)
            .map(|i| {
                let current_x = x + (y + i) / self.rows;
                let current_y = (y + i) % self.rows;

                current_y * self.columns + current_x
            })
            .collect();

        if clamped_count != count {
            warn!(
                "{CRATE_NAME}: vertical strip from {x}/{y} with {count} entries exceeds the spritesheet size ({}, {})",
                self.columns, self.rows
            );
        }

        frames
    }

    /// Creates a [TextureAtlasLayout] from the spritesheet.
    ///
    /// # Arguments
    ///
    /// * `frame_width` - the width of a single frame
    /// * `frame_height` - the height of a single frame
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// fn setup(
    ///     mut commands: Commands,
    ///     mut library: ResMut<AnimationLibrary>,
    ///     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ///     assets: Res<AssetServer>,
    /// #   animation_id: AnimationId
    /// ) {
    ///     let spritesheet = Spritesheet::new(8, 8);
    ///
    ///     // ... omitted: create an animation ...
    ///
    ///     let image = assets.load("character.png");
    ///
    ///     let atlas = TextureAtlas {
    ///         layout: atlas_layouts.add(spritesheet.atlas_layout(100, 200)),
    ///         ..default()
    ///     };
    ///
    ///     commands.spawn((
    ///         Sprite::from_atlas_image(image, atlas),
    ///         SpritesheetAnimation::from_id(animation_id),
    ///     ));
    /// }
    /// ```
    pub fn atlas_layout(&self, frame_width: u32, frame_height: u32) -> TextureAtlasLayout {
        TextureAtlasLayout::from_grid(
            UVec2::new(frame_width, frame_height),
            self.columns as u32,
            self.rows as u32,
            None,
            None,
        )
    }
}
