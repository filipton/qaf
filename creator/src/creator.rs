use std::{path::PathBuf, process::Command};

use anyhow::{anyhow, Result};
use cargo_fnstack::{Database, ProjectOptions, WebServer, WebsocketServer};

pub fn create_app(git_path: PathBuf, templates_path: PathBuf) -> Result<()> {
    let options = ProjectOptions::prompt()?;

    println!("Creating project dir...");
    std::fs::create_dir(&options.path)?;

    println!("Copying files...");
    walkdir_copy(&templates_path, &options.path, &options)?;

    println!("Initalizing git...");
    init_git(&git_path, &options.path)?;

    println!("DONE!!!");
    Ok(())
}

fn init_git(git_path: &PathBuf, project_path: &PathBuf) -> Result<()> {
    // git init
    let cmd = Command::new(git_path)
        .current_dir(project_path)
        .arg("init")
        .output()?;
    if !cmd.status.success() {
        return Err(anyhow!("Error while initalizing git repo!"));
    }

    // git add .
    let cmd = Command::new(git_path)
        .current_dir(project_path)
        .arg("add")
        .arg(".")
        .output()?;
    if !cmd.status.success() {
        return Err(anyhow!("Error while executing \"git add .\"!"));
    }

    // git commit
    let cmd = Command::new(git_path)
        .current_dir(project_path)
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
            if file_name == ".git" || file_name == "target" {
                continue;
            }

            std::fs::create_dir(&_path_to)?;
            walkdir_copy(&entry.path(), &_path_to, options)?;
        } else if metadata.is_file() {
            let file_str = std::fs::read_to_string(entry.path())?;
            let file_str = generate_file(&file_str, options);

            if file_str == "" {
                continue;
            }

            std::fs::write(path_to.join(file_name), file_str)?;
        }
    }

    Ok(())
}

fn generate_file(string: &str, options: &ProjectOptions) -> String {
    let mut out = String::new();
    let mut ifs = Vec::new();

    for line in string.lines() {
        let line_trimmed = line.trim();

        if line_trimmed.starts_with("#[[ENDIF]]")
            || line_trimmed.starts_with("[[ENDIF]]*/")
            || line_trimmed.starts_with("//[[ENDIF]]")
        {
            ifs.pop();
            continue;
        }

        if line_trimmed.starts_with("#[[IF ")
            || line_trimmed.starts_with("/*[[IF ")
            || line_trimmed.starts_with("//[[IF ")
        {
            // 0 - #[[IF|//[[IF    1 - DATABASE|WEBSOCKETS|etc...    2 - value
            let line_trimmed = line_trimmed.replace("]]", "");
            let splitted_args: Vec<&str> = line_trimmed.split(" ").collect();
            if splitted_args.len() != 3 {
                panic!("Invalid IF statement!");
            }

            let if_statement = match splitted_args[1] {
                "DATABASE" => Database::from_str(splitted_args[2]) == options.database,
                "WEBSOCKET" => {
                    WebsocketServer::from_str(splitted_args[2]) == options.websocket_server
                }
                "WEBSERVER" => {
                    WebServer::from_str(splitted_args[2]).unwrap_or(WebServer::Actix)
                        == options.web_server
                }
                _ => false,
            };

            // If the last IF statement was false, then this one is also false
            ifs.push(if_statement && ifs.last() != Some(&false));

            continue;
        }

        if ifs.len() > 0 && ifs.last() == Some(&false) {
            continue;
        }

        out.push_str(&format!("{}\n", line));
    }

    if ifs.len() > 0 {
        panic!("Unclosed IF statement!");
    }

    out.replace(
        "rust_project_name_t",
        &options.name.replace("-", "_").to_lowercase(),
    )
    .replace("project_name_t", &options.name)
}
