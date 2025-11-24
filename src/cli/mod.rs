pub mod commands;
pub mod parsers;

use crate::error::TensileResult;
use clap::Parser;

#[derive(Parser)]
#[command(name = "tensile")]
#[command(about = "A structural tension tracker for creative systems builders", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}

impl Cli {
    pub fn run() -> TensileResult<()> {
        let cli = Cli::parse();
        cli.command.execute()
    }
}
