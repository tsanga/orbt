use clap::{Parser, Subcommand};

/// Command-line interface for running the orbt application.
#[derive(Parser)]
#[command(name = "orbt", author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub debug: bool,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    // todo: support non-interactive creation command
    Start,
    Stop,
}
