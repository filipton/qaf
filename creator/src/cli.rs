use clap::{command, Parser, Subcommand};

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
