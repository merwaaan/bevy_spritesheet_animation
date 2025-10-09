use std::ops::RangeBounds;

use bevy::prelude::*;

use crate::{CRATE_NAME, prelude::*};

/// A builder to create animations from a spritesheet.
///
/// Set playback parameters with [set_duration()](AnimationBuilder::set_duration), [set_repetitions()](AnimationBuilder::set_repetitions), etc...
///
/// Add additional clips with [start_clip()](AnimationBuilder::start_clip) and [copy_clip()](AnimationBuilder::copy_clip) (only needed for complex animations).
///
/// Set clip-level playback parameters with [set_clip_duration()](AnimationBuilder::set_clip_duration), [set_clip_repetitions()](AnimationBuilder::set_clip_repetitions), etc...
///
/// Call [build()](AnimationBuilder::build) to retrieve the final animation.
#[derive(Clone)]
pub struct AnimationBuilder {
    spritesheet: Spritesheet,
    animation: Animation,
}

impl AnimationBuilder {
    pub fn new(spritesheet: Spritesheet) -> Self {
        Self {
            spritesheet,
            animation: Animation::empty(),
        }
    }

    /// Sets the duration of the whole animation.
    ///
    /// If specified, this will be combined with the underlying clips' durations set with [set_clip_duration()](AnimationBuilder::set_clip_duration).
    pub fn set_duration(mut self, duration: AnimationDuration) -> Self {
        self.animation.duration = Some(duration);
        self
    }

    /// Sets the repetitions of the whole animation.
    ///
    /// If specified, this will be combined with the underlying clips' repetitions set with [set_clip_repetitions()](AnimationBuilder::set_clip_repetitions).
    pub fn set_repetitions(mut self, repetitions: AnimationRepeat) -> Self {
        self.animation.repetitions = Some(repetitions);
        self
    }

    /// Sets the direction of the whole animation.
    ///
    /// If specified, this will be combined with the underlying clips' directions set with [set_clip_direction()](AnimationBuilder::set_clip_direction).
    pub fn set_direction(mut self, direction: AnimationDirection) -> Self {
        self.animation.direction = Some(direction);
        self
    }

    /// Sets the easing of the whole animation.
    ///
    /// If specified, this will be combined with the underlying clips' easings set with [set_clip_easing()](AnimationBuilder::set_clip_easing).
    pub fn set_easing(mut self, easing: Easing) -> Self {
        self.animation.easing = Some(easing);
        self
    }

    /// Creates a new clip in the animation.
    ///
    /// All the clip-related calls ([get_current_clip_id()](AnimationBuilder::get_current_clip_id), [set_clip_duration()](AnimationBuilder::set_clip_duration), ...) will apply to this new clip until another one is created.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # fn f(spritesheet: &Spritesheet) {
    /// let animation = spritesheet
    ///     .create_animation()
    ///     // Clip 1 (default clip, doesn't need to be created explicitly)
    ///     .add_row(3)
    ///     .set_clip_duration(AnimationDuration::PerFrame(1000))
    ///     // Clip 2
    ///     .start_clip()
    ///     .add_row(5)
    ///     .set_clip_duration(AnimationDuration::PerFrame(200))
    ///     .set_clip_repetitions(10)
    ///     .build();
    /// # }
    /// ```
    pub fn start_clip(mut self) -> Self {
        self.animation.clips.push(Clip::empty());
        self
    }

    /// Copies a clip that is already part of the the animation.
    ///
    /// Life after [start_clip()](AnimationBuilder::start_clip), all the clip-related calls will apply to this clip until another one is created/copied.
    ///
    /// # Arguments
    ///
    /// - `clip_id` - the ID of a clip that is already part of the the animation
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # fn f(spritesheet: &Spritesheet) {
    /// let mut first_clip_id = ClipId::dummy();
    ///
    /// let animation = spritesheet
    ///     .create_animation()
    ///     // Clip 1 (the default clip)
    ///     .add_row(0)
    ///     .set_clip_duration(AnimationDuration::PerRepetition(5000))
    ///     .get_current_clip_id(&mut first_clip_id)
    ///     // Clip 2: copied from clip 1, with overridden parameters
    ///     .copy_clip(first_clip_id)
    ///     .set_clip_duration(AnimationDuration::PerRepetition(800))
    ///     .build();
    /// # }
    /// ```
    pub fn copy_clip(mut self, clip_id: ClipId) -> Self {
        let clip = self
            .animation
            .clips
            .iter()
            .find(|clip| clip.id() == clip_id);

        match clip {
            Some(clip) => self.animation.clips.push(clip.clone()),
            None => error!(
                "{CRATE_NAME}: clip {} is not part of the animation",
                clip_id.value
            ),
        }

        self
    }

    fn current_clip(&self) -> &Clip {
        self.animation.clips.last().unwrap()
    }

    fn current_clip_mut(&mut self) -> &mut Clip {
        self.animation.clips.last_mut().unwrap()
    }

