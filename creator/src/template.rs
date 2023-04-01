use anyhow::{anyhow, Result};
use std::{path::PathBuf, process::Command};
use which::which;

use crate::{CliArgs, TEMPLATES_REPO};

pub fn init(args: &CliArgs, git_path: &PathBuf) -> Result<PathBuf> {
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
        clone(&git_path, &home_path)?;
    }

    Ok(templates_path)
}

pub fn clone(git_path: &PathBuf, home_path: &String) -> anyhow::Result<()> {
    println!(
            "Cloning templates... (NOTE: you can update them later by running \"cargo fnstack update\")"
        );

    let cmd = Command::new(&git_path)
        .current_dir(&home_path)
        .arg("clone")
        .arg(TEMPLATES_REPO)
        .arg(".fnstack")
        .output();

    if cmd.is_err() {
        println!("\x1b[31mCloning error: {}\x1b[0m", cmd.unwrap_err());
        return Err(anyhow!(""));
    }

    Ok(())
}

pub fn update(templates_path: &PathBuf) -> Result<()> {
    let sure = inquire::Confirm::new(
        "Are you sure? (It will delete all local changes on ~/.fnstack folder)",
    )
    .prompt()?;

    if !sure {
        return Err(anyhow!("User selected no"));
    }

    println!("Updating templates folder...");

    let git_path = which("git").expect("Git is not installed!");
    Command::new(&git_path)
        .current_dir(&templates_path)
        .arg("reset")
        .arg("--hard")
        .output()?;

    Command::new(&git_path)
        .current_dir(&templates_path)
        .arg("pull")
        .arg("--force")
        .output()?;

    Ok(())
}
