use anyhow::{anyhow, Result};
use clap::{command, Parser, Subcommand};
use creator::create_app;
use std::{io::Write, os::unix::process::CommandExt, path::PathBuf, process::Command};
use template_utils::{clone_templates, update_templates};
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
    Kill,
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
    _ = which("tmux").map_err(|_| anyhow!("Tmux is not installed!"))?;

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
            Commands::Kill => kill().await?,
        }
    }

    Ok(())
}

async fn kill() -> Result<()> {
    let cargo_toml = PathBuf::from("Cargo.toml");
    let cargo_toml = std::fs::read_to_string(cargo_toml)?;

    let project_name = cargo_toml
        .lines()
        .find(|line| line.starts_with("name = "))
        .unwrap()
        .split(" = ")
        .nth(1)
        .unwrap()
        .replace("\"", "");

    _ = Command::new("tmux")
        .arg("-L")
        .arg("fnstack")
        .arg("kill-server")
        .output();

    _ = Command::new("bash")
        .arg("-c")
        .arg(format!("kill -9 $(pidof {})", project_name))
        .output();

    Ok(())
}

async fn dev() -> Result<()> {
    //let config_path = PathBuf::from("fnstack.json");

    //let config = config::BuildrsConfig::from_file(config_path)?;

    kill().await?;
    _ = Command::new("tmux")
        .arg("-L")
        .arg("fnstack")
        .arg("-f")
        .arg("tmux.conf")
        .arg("new-session")
        .arg("-c")
        .arg("./")
        .arg("-d")
        .arg("cargo watch -x run")
        .output();

    print!("To exit from tmux use C^f + x. Click enter to continue... ");
    std::io::stdout().flush()?;

    let mut string = String::new();
    std::io::stdin().read_line(&mut string)?;
    drop(string);

    _ = Command::new("tmux")
        .arg("-L")
        .arg("fnstack")
        .arg("attach")
        .exec();

    Ok(())
}
