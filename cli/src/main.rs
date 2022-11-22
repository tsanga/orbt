mod cli;
mod server;
mod wizard;

pub mod prelude {
    pub use anyhow::Result;
}

use clap::Parser;
use cli::{Cli, Command};
use console::style;
use dialoguer::Confirm;
use prelude::*;
use server::{get_spinner, PID_LOC};

fn main() -> Result<()> {
    execute_cli()?;
    Ok(())
}

fn kill_old_server(pid: Option<String>) -> Result<()> {
    let mut is_running = false;
    if let Some(pid) = pid {
        is_running = true;

        std::fs::remove_file(PID_LOC).ok();

        let spinner = get_spinner(true);
        spinner.set_message("Shutting down services...");

        std::process::Command::new("kill")
            .args(["-s", "TERM", &format!("-{}", &pid)])
            .status()?;

        spinner.finish_with_message("Successfully shut down orbt services.");
    }

    if !is_running {
        println!("{}", style("No orbt services found running.").red());
    }

    Ok(())
}

fn execute_cli() -> Result<()> {
    let cli = Cli::parse();

    let daemon_pid = std::fs::read_to_string(PID_LOC);

    if let Some(cmd) = &cli.command {
        match cmd {
            Command::Stop => {
                return kill_old_server(daemon_pid.ok());
            }
            Command::Start => {}
        }
    }

    if daemon_pid.is_ok() {
        println!(
            "{}",
            style("Another instance of orbt is already running!")
                .red()
                .bold()
        );
        let kill = Confirm::with_theme(&*wizard::THEME)
            .with_prompt("Do you want to stop this instance and start a new one?")
            .interact()?;

        if !kill {
            return Ok(());
        }

        kill_old_server(daemon_pid.ok())?;
    }

    if let Some(config) = wizard::init()? {
        server::start(config)?;
    }

    Ok(())
}
