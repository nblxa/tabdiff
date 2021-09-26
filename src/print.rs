use crate::diff::{Diff, SharedRow};
use prettytable::{Table, Row, Cell};
use similar::ChangeTag;
use console::Style;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub fn diffs_as_table(diffs: Vec<Diff>, color: bool) -> Table {
    let mut vec = vec!();
    for diff in diffs {
        let mut left = diff.left()
            .map(|row| row_to_diff_text(row))
            .or(diff.right()
                .map(|_| String::new()))
            .unwrap();
        let mut right = diff.right()
            .map(|row| row_to_diff_text(row))
            .or(diff.left()
                .map(|_| String::new()))
            .unwrap();
        let descr = match diff {
            Diff::Left { .. } => "left",
            Diff::Right { .. } => "right",
            Diff::Both { left, right } => if left.vals == right.vals { "eq" } else { "diff" },
        };
        if color {
            let (left_color, right_color) = colorize(&left, &right);
            left = left_color;
            right = right_color;
        }
        vec.push((descr, left, right));
    }
    create_table(vec)
}

fn row_to_diff_text(row: SharedRow) -> String {
    let mut diff_text = String::new();
    let mut write_nl = false;
    let maxcollen = row.cols.iter().map(|c| c.name.len()).max().unwrap_or(0);
    for (i, val) in row.vals.iter().enumerate() {
        if write_nl {
            diff_text.push_str(LINE_ENDING);
        }
        diff_text.push_str(format!("{0:1$} : {2}", row.cols[i].name.as_str(), maxcollen, val.obj.as_str()).as_str());
        write_nl = true;
    }
    diff_text
}

fn create_table(vec: Vec<(&str, String, String)>) -> Table {
    let mut table = Table::new();
    for (descr, left, right) in vec {
        let row = Row::new(vec!(Cell::new(descr), Cell::new(left.as_str()), Cell::new(right.as_str())));
        table.add_row(row);
    }
    table
}

fn colorize(left_ref: &String, right_ref: &String) -> (String, String) {
    let diff = similar::TextDiff::from_lines(left_ref, right_ref);
    let mut left = String::new();
    let mut right = String::new();
    for op in diff.ops() {
        for change in diff.iter_inline_changes(op) {
            let s = match change.tag() {
                ChangeTag::Delete => Style::new().red(),
                ChangeTag::Insert => Style::new().green(),
                ChangeTag::Equal => Style::new(),
            };
            let mut t = String::new();
            for (emphasized, value) in change.iter_strings_lossy() {
                if emphasized {
                    t.push_str(format!("{}", s.apply_to(value).underlined()).as_str());
                } else {
                    t.push_str(format!("{}", s.apply_to(value)).as_str());
                }
            }
            match change.tag() {
                ChangeTag::Delete => left.push_str(t.as_str()),
                ChangeTag::Insert => right.push_str(t.as_str()),
                ChangeTag::Equal => {
                    left.push_str(t.as_str());
                    right.push_str(t.as_str());
                },
            }
        }
    }
    (left, right)
}
