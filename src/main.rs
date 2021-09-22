mod tab;

use std::error::Error;
use std::process;
use csv::Reader;
use tab::Tab;

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("src/a.csv")?;
    let tab = Tab::from(rdr);
    println!("{}", serde_yaml::to_string(&tab).unwrap());
    rdr = Reader::from_path("src/b.csv")?;
    let tab = Tab::from(rdr);
    println!("{}", serde_yaml::to_string(&tab).unwrap());
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
