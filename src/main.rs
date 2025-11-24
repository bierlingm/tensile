mod cli;
mod config;
mod engine;
mod error;
mod models;
mod persistence;

#[cfg(feature = "tui")]
mod tui;

use cli::Cli;
#[allow(unused_imports)]
use error::{TensileError, TensileResult};
use std::process;

fn main() {
    if let Err(e) = Cli::run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
