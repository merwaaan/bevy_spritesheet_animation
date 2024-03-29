use bevy_spritesheet_animation::prelude::*;

#[test]
fn position() {
    let sheet = Spritesheet::new(4, 3);

    assert_eq!(sheet.positions([]), Vec::<usize>::new());
    assert_eq!(sheet.positions([(0, 0)]), vec![0]);
    assert_eq!(sheet.positions([(2, 2)]), vec![10]);
    assert_eq!(sheet.positions([(1, 1), (0, 0), (2, 2)]), vec![5, 0, 10]);
    assert_eq!(sheet.positions([(100, 100)]), Vec::<usize>::new());
    assert_eq!(sheet.positions([(1000, 0), (3, 0), (0, 2000)]), vec![3]);
}

#[test]
fn row() {
    let sheet = Spritesheet::new(3, 6);

    assert_eq!(sheet.row(0), vec![0, 1, 2]);
    assert_eq!(sheet.row(3), vec![9, 10, 11]);
    assert_eq!(sheet.row(1000), Vec::<usize>::new());
}

#[test]
fn row_partial() {
    let sheet = Spritesheet::new(5, 4);

    // First row, starting at column 0

    assert_eq!(sheet.row_partial(0, 0..0), Vec::<usize>::new());
    assert_eq!(sheet.row_partial(0, 0..), vec![0, 1, 2, 3, 4]);
    assert_eq!(sheet.row_partial(0, 0..2), vec![0, 1]);
    assert_eq!(sheet.row_partial(0, 0..5), vec![0, 1, 2, 3, 4]);
    assert_eq!(sheet.row_partial(0, 0..100), vec![0, 1, 2, 3, 4]);
    assert_eq!(sheet.row_partial(0, 0..=0), vec![0]);
    assert_eq!(sheet.row_partial(0, 0..=2), vec![0, 1, 2]);
    assert_eq!(sheet.row_partial(0, 0..=100), vec![0, 1, 2, 3, 4]);

    // First row, starting at another column

    assert_eq!(sheet.row_partial(0, 2..2), Vec::<usize>::new());
    assert_eq!(sheet.row_partial(0, 2..), vec![2, 3, 4]);
    assert_eq!(sheet.row_partial(0, 1..3), vec![1, 2]);
    assert_eq!(sheet.row_partial(0, 2..5), vec![2, 3, 4]);
    assert_eq!(sheet.row_partial(0, 2..100), vec![2, 3, 4]);
    assert_eq!(sheet.row_partial(0, 3..=3), vec![3]);
    assert_eq!(sheet.row_partial(0, 3..=4), vec![3, 4]);
    assert_eq!(sheet.row_partial(0, 4..=100), vec![4]);

    // Other rows

    assert_eq!(sheet.row_partial(1, 0..0), Vec::<usize>::new());
    assert_eq!(sheet.row_partial(2, 0..), vec![10, 11, 12, 13, 14]);
    assert_eq!(sheet.row_partial(3, 0..2), vec![15, 16]);
    assert_eq!(sheet.row_partial(1, 0..5), vec![5, 6, 7, 8, 9]);
    assert_eq!(sheet.row_partial(2, 0..100), vec![10, 11, 12, 13, 14]);
    assert_eq!(sheet.row_partial(3, 0..=0), vec![15]);
    assert_eq!(sheet.row_partial(3, 0..=2), vec![15, 16, 17]);
    assert_eq!(sheet.row_partial(2, 0..=100), vec![10, 11, 12, 13, 14]);
    assert_eq!(sheet.row_partial(100, 0..3), Vec::<usize>::new());
    assert_eq!(sheet.row_partial(100, 0..=100), Vec::<usize>::new());
}

#[test]
fn column() {
    let sheet = Spritesheet::new(5, 3);

    assert_eq!(sheet.column(0), vec![0, 5, 10]);
    assert_eq!(sheet.column(1), vec![1, 6, 11]);
    assert_eq!(sheet.column(1000), Vec::<usize>::new());
}

#[test]
fn column_partial() {
    let sheet = Spritesheet::new(3, 4);

    // First column, starting at row 0

    assert_eq!(sheet.column_partial(0, 0..0), Vec::<usize>::new());
    assert_eq!(sheet.column_partial(0, 0..), vec![0, 3, 6, 9]);
    assert_eq!(sheet.column_partial(0, 0..2), vec![0, 3]);
    assert_eq!(sheet.column_partial(0, 0..5), vec![0, 3, 6, 9]);
    assert_eq!(sheet.column_partial(0, 0..100), vec![0, 3, 6, 9]);
    assert_eq!(sheet.column_partial(0, 0..=0), vec![0]);
    assert_eq!(sheet.column_partial(0, 0..=2), vec![0, 3, 6]);
    assert_eq!(sheet.column_partial(0, 0..=100), vec![0, 3, 6, 9]);

    // First column, starting at another row

    assert_eq!(sheet.column_partial(0, 2..2), Vec::<usize>::new());
    assert_eq!(sheet.column_partial(0, 2..), vec![6, 9]);
    assert_eq!(sheet.column_partial(0, 1..3), vec![3, 6]);
    assert_eq!(sheet.column_partial(0, 2..5), vec![6, 9]);
    assert_eq!(sheet.column_partial(0, 2..100), vec![6, 9]);
    assert_eq!(sheet.column_partial(0, 3..=3), vec![9]);
    assert_eq!(sheet.column_partial(0, 2..=4), vec![6, 9]);
    assert_eq!(sheet.column_partial(0, 3..=100), vec![9]);

    // Other columns

    assert_eq!(sheet.column_partial(1, 0..0), Vec::<usize>::new());
    assert_eq!(sheet.column_partial(2, 0..), vec![2, 5, 8, 11]);
    assert_eq!(sheet.column_partial(1, 0..2), vec![1, 4]);
    assert_eq!(sheet.column_partial(1, 0..5), vec![1, 4, 7, 10]);
    assert_eq!(sheet.column_partial(2, 0..100), vec![2, 5, 8, 11]);
    assert_eq!(sheet.column_partial(1, 0..=0), vec![1]);
    assert_eq!(sheet.column_partial(2, 0..=2), vec![2, 5, 8]);
    assert_eq!(sheet.column_partial(2, 0..=100), vec![2, 5, 8, 11]);
    assert_eq!(sheet.column_partial(100, 0..3), Vec::<usize>::new());
    assert_eq!(sheet.column_partial(100, 0..=100), Vec::<usize>::new());
}

#[test]
fn horizontal_strip() {
    let sheet = Spritesheet::new(8, 8);

    assert_eq!(sheet.horizontal_strip(0, 0, 3), vec![0, 1, 2]);
    assert_eq!(sheet.horizontal_strip(6, 0, 4), vec![6, 7, 8, 9]);
    assert_eq!(sheet.horizontal_strip(4, 5, 0), Vec::<usize>::new());
    assert_eq!(sheet.horizontal_strip(6, 7, 1000), vec![62, 63]);
}

#[test]
fn vertical_strip() {
    let sheet = Spritesheet::new(4, 3);
    assert_eq!(sheet.vertical_strip(0, 0, 2), vec![0, 4]);
    assert_eq!(sheet.vertical_strip(1, 1, 6), vec![5, 9, 2, 6, 10, 3]);
    assert_eq!(sheet.vertical_strip(3, 0, 1000), vec![3, 7, 11]);
}
