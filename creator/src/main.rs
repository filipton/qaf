use anyhow::{anyhow, Result};
use clap::{command, Parser, Subcommand};
use creator::create_app;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::{
    io::{BufRead, BufWriter, Write},
    path::PathBuf,
    process::{Command, Stdio},
};
use template_utils::{clone_templates, update_templates};
use utils::AlternateScreenCleanup;
use which::which;

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
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Update,
    Dev,
}

fn main() {
    // this fixes error while executing `cargo fnstack ...`
    let mut args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 && args[1] == "fnstack" {
        args.remove(0);
    }

    let args = CliArgs::parse_from(args);

    let res = process(args);
    if res.is_err() {
        println!("\x1b[31mError: {}\x1b[0m", res.unwrap_err());
    }
}

fn process(args: CliArgs) -> Result<()> {
    let git_path = which("git").map_err(|_| anyhow!("Git is not installed!"))?;
    let home_path = std::env::var("HOME").map_err(|_| anyhow!("HOME env var not set!"))?;
    let templates_path = PathBuf::from(format!("{}/.fnstack", home_path));

    if !templates_path.exists() {
        clone_templates(&git_path, &home_path).expect("");
    }

    if args.command.is_some() {
        match_commands(&args, &templates_path)?;
        return Ok(());
    }

    create_app(git_path, templates_path)?;
    Ok(())
}

fn match_commands(args: &CliArgs, templates_path: &PathBuf) -> Result<()> {
    if let Some(cmd) = args.command.clone() {
        match cmd {
            Commands::Update => update_templates(templates_path)?,
            Commands::Dev => dev()?,
        }
    }

    Ok(())
}

fn dev() -> Result<()> {
    let _clean_up = AlternateScreenCleanup::new()?;

    // start devs here:
    let mut dev_one = Command::new("cargo")
        .arg("watch")
        .arg("-x")
        .arg("run")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start dev 1");

    dev_one.stdout.take().map(|stdout| {
        std::thread::spawn(move || {
            let stdout = std::io::BufReader::new(stdout);
            for line in stdout.lines() {
                //println!("DEV 1: {}\r", line.unwrap());
            }
        });
    });

    let mut dev_id = 1;
    loop {
        let event = crossterm::event::read()?;
        if let Event::Key(event) = event {
            if event.code == KeyCode::Esc
                || (event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c'))
            {
                break;
            }

            dev_id = match event.code {
                KeyCode::Char('1') => 1,
                KeyCode::Char('2') => 2,
                KeyCode::Char('3') => 3,
                _ => dev_id,
            };
            println!("DEVID: {}\r", dev_id);
        }
    }

    Ok(())
}
