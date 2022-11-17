mod cli;
mod program;

pub mod prelude {
    pub use anyhow::Result;
}

use cli::Cli;
use prelude::*;
use clap::Parser;

fn main() -> Result<()> {
    execute_cli()?;
    Ok(())
}

fn execute_cli() -> Result<()> {
    let cli = Cli::parse();
    if let Some(cmd) = &cli.command {
        match cmd {
            cli::Commands::Start { name } => {
                println!("Starting with name: {}", name);
            }
        }
    } else {
        Cli::help()?;
    }
    Ok(())
}