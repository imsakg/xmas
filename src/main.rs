#![allow(unused)]

use clap::Parser;

use crate::prelude::*;
use rand::Rng;

mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    let args = Cli::parse();

    let size = &args.size.unwrap_or(12);
    let width = &args.width.unwrap_or(12);
    let term = &args.term.unwrap_or(true);
    println!("{:?} {:?} {:?}", size, width, term);

    Ok(())
}

fn random_change_char(s: &str, c: char) -> String {
    let mut rng = rand::thread_rng();
    let mut s = s.to_string();
    let mut i = 0;
    while i < s.len() {
        if rng.gen_bool(0.5) {
            s.replace_range(i..i + 1, &c.to_string());
        }
        i += 1;
    }
    s
}

fn tree(heigh: Option<usize>, screen_width: Option<usize>) {}