    /// Returns the ID of the current clip (ie. the last one added to the animation).
    ///
    /// Clip IDs are useful in a few cases:
    /// - You want to [copy a clip](AnimationBuilder::copy_clip()).
    /// - You want to check if some triggered [animation events](crate::prelude::AnimationEvent) are associated with a specific clip.
    ///
    /// # Arguments
    ///
    /// - `clip_id` - the clip ID that will be assigned the value of the current clip's
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # fn f(spritesheet: &Spritesheet) {
    /// // Create clip IDs
    ///
    /// let mut some_clip_id = ClipId::dummy();
    /// let mut another_clip_id = ClipId::dummy();
    ///
    /// // The IDs are initially invalid but they can be written to with get_current_clip_id()
    ///
    /// let animation = spritesheet
    ///     .create_animation()
    ///     // Clip 1 (the default clip)
    ///     .add_row(0)
    ///     .get_current_clip_id(&mut some_clip_id)
    ///     // Clip 2
    ///     .start_clip()
    ///     .add_row(3)
    ///     .get_current_clip_id(&mut another_clip_id)
    ///     .build();
    ///
    /// // You can use those clip IDs later, for instance with animation events
    /// # }
    /// ```
    pub fn get_current_clip_id(self, id: &mut ClipId) -> Self {
        *id = self.current_clip().id();
        self
    }

    /// Sets the duration of the current clip.
    ///
    /// If specified, this will be combined with the animation's duration set with [set_duration()](AnimationBuilder::set_duration).
    pub fn set_clip_duration(mut self, duration: AnimationDuration) -> Self {
        self.current_clip_mut().duration = Some(duration);
        self
    }

    /// Sets the repetitions of the current clip.
    ///
    /// If specified, this will be combined with the animation's repetitions set with [set_repetitions()](AnimationBuilder::set_repetitions).
    pub fn set_clip_repetitions(mut self, repetitions: usize) -> Self {
        self.current_clip_mut().repetitions = Some(repetitions);
        self
    }

    /// Sets the direction of the current clip.
    ///
    /// If specified, this will be combined with the animation's direction set with [set_direction()](AnimationBuilder::set_direction).
    pub fn set_clip_direction(mut self, direction: AnimationDirection) -> Self {
        self.current_clip_mut().direction = Some(direction);
        self
    }

    /// Sets the easing of the current clip.
    ///
    /// If specified, this will be combined with the animation's easing set with [set_easing()](AnimationBuilder::set_easing).
    pub fn set_clip_easing(mut self, easing: Easing) -> Self {
        self.current_clip_mut().easing = Some(easing);
        self
    }

    /// Adds a marker on a specific frame of the current clip.
    ///
    /// Multiple markers can be added to the same frame.
    ///
    /// # Arguments
    ///
    /// - `marker` - the marker to add
    /// - `frame_index` - the index of the frame to associate the marker with (the index in the clip, not in the whole animation)
    ///
    /// # Example
    /// ```
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # fn f(spritesheet: &Spritesheet) {    ///
    /// let marker1 = Marker::new();
    /// let marker2 = Marker::new();
    ///
    /// let animation = spritesheet
    ///     .create_animation()
    ///     .add_row(0)
    ///     .add_clip_marker(marker1, 1)
    ///     .add_clip_marker(marker2, 4)
    ///     .build();
    /// # }
    /// ```
    pub fn add_clip_marker(mut self, marker: Marker, frame_index: usize) -> Self {
        let clip = self.current_clip_mut();

        if frame_index >= clip.atlas_indices().len() {
            error!(
                "{CRATE_NAME}: frame {frame_index} exceeds the clip size ({})",
                clip.atlas_indices().len()
            );
        } else {
            let frame_markers = clip.markers.entry(frame_index).or_default();

            frame_markers.push(marker);
        }

        self
    }

    /// Adds the frames at the given indices of the spritesheet to the current clip.
    ///
    /// Indices increase from left-to-right and top-to-bottom.
    ///
    /// # Example
    ///
    /// ```
    /// // ┌───┐
    /// // │0 1│
    /// // │2 3│
    /// // └───┘
    ///
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let image = &Handle::default();
    /// let animation = Spritesheet::new(&image, 2, 2)
    ///     .create_animation()
    ///     .add_indices([0, 3])
    ///     .build();
    ///
    /// // This clip will play frames 0 → 3
    ///
    /// let clip = animation.clips().first().unwrap();
    ///
    /// assert_eq!(clip.atlas_indices(), vec![0, 3]);
    /// ```
    pub fn add_indices(mut self, indices: impl IntoIterator<Item = usize>) -> Self {
        for index in indices {
            if index >= self.spritesheet.columns() * self.spritesheet.rows() {
                error!(
                    "{CRATE_NAME}: index {index} exceeds the spritesheet size ({})",
                    self.spritesheet.columns() * self.spritesheet.rows()
                );
            } else {
                self.current_clip_mut().atlas_indices.push(index);
            }
        }

        self
    }

