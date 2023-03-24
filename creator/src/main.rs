use anyhow::Result;
use clap::{command, Parser, Subcommand};
use creator::create_app;
use std::path::PathBuf;
use template_utils::{clone_templates, update_templates};
use which::which;

mod creator;
mod template_utils;

pub const TEMPLATES_REPO: &str = "https://github.com/filipton/fn-stack-templates";

#[derive(Parser, Debug)]
#[command(version)]
#[command(propagate_version = true)]
struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Update,
}

fn main() {
    // this fixes error while executing `cargo fnstack ...`
    let mut args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 && args[1] == "fnstack" {
        args.remove(0);
    }

    let args = CliArgs::parse_from(args);

    let git_path = which("git").expect("Git is not installed!");
    let home_path = std::env::var("HOME").expect("HOME env var not set!");
    let templates_path = PathBuf::from(format!("{}/.fnstack", home_path));

    if !templates_path.exists() {
        clone_templates(&git_path, &home_path).expect("");
    }

    if args.command.is_some() {
        let res = match_commands(&args, &templates_path);
        if res.is_err() {
            println!("\x1b[31mError: {}\x1b[0m", res.unwrap_err());
        }

        return;
    }

    let res = create_app(git_path, templates_path);
    if res.is_err() {
        println!("\x1b[31mError: {}\x1b[0m", res.unwrap_err());
    }
}

fn match_commands(args: &CliArgs, templates_path: &PathBuf) -> Result<()> {
    if let Some(cmd) = args.command.clone() {
        match cmd {
            Commands::Update => update_templates(templates_path)?,
        }
    }

    Ok(())
}
