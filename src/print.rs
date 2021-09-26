use crate::diff::{Diff, SharedRow};
use prettytable::{Table, Row, Cell, format};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub fn diffs_as_table(diffs: Vec<Diff>, table: &mut Table) -> () {
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
    create_table(vec, table);
}

fn row_to_prettytable_rows(row: SharedRow) -> Cell {
    let mut cell_text = String::new();
    let mut write_nl = false;
    let maxcollen = row.cols.iter().map(|c| c.name.len()).max().unwrap_or(0);
    for (i, val) in row.vals.iter().enumerate() {
        if write_nl {
            cell_text.push_str(LINE_ENDING);
        }
        cell_text.push_str(format!("{0:1$} : {2}", row.cols[i].name.as_str(), maxcollen, val.obj.as_str()).as_str());
        write_nl = true;
    }
    Cell::new(cell_text.as_str())
}

fn create_table(vec: Vec<(Cell, Cell)>, table: &mut Table) -> () {
    for (left, right) in vec {
        let row = Row::new(vec!(left.clone(), right.clone()));
        table.add_row(row);
    }
    table.set_format(format::FormatBuilder::new()
        .column_separator('|')
        .separators(&[format::LinePosition::Intern],
                    format::LineSeparator::new('-', '+', '+', '+'))
        .padding(2, 2)
        .build());
}
