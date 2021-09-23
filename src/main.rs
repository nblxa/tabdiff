mod tab;

use std::error::Error;
use csv::Reader;
use tab::Tab;
use std::fmt;
use std::process::exit;

use console::{style, Style};
use similar::{ChangeTag, TextDiff};

struct Line(Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:>4}", idx + 1),
        }
    }
}

fn example() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 {
        eprintln!("usage: terminal-inline [old] [new]");
        exit(1);
    }

    let left_name = &args[1];
    let right_name = &args[2];

    let rdr = Reader::from_path(left_name)?;
    let tab = Tab::from(rdr);
    let left = serde_yaml::to_string(&tab)?;
    let rdr = Reader::from_path(right_name)?;
    let tab = Tab::from(rdr);
    let right = serde_yaml::to_string(&tab)?;

    println!("{}", left);
    println!("{}", right);

    let diff = TextDiff::from_lines(&left, &right);
    console::set_colors_enabled(true);

    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            println!("{:-^1$}", "-", 80);
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Insert => ("<  ", Style::new().green()),
                    ChangeTag::Delete => (" > ", Style::new().red()),
                    ChangeTag::Equal => ("   ", Style::new().dim()),
                };
                print!(
                    "{}{} |{}",
                    style(Line(change.old_index())).dim(),
                    style(Line(change.new_index())).dim(),
                    s.apply_to(sign).bold(),
                );
                for (emphasized, value) in change.iter_strings_lossy() {
                    if emphasized {
                        print!("{}", s.apply_to(value).underlined().on_black());
                    } else {
                        print!("{}", s.apply_to(value));
                    }
                }
                if change.missing_newline() {
                    println!();
                }
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        exit(1);
    }
}
