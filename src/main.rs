mod cli;
mod config;
mod engine;
mod error;
mod models;
mod persistence;

use cli::Cli;
use std::process;

fn main() {
    if let Err(e) = Cli::run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
