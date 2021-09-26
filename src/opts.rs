use std::str::{FromStr, ParseBoolError};
use clap::{Clap, AppSettings};

/// Compare two CSV files record by record, field by field, assuming that each of them has
/// a set of key columns.
#[derive(Clap)]
#[clap(version = "1.0", author = "Pavel Mitrofanov <https://github.com/nblxa>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// Left file
    pub left: String,
    /// Right file
    pub right: String,
    /// Left file's key fields separated by commas. Default: all columns.
    #[clap(short, long, default_value="")]
    pub left_keys: Keys,
    /// Right file's key fields separated by commas. Default: all columns.
    #[clap(short, long, default_value="")]
    pub right_keys: Keys,
    /// Colorize output
    #[clap(long, default_value="true")]
    pub color: BoolDefaultTrue,
}

pub struct Keys {
    pub keys: Vec<String>,
}

impl FromStr for Keys {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut keys = vec!();
        if !s.trim().is_empty() {
            for val in s.split(',') {
                keys.push(val.trim().into());
            }
        }
        Ok(Keys { keys })
    }
}

#[derive(Copy, Clone)]
pub struct BoolDefaultTrue {
    value: bool,
}

impl FromStr for BoolDefaultTrue {
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            Ok(BoolDefaultTrue { value: true })
        } else {
            Ok(BoolDefaultTrue { value: bool::from_str(s)? })
        }
    }
}

impl Into<bool> for BoolDefaultTrue {
    fn into(self) -> bool {
        self.value
    }
}
