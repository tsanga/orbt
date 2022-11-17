use clap::{Parser, Subcommand, CommandFactory};

/// Command-line interface for running the orbt application.
#[derive(Parser)]
#[command(name = "orb", author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub debug: bool,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn help() -> std::io::Result<()> {
        Self::command().print_help()?;
        Ok(())
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Start {
        #[arg(short, long)]
        name: String,
    }
}