    /// Adds all the frames of the spritesheet to the current clip.
    ///
    /// This is convenient if the whole spritesheet represents a single animation.
    ///
    /// # Example
    ///
    /// ```
    /// // ┌───┐
    /// // │0 1│
    /// // │2 3│
    /// // └───┘
    ///
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let image = &Handle::default();
    /// let animation = Spritesheet::new(&image, 2, 2)
    ///     .create_animation()
    ///     .add_all_cells()
    ///     .build();
    ///
    /// // This clip will play frames 0 → 1 → 2 → 3
    ///
    /// let clip = animation.clips().first().unwrap();
    ///
    /// assert_eq!(clip.atlas_indices(), vec![0, 1, 2, 3]);
    /// ```
    pub fn add_all_cells(mut self) -> Self {
        let cols = self.spritesheet.columns();
        let rows = self.spritesheet.rows();

        self.current_clip_mut()
            .atlas_indices
            .extend(0..(cols * rows));

        self
    }

    /// Adds the frame at coordinates (x, y) of the spritesheet to the current clip.
    ///
    /// # Example
    ///
    /// ```
    /// // ┌───┐
    /// // │0 1│
    /// // │2 3│
    /// // └───┘
    ///
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let image = &Handle::default();
    /// let animation = Spritesheet::new(&image, 2, 2)
    ///     .create_animation()
    ///     .add_cell(1, 1)
    ///     .add_cell(1, 0)
    ///     .build();
    ///
    /// // This clip will play frames 3 → 1
    ///
    /// let clip = animation.clips().first().unwrap();
    ///
    /// assert_eq!(clip.atlas_indices(), vec![3, 1]);
    /// ```
    pub fn add_cell(mut self, x: usize, y: usize) -> Self {
        let index = y * self.spritesheet.columns() + x;

        if index >= self.spritesheet.columns() * self.spritesheet.rows() {
            error!(
                "{CRATE_NAME}: position ({x}, {y}) exceeds the spritesheet size ({}, {})",
                self.spritesheet.columns(),
                self.spritesheet.rows()
            );
        } else {
            self.current_clip_mut().atlas_indices.push(index);
        }

        self
    }

    /// Adds the frames at multiple disjoint coordinates of the spritesheet to the current clip.
    ///
    /// This is convenient if the frames that you're interested in are scattered all over the spritesheet.
    ///
    /// # Arguments
    ///
    /// - `positions` - the list of (x, y) positions, one per frame
    ///
    /// # Example
    ///
    /// ```
    /// // ┌───┐
    /// // │0 1│
    /// // │2 3│
    /// // └───┘
    ///
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let image = &Handle::default();
    /// let animation = Spritesheet::new(&image, 2, 2)
    ///     .create_animation()
    ///     .add_cells([(1, 0), (0, 1)])
    ///     .build();
    ///
    /// // This clip will play frames 1 → 2
    ///
    /// let clip = animation.clips().first().unwrap();
    ///
    /// assert_eq!(clip.atlas_indices(), vec![1, 2]);
    /// ```
    pub fn add_cells(mut self, positions: impl IntoIterator<Item = (usize, usize)>) -> Self {
        for (x, y) in positions {
            self = self.add_cell(x, y);
        }

        self
    }

    /// Adds all the frames in a row of the spritesheet to the current clip.
    ///
    /// This is convenient if some spritesheet row contains a single animation.
    ///
    /// # Arguments
    ///
    /// - `row` - the index of the spritesheet row
    ///
    /// # Example
    ///
    /// ```
    /// // ┌─────┐
    /// // │0 1 2│
    /// // │3 4 5│
    /// // └─────┘
    ///
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let image = &Handle::default();
    /// let animation = Spritesheet::new(&image, 3, 2)
    ///     .create_animation()
    ///     .add_row(1)
    ///     .build();
    ///
    /// // This clip will play frames 3 → 4 → 5
    ///
    /// let clip = animation.clips().first().unwrap();
    ///
    /// assert_eq!(clip.atlas_indices(), vec![3, 4, 5]);
    /// ```
    pub fn add_row(mut self, row: usize) -> Self {
        if row < self.spritesheet.rows() {
            let cols = self.spritesheet.columns();
            let first_index = row * cols;

            self.current_clip_mut()
                .atlas_indices
                .extend(first_index..first_index + cols);
        } else {
            error!(
                "{CRATE_NAME}: row {row} exceeds the spritesheet size ({}, {})",
                self.spritesheet.columns(),
                self.spritesheet.rows()
            );
        }

        self
    }

