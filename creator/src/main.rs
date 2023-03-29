use anyhow::{anyhow, Result};
use clap::{command, Parser, Subcommand};
use creator::create_app;
use std::path::PathBuf;
use template_utils::{clone_templates, update_templates};
use tokio::process::Command;
use which::which;

mod config;
mod creator;
mod template_utils;
mod utils;

pub const TEMPLATES_REPO: &str = "https://github.com/filipton/fn-stack-templates";

#[derive(Parser, Debug)]
#[command(version)]
#[command(propagate_version = true)]
struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    templates_path: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Update,
    Dev,
}

#[tokio::main]
async fn main() {
    // this fixes error while executing `cargo fnstack ...`
    let mut args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 && args[1] == "fnstack" {
        args.remove(0);
    }

    let args = CliArgs::parse_from(args);

    let res = process(args).await;
    if res.is_err() {
        println!("\x1b[31mError: {}\x1b[0m", res.unwrap_err());
    }
}

async fn process(args: CliArgs) -> Result<()> {
    let git_path = which("git").map_err(|_| anyhow!("Git is not installed!"))?;
    let home_path = std::env::var("HOME").map_err(|_| anyhow!("HOME env var not set!"))?;
    let mut templates_path = PathBuf::from(format!("{}/.fnstack", home_path));

    if args.templates_path.is_some() {
        let path_str = args
            .templates_path
            .clone()
            .unwrap()
            .replace("~", &home_path);

        templates_path = PathBuf::from(path_str);
    }

    if !templates_path.exists() {
        clone_templates(&git_path, &home_path).expect("");
    }

    if args.command.is_some() {
        match_commands(&args, &templates_path).await?;
        return Ok(());
    }

    create_app(git_path, templates_path)?;
    Ok(())
}

async fn match_commands(args: &CliArgs, templates_path: &PathBuf) -> Result<()> {
    if let Some(cmd) = args.command.clone() {
        match cmd {
            Commands::Update => update_templates(templates_path)?,
            Commands::Dev => dev().await?,
        }
    }

    Ok(())
}

async fn dev() -> Result<()> {
    // i should have some configuration file with file paths for project (backend, frontend)
    let mut backend_watcher = Command::new("cargo")
        .arg("watch")
        .arg("-x")
        .arg("run")
        .spawn()
        .expect("Failed to start backend!");

    loop {
        if let Some(status) = backend_watcher.try_wait()? {
            println!("Backend exited with status: {}", status);
            break;
        }
    }

    backend_watcher.kill().await?;
    Ok(())
}
