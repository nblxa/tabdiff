use crate::diff::{Diff, SharedRow};
use prettytable::{Table, Row, Cell, format};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub fn diffs_as_table(diffs: Vec<Diff>, max_width: usize) -> Table {
    let mut vec = vec!();
    for diff in diffs {
        let left = diff.left()
            .map(|row| row_to_prettytable_rows(row))
            .or(diff.right()
                .map(|_| Cell::default()))
            .unwrap();
        let right = diff.right()
            .map(|row| row_to_prettytable_rows(row))
            .or(diff.left()
                .map(|_| Cell::default()))
            .unwrap();
        vec.push((left, right));
    }
    create_table(vec, max_width)
}

fn row_to_prettytable_rows(row: SharedRow) -> Cell {
    let mut cell_text = String::new();
    let mut write_nl = false;
    for (i, val) in row.vals.iter().enumerate() {
        if write_nl {
            cell_text.push_str(LINE_ENDING);
        }
        cell_text.push_str(row.cols[i].name.as_str());
        cell_text.push_str(": ");
        cell_text.push_str(val.obj.as_str());
        write_nl = true;
    }
    Cell::new(cell_text.as_str())
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

fn create_table(vec: Vec<(Cell, Cell)>, max_width: usize) -> Table {
    let half_width = max_width / 2;
    let mut table = Table::new();
    for (left, right) in vec {
        let row = Row::new(vec!(left.clone(), right.clone()));
        table.add_row(row);
    }
    table.set_format(format::FormatBuilder::new()
        .column_separator('│')
        .borders('│')
        .separators(&[format::LinePosition::Top],
                    format::LineSeparator::new('─', '┬', '┌', '┐'))
        .separators(&[format::LinePosition::Intern],
                    format::LineSeparator::new('─', '┼', '├', '┤'))
        .separators(&[format::LinePosition::Bottom],
                    format::LineSeparator::new('─', '┴', '└', '┘'))
        .padding(1, 1)
        .build());
    table
}
