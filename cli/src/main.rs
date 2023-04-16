use anyhow::{anyhow, Result};
use clap::{command, Parser, Subcommand};
use creator::create_app;
use std::path::PathBuf;
use utils::{creator, dev, docker, template};
use which::which;

mod config;
mod options;

pub mod utils {
    pub mod creator;
    pub mod dev;
    pub mod docker;
    pub mod template;
}

pub const TEMPLATES_REPO: &str = "https://github.com/filipton/qaf-templates";

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let res = process(args).await;
    if res.is_err() {
        println!("\x1b[31mError: {}\x1b[0m", res.unwrap_err());
    }
}

async fn process(args: CliArgs) -> Result<()> {
    let git_path = which("git").map_err(|_| anyhow!("Git is not installed!"))?;
    let templates_path = template::init(&args, &git_path)?;

    if args.command.is_some() {
        process_cli_args(&args, &templates_path).await?;
        return Ok(());
    }

    create_app(git_path, templates_path)?;
    Ok(())
}

async fn process_cli_args(args: &CliArgs, templates_path: &PathBuf) -> Result<()> {
    if let Some(cmd) = args.command.clone() {
        match cmd {
            Commands::Update => template::update(templates_path)?,
            Commands::Dev => dev::dev().await?,
            Commands::Docker { command } => match command {
                DockerCommands::Build => docker::build().await?,
                DockerCommands::BuildCached => docker::build_cached().await?,
                DockerCommands::Run => docker::run().await?,
            },
        }
    }

    Ok(())
}

/*
 * CLI ARGS
 */
#[derive(Parser, Debug)]
#[command(version)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long)]
    pub templates_path: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Docker {
        #[command(subcommand)]
        command: DockerCommands,
    },
    Dev,
    Update,
}

#[derive(Subcommand, Debug, Clone)]
pub enum DockerCommands {
    Build,
    BuildCached,
    Run,
}
