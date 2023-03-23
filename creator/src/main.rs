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
    std::fs::create_dir(&options.path)?;

    println!("Copying files...");
    walkdir_copy(&templates_path, &options.path, &options)?;

    if options.init_git {
        println!("Initalizing git...");
        let cmd = Command::new(&git_path)
            .current_dir(&options.name)
            .arg("init")
            .output()?;

        if !cmd.status.success() {
            return Err(anyhow!("Error while initalizing git repo!"));
        }
    }

    println!("DONE!!!");
    Ok(())
}

fn walkdir_copy(path_from: &PathBuf, path_to: &PathBuf, options: &ProjectOptions) -> Result<()> {
    for entry in path_from.read_dir()? {
        let entry = entry?;
        let file_name = entry.file_name().into_string().unwrap();
        let _path_to = path_to.join(&file_name);

        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            std::fs::create_dir(&_path_to)?;
            walkdir_copy(&entry.path(), &_path_to, options)?;
        } else if metadata.is_file() {
            if file_name.starts_with("[GEN]") {
                let file_str = std::fs::read_to_string(entry.path())?;
                let file_str = inject_project_options(&file_str, options);

                std::fs::write(path_to.join(file_name.replace("[GEN]", "")), file_str)?;
            } else {
                std::fs::copy(entry.path(), _path_to)?;
            }
        }
    }

    Ok(())
}

fn inject_project_options(string: &str, options: &ProjectOptions) -> String {
    string
        .replace("[[PROJECT_NAME]]", &options.name)
        .replace("[[RUST_PROJECT_NAME]]", &options.name.replace("-", "_"))
}
