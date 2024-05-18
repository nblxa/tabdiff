mod tab;
mod diff;
mod print;
mod opts;

use std::error::Error;
use csv::Reader;
use tab::Tab;
use std::process::exit;
use diff::Diff;
use opts::Opts;
use clap::Parser;

fn print_diff(opts: Opts) -> Result<(), Box<dyn Error>> {
    let rdr = Reader::from_path(&opts.left)?;
    let left = Tab::init(rdr, &opts.left_keys.keys);
    let rdr = Reader::from_path(&opts.right)?;
    let right = Tab::init(rdr, &opts.right_keys.keys);

    let diffs = Diff::create_diffs(left, right);
    let mut table = print::diffs_as_table(diffs, opts.color.into());
    table.set_titles(prettytable::Row::new(vec!(
        prettytable::Cell::new("Diff"),
        prettytable::Cell::new(opts.left.as_str()),
        prettytable::Cell::new(opts.right.as_str()),
    )));
    table.printstd();
    Ok(())
}

fn main() {
    let opts: Opts = Opts::parse();
    console::set_colors_enabled(opts.color.into());
    if let Err(err) = print_diff(opts) {
        println!("error running print_diff: {}", err);
        exit(1);
    }
}