    /// Adds the frames in a section of a row of the spritesheet to the current clip.
    ///
    /// This is convenient if some spritesheet row contains an animation next to other unrelated frames.
    ///
    /// # Arguments
    ///
    /// - `row` - the index of the spritesheet row
    /// - `column_range` - the range of columns
    ///
    /// # Example
    ///
    /// ```
    /// // ┌─────────┐
    /// // │0 1 2 3 4│
    /// // │5 6 7 8 9│
    /// // └─────────┘
    ///
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let image = &Handle::default();
    /// let spritesheet = Spritesheet::new(&image, 5, 2);
    ///
    /// // This clip will play frames 2 → 3 → 4
    ///
    /// let animation = spritesheet
    ///     .create_animation()
    ///     .add_partial_row(0, 2..)
    ///     .build();
    ///
    /// let clip = animation.clips().first().unwrap();
    ///
    /// assert_eq!(clip.atlas_indices(), vec![2, 3, 4]);
    ///
    /// // This clip will play frames 6 → 7 → 8 → 9
    ///
    /// let animation = spritesheet
    ///     .create_animation()
    ///     .add_partial_row(1, 1..=4)
    ///     .build();
    ///
    /// let clip = animation.clips().first().unwrap();
    ///
    /// assert_eq!(clip.atlas_indices(), vec![6, 7, 8, 9]);
    ///
    /// // This clip will play frames 7 → 8
    ///
    /// let animation = spritesheet
    ///     .create_animation()
    ///     .add_partial_row(1, 2..4)
    ///     .build();
    ///
    /// let clip = animation.clips().first().unwrap();
    ///
    /// assert_eq!(clip.atlas_indices(), vec![7, 8]);
    /// ```
    pub fn add_partial_row<R: RangeBounds<usize>>(mut self, row: usize, column_range: R) -> Self {
        if row >= self.spritesheet.rows() {
            error!(
                "{CRATE_NAME}: row {row} exceeds the spritesheet size ({}, {})",
                self.spritesheet.columns(),
                self.spritesheet.rows()
            );
        } else {
            let first_column = match column_range.start_bound() {
                std::ops::Bound::Included(index) => *index,
                std::ops::Bound::Excluded(_index) => unreachable!(),
                std::ops::Bound::Unbounded => 0,
            };

            let end_column = match column_range.end_bound() {
                std::ops::Bound::Included(index) => (*index).saturating_add(1),
                std::ops::Bound::Excluded(index) => *index,
                std::ops::Bound::Unbounded => self.spritesheet.columns(),
            };

            if first_column >= self.spritesheet.columns() || end_column > self.spritesheet.columns()
            {
                error!(
                    "{CRATE_NAME}: range ({:?}, {:?}) exceeds the spritesheet size ({}, {})",
                    column_range.start_bound(),
                    column_range.end_bound(),
                    self.spritesheet.columns(),
                    self.spritesheet.rows()
                );
            }

            let first_index = row * self.spritesheet.columns()
                + first_column.clamp(0, self.spritesheet.columns().saturating_sub(1));

            let end_index =
                row * self.spritesheet.columns() + end_column.clamp(0, self.spritesheet.columns());

            self.current_clip_mut()
                .atlas_indices
                .extend(first_index..end_index);
        }

        self
    }

    /// Adds all the frames in a column of the spritesheet to the current clip.
    ///
    /// This is convenient if some spritesheet column contains a single animation.
    ///
    /// # Arguments
    ///
    /// - `column` - the index of the spritesheet column
    ///
    /// # Example
    ///
    /// ```
    /// // ┌─────┐
    /// // │0 1 2│
    /// // │3 4 5│
    /// // └─────┘
    ///
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let image = &Handle::default();
    /// let animation = Spritesheet::new(&image, 3, 2)
    ///     .create_animation()
    ///     .add_column(1)
    ///     .build();
    ///
    /// // This clip will play frames 1 → 4
    ///
    /// let clip = animation.clips().first().unwrap();
    ///
    /// assert_eq!(clip.atlas_indices(), vec![1, 4]);
    /// ```
    pub fn add_column(mut self, column: usize) -> Self {
        if column < self.spritesheet.columns() {
            let cols = self.spritesheet.columns();
            let rows = self.spritesheet.rows();

            self.current_clip_mut()
                .atlas_indices
                .extend((0..rows).map(|current_row| column + current_row * cols));
        } else {
            error!(
                "{CRATE_NAME}: column {column} exceeds the spritesheet size ({}, {})",
                self.spritesheet.columns(),
                self.spritesheet.rows()
            );
        }

        self
    }

    /// Adds the frames in a section of a column of the spritesheet to the current clip.
    ///
    /// This is convenient if some spritesheet column contains an animation among other unrelated frames.
    ///
    /// # Arguments
    ///
    /// - `column` - the index of the spritesheet column to add frames for
    /// - `row_range` - the range of rows to add frames for
    ///
    /// # Example
    ///
    /// ```
    /// // ┌─────────┐
    /// // │0   1   2│
    /// // |3   4   5│
    /// // │6   7   8│
    /// // │9  10  11│
    /// // └─────────┘
    ///
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let image = &Handle::default();
    /// let animation = Spritesheet::new(&image, 3, 4)
    ///     .create_animation()
    ///     .add_partial_column(1, 1..)
    ///     .build();
    ///
    /// // This clip will play frames 4 → 7 → 10
    ///
    /// let clip = animation.clips().first().unwrap();
    ///
    /// assert_eq!(clip.atlas_indices(), vec![4, 7, 10]);
    /// ```
    pub fn add_partial_column<R: RangeBounds<usize>>(
        mut self,
        column: usize,
        row_range: R,
    ) -> Self {
        let cols = self.spritesheet.columns();
        let rows = self.spritesheet.rows();

        if column >= cols {
            error!(
                "{CRATE_NAME}: column {column} exceeds the spritesheet size ({}, {})",
                cols, rows
            );
        } else {
            let mut first_row = match row_range.start_bound() {
                std::ops::Bound::Included(index) => *index,
                std::ops::Bound::Excluded(_index) => unreachable!(),
                std::ops::Bound::Unbounded => 0,
            };

            let mut end_row = match row_range.end_bound() {
                std::ops::Bound::Included(index) => (*index).saturating_add(1),
                std::ops::Bound::Excluded(index) => *index,
                std::ops::Bound::Unbounded => rows,
            };

            if first_row >= rows || end_row > rows {
                error!(
                    "{CRATE_NAME}: range ({:?}, {:?}) exceeds the spritesheet size ({}, {})",
                    row_range.start_bound(),
                    row_range.end_bound(),
                    cols,
                    rows
                );
            }

            first_row = first_row.clamp(0, rows.saturating_sub(1));

            end_row = end_row.clamp(0, rows);

            self.current_clip_mut()
                .atlas_indices
                .extend((first_row..end_row).map(|row| row * cols + column))
        }

        self
    }

