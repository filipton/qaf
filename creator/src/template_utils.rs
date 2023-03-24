use anyhow::anyhow;
use std::{path::PathBuf, process::Command};
use which::which;

use crate::TEMPLATES_REPO;

pub fn clone_templates(git_path: &PathBuf, home_path: &String) -> anyhow::Result<()> {
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
        println!("\x1b[31mError: {}\x1b[0m", cmd.unwrap_err());
        return Err(anyhow!(""));
    }

    Ok(())
}

pub fn update_templates(templates_path: &PathBuf) {
    let sure = inquire::Confirm::new(
        "Are you sure? (It will delete all local changes on ~/.fnstack folder)",
    )
    .prompt();

    if !sure.unwrap() {
        return;
    }

    println!("Updating templates folder...");

    let git_path = which("git").expect("Git is not installed!");
    let cmd = Command::new(&git_path)
        .current_dir(&templates_path)
        .arg("reset")
        .arg("--hard")
        .output();
    if cmd.is_err() {
        println!("\x1b[31mError: {}\x1b[0m", cmd.unwrap_err());
        return;
    }

    let cmd = Command::new(&git_path)
        .current_dir(&templates_path)
        .arg("pull")
        .arg("--force")
        .output();
    if cmd.is_err() {
        println!("\x1b[31mError: {}\x1b[0m", cmd.unwrap_err());
        return;
    }
}
