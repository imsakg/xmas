//! Crate prelude

pub use crate::error::Error;
use clap::Parser;

const BALL: char = '⏺';

enum COLOR {
    blue,
    yellow,
    cyan,
    green,
    magenta,
    white,
    red,
}
impl COLOR {
    fn to_string(&self) -> String {
        match self {
            COLOR::blue => "\033[94m".to_string(),
            COLOR::yellow => "\033[93m".to_string(),
            COLOR::cyan => "\033[96m".to_string(),
            COLOR::green => "\033[92m".to_string(),
            COLOR::magenta => "\033[95m".to_string(),
            COLOR::white => "\033[97m".to_string(),
            COLOR::red => "\033[91m".to_string(),
        }
    }
}

const STAR: char = '★';

pub type Result<T> = core::result::Result<T, Error>;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    /// The pattern to look for
    pub term: Option<bool>,
    /// The path to the file to read
    pub size: Option<usize>,
    pub width: Option<usize>,
}

// Generic Wrapper tuple struct for newtype pattern
pub struct W<T>(pub T);

// Personal preference
pub use std::format as f;