    /// Adds the frames in an horizontal strip of the spritesheet to the current clip, wrapping from row to row.
    ///
    /// This is convenient if an animation spans several rows of a spritesheet.
    ///
    /// # Arguments
    ///
    /// - `x` - the x position of the beginning of the strip
    /// - `y` - the y position of the beginning of the strip
    /// - `count` - the number of frames to add
    ///
    /// # Example
    ///
    /// ```
    /// // ┌─────┐
    /// // │0 1 2│
    /// // │3 4 5│
    /// // └─────┘
    ///
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let image = &Handle::default();
    /// let animation = Spritesheet::new(&image, 3, 2)
    ///     .create_animation()
    ///     .add_horizontal_strip(2, 0, 3)
    ///     .build();
    ///
    /// // This clip will play frames 2 → 3 → 4
    ///
    /// let clip = animation.clips().first().unwrap();
    ///
    /// assert_eq!(clip.atlas_indices(), vec![2, 3, 4]);
    /// ```
    pub fn add_horizontal_strip(mut self, x: usize, y: usize, count: usize) -> Self {
        let cols = self.spritesheet.columns();
        let rows = self.spritesheet.rows();

        if x > cols || y > rows {
            error!(
                "{CRATE_NAME}: horizontal strip from {x}/{y} exceeds the spritesheet size ({}, {})",
                cols, rows
            );
        } else {
            let first_index = y * cols + x;

            let last_index = (first_index + count).min(cols * rows);

            self.current_clip_mut()
                .atlas_indices
                .extend(first_index..last_index);

            if last_index != first_index + count {
                error!(
                    "{CRATE_NAME}: horizontal strip from {x}/{y} with {count} entries exceeds the spritesheet size ({}, {})",
                    cols, rows
                );
            }
        }

        self
    }

    /// Adds the frames in an vertical strip of the spritesheet to the current clip, wrapping from column to column.
    ///
    /// This is convenient if an animation spans several columns of a spritesheet.
    ///
    /// # Arguments
    ///
    /// - `x` - the x position of the beginning of the strip
    /// - `y` - the y position of the beginning of the strip
    /// - `count` - the number of frames to add
    ///
    /// # Example
    ///
    /// ```
    /// // ┌─────┐
    /// // │0 1 2│
    /// // │3 4 5│
    /// // └─────┘
    ///
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// # let image = &Handle::default();
    /// let animation = Spritesheet::new(&image, 3, 2)
    ///     .create_animation()
    ///     .add_vertical_strip(1, 0, 3)
    ///     .build();
    ///
    /// // This clip will play frames 1 → 4 → 2
    ///
    /// let clip = animation.clips().first().unwrap();
    ///
    /// assert_eq!(clip.atlas_indices(), vec![1, 4, 2]);
    /// ```
    pub fn add_vertical_strip(mut self, x: usize, y: usize, count: usize) -> Self {
        let cols = self.spritesheet.columns();
        let rows = self.spritesheet.rows();

        if x > cols || y > rows {
            error!(
                "{CRATE_NAME}: vertical strip from {x}/{y} exceeds the spritesheet size ({}, {})",
                cols, rows
            );
        } else {
            let available_count = (cols - (x + 1)) * rows + rows - y;

            let clamped_count = count.min(available_count);

            self.current_clip_mut()
                .atlas_indices
                .extend((0..clamped_count).map(|i| {
                    let current_x = x + (y + i) / rows;
                    let current_y = (y + i) % rows;

                    current_y * cols + current_x
                }));

            if clamped_count != count {
                error!(
                    "{CRATE_NAME}: vertical strip from {x}/{y} with {count} entries exceeds the spritesheet size ({}, {})",
                    cols, rows
                );
            }
        }

        self
    }

