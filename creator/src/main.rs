use std::{path::PathBuf, process::Command};

use anyhow::{anyhow, Result};
use creator::ProjectOptions;
use which::which;

fn main() -> Result<()> {
    let git_path = which("git").expect("Git is not installed!");

    // TODO: change this later
    let templates_path = PathBuf::from("./templates");
    let options = ProjectOptions::prompt();

    println!("Creating project dir...");
    std::fs::create_dir(&options.name)?;

    if options.init_git {
        println!("Initalizing git...");
        let cmd = Command::new(&git_path)
            .current_dir(&options.name)
            .arg("init")
            .output()
            .unwrap();

        if !cmd.status.success() {
            return Err(anyhow!("Error while initalizing git repo!"));
        }

        if options.generate_readme {
            std::fs::copy(
                templates_path.join("README.md"),
                &options.path.join("README.md"),
            )
            .map_err(|_| anyhow!("Error while copying README.md template!"))
            .unwrap();
        }
    }

    println!("{:?}, {:?}", options, git_path);
    Ok(())
}
