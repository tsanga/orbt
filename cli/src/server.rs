use crate::prelude::*;
use console::style;
use fork::{fork, Fork};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;

use nix::unistd::Pid;
use std::future::Future;
use std::process::ExitStatus;
use std::sync::mpsc;
use std::time::Duration;

lazy_static! {
    static ref CHECK: String = style("✔".to_string()).magenta().to_string();
}

#[derive(Debug)]
pub struct Config {
    pub(crate) host: String,
    pub(crate) hostname: Option<String>,
}

pub const PID_LOC: &'static str = "/tmp/.orbt.daemon.pid";

pub const REPO_LOCAL: &'static str = "/tmp/.orbt";
pub const REPO_REMOTE: &'static str = "https://github.com/tsanga/orbt";

pub fn get_spinner(do_tick: bool) -> ProgressBar {
    let pb = ProgressBar::new_spinner();

    if do_tick {
        pb.enable_steady_tick(Duration::from_millis(80));
    }

    pb.set_style(
        ProgressStyle::with_template("{spinner:.magenta}{msg:.bold} {elapsed:.magenta.dim}")
            .unwrap()
            .tick_strings(&[
                "| ",
                "/ ",
                "- ",
                "\\ ",
                "",
            ]),
    );

    pb
}

fn update_or_get_web_files() -> Result<()> {
    // if debug build, assume it's being ran from within monorepo, and clone self
    // instead of cloning from remote
    let repo = if cfg!(debug_assertions) {
        "../"
    } else {
        REPO_REMOTE
    };

    // todo: @alex, don't pipe stdout to /dev/null if debug env var set

    let spinner = get_spinner(true);

    let mut action = "Pulling web app...";
    if std::path::Path::new(REPO_LOCAL).exists() {
        action = "Updating web app...";
        spinner.set_message(action);
        let mut command = std::process::Command::new("git");
        command.arg("pull");
        command.current_dir(REPO_LOCAL);
        command.stdout(std::process::Stdio::null());
        command.spawn()?.wait()?;
    } else {
        spinner.set_message(action);
        std::process::Command::new("git")
            .arg("clone")
            .arg(repo)
            .arg(REPO_LOCAL)
            .stdout(std::process::Stdio::null())
            .spawn()?
            .wait()?;
    }

    spinner.finish_with_message(format!("{} {}", *CHECK, style(action).bold()));

    Ok(())
}

fn build_web() -> Result<()> {
    let spinner = get_spinner(true);
    spinner.set_message("Building web app...");

    let web_path = format!("{}/web", REPO_LOCAL);

    let mut install_dependencies_command = std::process::Command::new("yarn");
    install_dependencies_command.arg("install");
    install_dependencies_command.stdout(std::process::Stdio::null());
    install_dependencies_command.current_dir(&web_path);

    install_dependencies_command.spawn()?.wait()?;

    let mut build_command = std::process::Command::new("yarn");
    build_command.arg("build");
    build_command.stdout(std::process::Stdio::null());
    build_command.stderr(std::process::Stdio::null());
    build_command.current_dir(web_path);
    build_command.spawn()?.wait()?;

    spinner.finish_with_message(format!(
        "{} {}",
        *CHECK,
        style("Building web app...").bold()
    ));
    Ok(())
}

async fn start_web_server<Fut>(callback: impl FnOnce() -> Fut) -> Result<ExitStatus>
where
    Fut: Future<Output = ()>,
{
    let web_path = format!("{}/web", REPO_LOCAL);
    let mut command = tokio::process::Command::new("yarn");

    command.arg("start");
    command.current_dir(web_path);
    command.stdout(std::process::Stdio::null());
    command.stderr(std::process::Stdio::null());

    let mut future = command.spawn().unwrap();
    let futures = futures::future::join(future.wait(), callback()).await;

    futures.0.map_err(anyhow::Error::from)
}

pub fn start(_config: Config) -> Result<()> {
    update_or_get_web_files()?;
    build_web()?;

    match fork() {
        Ok(Fork::Parent(child)) => {
            std::fs::write(PID_LOC, format!("{}", child))?;

            nix::sys::wait::waitpid(Some(Pid::from_raw(child)), None)?;
        }

        Ok(Fork::Child) => {
            nix::unistd::setsid()?;

            use tokio::runtime::Runtime;

            let rt = Runtime::new()?;
            let (tx, rx) = mpsc::channel();
            let tx2 = tx.clone();

            let api_spinner = get_spinner(false);
            let web_spinner = get_spinner(false);

            let multi_spinner = MultiProgress::new();
            let api_spinner = multi_spinner.add(api_spinner);
            let web_spinner = multi_spinner.add(web_spinner);

            api_spinner.set_message("Starting API...");
            web_spinner.set_message("Starting web app...");

            api_spinner.enable_steady_tick(Duration::from_millis(80));
            web_spinner.enable_steady_tick(Duration::from_millis(80));

            let api_server = api::start_api_server(Some(|| async move {
                api_spinner.finish_with_message(format!(
                    "{} {}",
                    *CHECK,
                    style("Starting API...").bold()
                ));

                tx.send(0).expect("Failed to send message to mpsc receiver");
            }));

            let web_server = start_web_server(|| async move {
                web_spinner.finish_with_message(format!(
                    "{} {}",
                    *CHECK,
                    style("Starting web app...").bold()
                ));

                tx2.send(0)
                    .expect("Failed to send message to mpsc receiver");
            });

            let futures = rt.block_on(futures::future::join(
                futures::future::join(api_server, web_server),
                async move {
                    let mut c = 0;
                    while let Ok(_) = rx.recv() {
                        if c == 1 {
                            let spinner = get_spinner(false);
                            let spinner = multi_spinner.add(spinner);

                            spinner.set_message("Creating room...");
                            spinner.enable_steady_tick(Duration::from_millis(80));

                            tokio::time::sleep(Duration::new(5, 0)).await;

                            spinner.finish_with_message(format!(
                                "{} {}",
                                *CHECK,
                                style("Creating room...").bold()
                            ));
                        }

                        c += 1;
                    }
                },
            ));

            futures.0 .0.map_err(|_| futures.0 .1.err()).unwrap();
        }

        Err(_) => println!("Fork failed"),
    };

    Ok(())
}
