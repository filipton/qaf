use std::{path::PathBuf, process::Command};

use anyhow::{anyhow, Result};
use cargo_fnstack::ProjectOptions;
use which::which;

fn main() {
    let res = create_app();
    if res.is_err() {
        println!("\x1b[31mError: {}\x1b[0m", res.unwrap_err());
    }
}

fn create_app() -> Result<()> {
    let git_path = which("git").expect("Git is not installed!");

    // TODO: change this path later
    let home_path = std::env::var("HOME").expect("HOME env var not set!");
    let templates_path =
        PathBuf::from(format!("{}/Documents/Github/fn-stack/templates", home_path));
    let options = ProjectOptions::prompt();

    println!("Creating project dir...");
    std::fs::create_dir(&options.name)?;

    if options.init_git {
        println!("Initalizing git...");
        let cmd = Command::new(&git_path)
            .current_dir(&options.name)
            .arg("init")
            .output()?;

        if !cmd.status.success() {
            return Err(anyhow!("Error while initalizing git repo!"));
        }

        if options.generate_readme {
            std::fs::copy(
                templates_path.join("README.md"),
                &options.path.join("README.md"),
            )
            .map_err(|_| anyhow!("Error while copying README.md template!"))?;
        }
    }

    println!("{:?}, {:?}", options, git_path);
    Ok(())
}
