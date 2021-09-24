use crate::diff::{Diff, SharedRow};
use prettytable::{Table, Row, Cell};

pub fn diffs_as_table(diffs: Vec<Diff>, max_width: usize) -> Table {
    let mut vec = vec!();
    for diff in diffs {
        let left = diff.left()
            .map(|row| row_to_prettytable_rows(row))
            .or(diff.right()
                .map(|other| row_to_empty(other.vals.len())))
            .unwrap();
        let right = diff.right()
            .map(|row| row_to_prettytable_rows(row))
            .or(diff.left()
                .map(|other| row_to_empty(other.vals.len())))
            .unwrap();
        vec.push((left, right));
    }
    create_table(vec, max_width)
}

fn row_to_prettytable_rows(row: SharedRow) -> Vec<(Cell, Cell)> {
    let mut vec = Vec::with_capacity(row.vals.len());
    for (i, val) in row.vals.iter().enumerate() {
        vec.push((
            Cell::new(row.cols[i].name.as_str()),
            Cell::new(val.obj.as_str()),
        ));
    }
    vec
}

fn row_to_empty(height: usize) -> Vec<(Cell, Cell)> {
    let mut vec = Vec::with_capacity(height);
    for _ in 0 .. height {
        vec.push((
            Cell::new(""),
            Cell::new(""),
        ));
    }
    vec
}

fn create_table(vec: Vec<(Vec<(Cell, Cell)>, Vec<(Cell, Cell)>)>, max_width: usize) -> Table {
    let half_width = max_width / 2;
    let mut table = Table::new();
    for (left, right) in vec {
        for i in 0 .. left.len() {
            let row = Row::new(vec!(left[i].0.clone(), left[i].1.clone(), right[i].0.clone(), right[i].1.clone()));
            table.add_row(row);
        }
    }
    table
}
