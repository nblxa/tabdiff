mod tab;
mod diff;
mod print;

use std::error::Error;
use csv::Reader;
use tab::Tab;
use std::process::exit;
use diff::Diff;

fn example() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 {
        eprintln!("usage: tabdiff [left] [right]");
        exit(1);
    }

    let left_name = &args[1];
    let right_name = &args[2];

    let rdr = Reader::from_path(left_name)?;
    let left = Tab::from(rdr);
    let rdr = Reader::from_path(right_name)?;
    let right = Tab::from(rdr);

    let diffs = Diff::create_diffs(left, right);
    let table = print::diffs_as_table(diffs, 120);
    table.printstd();
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        exit(1);
    }
}
