use std::{path::PathBuf, process::Command};

use anyhow::{anyhow, Result};
use cargo_fnstack::ProjectOptions;

pub fn create_app(git_path: PathBuf, templates_path: PathBuf) -> Result<()> {
    let options = ProjectOptions::prompt()?;

    println!("Creating project dir...");
    std::fs::create_dir(&options.path)?;

    println!("Copying files...");
    walkdir_copy(&templates_path, &options.path, &options)?;

    if options.init_git {
        println!("Initalizing git...");
        init_git(git_path, &options)?;
    }

    println!("DONE!!!");
    Ok(())
}

fn init_git(git_path: PathBuf, options: &ProjectOptions) -> Result<()> {
    // git init
    let cmd = Command::new(&git_path)
        .current_dir(&options.name)
        .arg("init")
        .output()?;
    if !cmd.status.success() {
        return Err(anyhow!("Error while initalizing git repo!"));
    }

    // git add .
    let cmd = Command::new(&git_path)
        .current_dir(&options.name)
        .arg("add")
        .arg(".")
        .output()?;
    if !cmd.status.success() {
        return Err(anyhow!("Error while executing \"git add .\"!"));
    }

    // git commit
    let cmd = Command::new(&git_path)
        .current_dir(&options.name)
        .arg("commit")
        .arg("-am")
        .arg("\"Initial commit\"")
        .output()?;
    if !cmd.status.success() {
        return Err(anyhow!("Error while commiting!"));
    }

    Ok(())
}

fn walkdir_copy(path_from: &PathBuf, path_to: &PathBuf, options: &ProjectOptions) -> Result<()> {
    for entry in path_from.read_dir()? {
        let entry = entry?;
        let file_name = entry.file_name().into_string().unwrap();
        let _path_to = path_to.join(&file_name);

        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            if file_name == ".git" {
                continue;
            }

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
