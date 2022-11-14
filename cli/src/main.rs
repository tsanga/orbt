mod cli;

pub mod prelude {
    pub type Result<T> = anyhow::Result<T>;
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