    /// Creates the final animation, ready to be registered as a Bevy asset.
    ///
    /// The resulting animation handle can then be associated with a [SpritesheetAnimation] component to animate a sprite.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_spritesheet_animation::prelude::*;
    /// fn create_animated_sprite(
    ///     mut commands: Commands,
    ///     assets: Res<AssetServer>,
    ///     mut animations: ResMut<Assets<Animation>>,
    ///     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    /// ) {
    ///     let image = assets.load("character.png");
    ///
    ///     let spritesheet =  Spritesheet::new(&image, 8, 8);
    ///
    ///     let animation = spritesheet
    ///         .create_animation()
    ///         .add_row(6)
    ///         .build();
    ///
    ///     // Register the animation
    ///
    ///     let animation_handle = animations.add(animation);
    ///
    ///     // Spawn the entity with a SpritesheetAnimation component
    ///
    ///     commands.spawn((
    ///         spritesheet.with_size_hint(800, 800).sprite(& mut atlas_layouts),
    ///         SpritesheetAnimation::new(animation_handle),
    ///     ));
    /// }
    /// ```
    pub fn build(self) -> Animation {
        self.into()
    }
}

impl From<AnimationBuilder> for Animation {
    fn from(builder: AnimationBuilder) -> Self {
        builder.animation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod spritesheet {
        use super::*;

        struct Tester {
            spritesheet: Spritesheet,
        }

        impl Tester {
            fn new(columns: usize, rows: usize) -> Self {
                Self {
                    spritesheet: Spritesheet::new(&Handle::default(), columns, rows),
                }
            }

            fn test<F>(&self, f: F, expected_indices: Vec<usize>)
            where
                F: Fn(AnimationBuilder) -> AnimationBuilder,
            {
                let builder = self.spritesheet.clone().create_animation();

                let animation = f(builder).build();

                let indices: Vec<_> = animation
                    .clips()
                    .iter()
                    .flat_map(|clip| clip.atlas_indices().iter())
                    .copied()
                    .collect();

                assert_eq!(indices, expected_indices);
            }
        }

        #[test]
        fn add_indices() {
            let t = Tester::new(3, 2);

            t.test(|b| b.add_indices([]), vec![]);
            t.test(|b| b.add_indices([0]), vec![0]);
            t.test(|b| b.add_indices([1, 3]), vec![1, 3]);
            t.test(|b| b.add_indices([999]), vec![]);
            t.test(|b| b.add_indices([2, 999, 4]), vec![2, 4]);
        }

        #[test]
        fn add_all_cells() {
            let t1 = Tester::new(0, 0);
            t1.test(|b| b.add_all_cells(), vec![]);

            let t2 = Tester::new(2, 3);
            t2.test(|b| b.add_all_cells(), vec![0, 1, 2, 3, 4, 5]);
        }

        #[test]
        fn add_cell() {
            let t = Tester::new(4, 3);

            t.test(|b| b.add_cell(0, 0), vec![0]);
            t.test(|b| b.add_cell(2, 2).add_cell(0, 0), vec![10, 0]);
            t.test(|b| b.add_cell(1000, 1), vec![]);
            t.test(|b| b.add_cell(2, 1000), vec![]);
        }

        #[test]
        fn add_cells() {
            let t = Tester::new(4, 3);

            t.test(|b| b.add_cells([]), vec![]);
            t.test(|b| b.add_cells([(0, 0)]), vec![0]);
            t.test(|b| b.add_cells([(2, 2)]), vec![10]);
            t.test(|b| b.add_cells([(1, 1), (0, 0), (2, 2)]), vec![5, 0, 10]);
            t.test(|b| b.add_cells([(100, 100)]), vec![]);
            t.test(|b| b.add_cells([(1000, 0), (3, 0), (0, 2000)]), vec![3]);
        }

        #[test]
        fn add_row() {
            let t = Tester::new(3, 6);

            t.test(|b| b.add_row(0), vec![0, 1, 2]);
            t.test(|b| b.add_row(3), vec![9, 10, 11]);
            t.test(|b| b.add_row(1000), vec![]);
            t.test(|b| b.add_row(1000).add_row(1), vec![3, 4, 5]);
        }

        #[test]
        fn add_partial_row() {
            let t = Tester::new(5, 4);

            // First row, starting at column 0

            t.test(|b| b.add_partial_row(0, 0..0), vec![]);
            t.test(|b| b.add_partial_row(0, 0..), vec![0, 1, 2, 3, 4]);
            t.test(|b| b.add_partial_row(0, 0..2), vec![0, 1]);
            t.test(|b| b.add_partial_row(0, 0..5), vec![0, 1, 2, 3, 4]);
            t.test(|b| b.add_partial_row(0, 0..100), vec![0, 1, 2, 3, 4]);
            t.test(|b| b.add_partial_row(0, 0..=0), vec![0]);
            t.test(|b| b.add_partial_row(0, 0..=2), vec![0, 1, 2]);
            t.test(|b| b.add_partial_row(0, 0..=100), vec![0, 1, 2, 3, 4]);

            // First row, starting at another column

            t.test(|b| b.add_partial_row(0, 2..2), vec![]);
            t.test(|b| b.add_partial_row(0, 2..), vec![2, 3, 4]);
            t.test(|b| b.add_partial_row(0, 1..3), vec![1, 2]);
            t.test(|b| b.add_partial_row(0, 2..5), vec![2, 3, 4]);
            t.test(|b| b.add_partial_row(0, 2..100), vec![2, 3, 4]);
            t.test(|b| b.add_partial_row(0, 3..=3), vec![3]);
            t.test(|b| b.add_partial_row(0, 3..=4), vec![3, 4]);
            t.test(|b| b.add_partial_row(0, 4..=100), vec![4]);

            // Other rows

            t.test(|b| b.add_partial_row(1, 0..0), vec![]);
            t.test(|b| b.add_partial_row(2, 0..), vec![10, 11, 12, 13, 14]);
            t.test(|b| b.add_partial_row(3, 0..2), vec![15, 16]);
            t.test(|b| b.add_partial_row(1, 0..5), vec![5, 6, 7, 8, 9]);
            t.test(|b| b.add_partial_row(2, 0..100), vec![10, 11, 12, 13, 14]);
            t.test(|b| b.add_partial_row(3, 0..=0), vec![15]);
            t.test(|b| b.add_partial_row(3, 0..=2), vec![15, 16, 17]);
            t.test(|b| b.add_partial_row(2, 0..=100), vec![10, 11, 12, 13, 14]);
            t.test(|b| b.add_partial_row(100, 0..3), vec![]);
            t.test(|b| b.add_partial_row(100, 0..=100), vec![]);
        }

        #[test]
        fn add_column() {
            let t = Tester::new(5, 3);

            t.test(|b| b.add_column(0), vec![0, 5, 10]);
            t.test(|b| b.add_column(1), vec![1, 6, 11]);
            t.test(|b| b.add_column(1000), vec![]);
            t.test(|b| b.add_column(1000).add_column(1), vec![1, 6, 11]);
        }

        #[test]
        fn add_partial_column() {
            let t = Tester::new(3, 4);

            // First column, starting at row 0

            t.test(|b| b.add_partial_column(0, 0..0), vec![]);
            t.test(|b| b.add_partial_column(0, 0..), vec![0, 3, 6, 9]);
            t.test(|b| b.add_partial_column(0, 0..2), vec![0, 3]);
            t.test(|b| b.add_partial_column(0, 0..5), vec![0, 3, 6, 9]);
            t.test(|b| b.add_partial_column(0, 0..100), vec![0, 3, 6, 9]);
            t.test(|b| b.add_partial_column(0, 0..=0), vec![0]);
            t.test(|b| b.add_partial_column(0, 0..=2), vec![0, 3, 6]);
            t.test(|b| b.add_partial_column(0, 0..=100), vec![0, 3, 6, 9]);

            // First column, starting at another row

            t.test(|b| b.add_partial_column(0, 2..2), vec![]);
            t.test(|b| b.add_partial_column(0, 2..), vec![6, 9]);
            t.test(|b| b.add_partial_column(0, 1..3), vec![3, 6]);
            t.test(|b| b.add_partial_column(0, 2..5), vec![6, 9]);
            t.test(|b| b.add_partial_column(0, 2..100), vec![6, 9]);
            t.test(|b| b.add_partial_column(0, 3..=3), vec![9]);
            t.test(|b| b.add_partial_column(0, 2..=4), vec![6, 9]);
            t.test(|b| b.add_partial_column(0, 3..=100), vec![9]);

            // Other columns

            t.test(|b| b.add_partial_column(1, 0..0), vec![]);
            t.test(|b| b.add_partial_column(2, 0..), vec![2, 5, 8, 11]);
            t.test(|b| b.add_partial_column(1, 0..2), vec![1, 4]);
            t.test(|b| b.add_partial_column(1, 0..5), vec![1, 4, 7, 10]);
            t.test(|b| b.add_partial_column(2, 0..100), vec![2, 5, 8, 11]);
            t.test(|b| b.add_partial_column(1, 0..=0), vec![1]);
            t.test(|b| b.add_partial_column(2, 0..=2), vec![2, 5, 8]);
            t.test(|b| b.add_partial_column(2, 0..=100), vec![2, 5, 8, 11]);
            t.test(|b| b.add_partial_column(100, 0..3), vec![]);
            t.test(|b| b.add_partial_column(100, 0..=100), vec![]);
        }

        #[test]
        fn add_horizontal_strip() {
            let t = Tester::new(8, 8);

            t.test(|b| b.add_horizontal_strip(0, 0, 3), vec![0, 1, 2]);
            t.test(|b| b.add_horizontal_strip(6, 0, 4), vec![6, 7, 8, 9]);
            t.test(|b| b.add_horizontal_strip(4, 5, 0), vec![]);
            t.test(|b| b.add_horizontal_strip(100, 0, 1), vec![]);
            t.test(|b| b.add_horizontal_strip(0, 100, 1), vec![]);
            t.test(|b| b.add_horizontal_strip(6, 7, 1000), vec![62, 63]);
        }

        #[test]
        fn add_vertical_strip() {
            let t = Tester::new(4, 3);

            t.test(|b| b.add_vertical_strip(0, 0, 2), vec![0, 4]);
            t.test(|b| b.add_vertical_strip(1, 1, 6), vec![5, 9, 2, 6, 10, 3]);
            t.test(|b| b.add_vertical_strip(100, 0, 1), vec![]);
            t.test(|b| b.add_vertical_strip(0, 100, 1), vec![]);
            t.test(|b| b.add_vertical_strip(3, 0, 1000), vec![3, 7, 11]);
        }
    }

    mod clips {
        use bevy::platform::collections::HashMap;

        use super::*;

        #[test]
        fn id() {
            let mut clip1_id = ClipId::dummy();
            let mut clip2_id = ClipId::dummy();
            let mut clip3_id = ClipId::dummy();

            let animation = Spritesheet::new(&Handle::default(), 8, 8)
                .create_animation()
                .get_current_clip_id(&mut clip1_id)
                .start_clip()
                .get_current_clip_id(&mut clip2_id)
                .start_clip()
                .get_current_clip_id(&mut clip3_id)
                .build();

            assert_eq!(animation.clips().first().unwrap().id(), clip1_id);
            assert_eq!(animation.clips().get(1).unwrap().id(), clip2_id);
            assert_eq!(animation.clips().get(2).unwrap().id(), clip3_id);
        }

        #[test]
        fn atlas_indices() {
            let animation = Spritesheet::new(&Handle::default(), 3, 6)
                .create_animation()
                .add_row(1)
                .start_clip()
                .add_row(2)
                .add_cell(1, 5)
                .add_cell(2, 4)
                .build();

            let clip1 = animation.clips().first().unwrap();
            assert_eq!(clip1.atlas_indices(), [3, 4, 5]);

            let clip2 = animation.clips().last().unwrap();
            assert_eq!(clip2.atlas_indices(), [6, 7, 8, 16, 14]);
        }

        #[test]
        fn parameters() {
            let animation = Spritesheet::new(&Handle::default(), 8, 8)
                .create_animation()
                // Only the last set_clip_xxx() should apply
                .set_clip_direction(AnimationDirection::Backwards)
                .set_clip_direction(AnimationDirection::PingPong)
                .set_clip_duration(AnimationDuration::PerRepetition(1000))
                .set_clip_duration(AnimationDuration::PerFrame(2000))
                .set_clip_repetitions(10)
                .set_clip_repetitions(20)
                .set_clip_easing(Easing::Linear)
                .set_clip_easing(Easing::InOut(EasingVariety::Cubic))
                // New clip with default parameters
                .start_clip()
                // The animation's set_xxx() should not alter the clip's parameters
                .set_direction(AnimationDirection::Backwards)
                .set_duration(AnimationDuration::PerFrame(123))
                .set_repetitions(AnimationRepeat::Times(9))
                .set_easing(Easing::In(EasingVariety::Quintic))
                .build();

            // Animation

            assert!(matches!(
                animation.direction(),
                Some(AnimationDirection::Backwards)
            ));

            assert!(matches!(
                animation.duration(),
                Some(AnimationDuration::PerFrame(123))
            ));

            assert!(matches!(
                animation.repetitions(),
                Some(AnimationRepeat::Times(9))
            ));

            assert!(matches!(
                animation.easing(),
                Some(Easing::In(EasingVariety::Quintic))
            ));

            // Clip 1

            let clip1 = animation.clips().first().unwrap();

            assert!(matches!(
                clip1.direction(),
                Some(AnimationDirection::PingPong)
            ));

            assert!(matches!(
                clip1.duration(),
                Some(AnimationDuration::PerFrame(2000))
            ));

            assert!(matches!(clip1.repetitions(), Some(20)));

            assert!(matches!(
                clip1.easing(),
                Some(Easing::InOut(EasingVariety::Cubic))
            ));

            let clip2 = animation.clips().last().unwrap();

            // Clip 2

            assert!(clip2.direction().is_none());
            assert!(clip2.duration().is_none());
            assert!(clip2.repetitions().is_none());
            assert!(clip2.easing().is_none());
        }

        #[test]
        fn markers() {
            let marker1 = Marker::new();
            let marker2 = Marker::new();
            let marker3 = Marker::new();

            let animation = Spritesheet::new(&Handle::default(), 8, 8)
                .create_animation()
                // Clip 1: add markers to different frames
                .add_row(0)
                .add_clip_marker(marker1, 0)
                .add_clip_marker(marker2, 5)
                // Clip 2: add markers to the same frame
                .start_clip()
                .add_row(3)
                .add_clip_marker(marker3, 2)
                .add_clip_marker(marker2, 2)
                // Add out-of-bound markers, this should not work
                .add_clip_marker(marker1, 100)
                .add_clip_marker(marker1, 999999)
                .build();

            let clip1 = animation.clips().first().unwrap();

            assert_eq!(
                clip1.markers(),
                &HashMap::from([(0, vec![marker1]), (5, vec![marker2])])
            );

            let clip2 = animation.clips().last().unwrap();

            assert_eq!(
                clip2.markers(),
                &HashMap::from([(2, vec![marker3, marker2])])
            );
        }
    }
}
