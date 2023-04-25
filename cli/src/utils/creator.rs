use std::{path::PathBuf, process::Command};

use crate::options::{Database, ProjectOptions, WebServer, WebsocketServer};
use anyhow::{anyhow, Result};

use crate::config::QafConfig;

pub fn create_app(git_path: PathBuf, templates_path: PathBuf) -> Result<()> {
    let options = ProjectOptions::prompt()?;

    println!("Creating project dir...");
    std::fs::create_dir(&options.path)?;

    println!("Creating config file...");
    let config = QafConfig::generate(&options)?;
    QafConfig::to_file(config, options.path.join("qaf.json"))?;

    println!("Copying files...");
    walkdir_copy(
        &templates_path.join(options.web_server.to_str()),
        &options.path,
        &options,
    )?;

    if options.init_git {
        println!("Initalizing git...");
        init_git(&git_path, &options.path)?;
    }

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

const IGNORED_FILES: [&str; 4] = [".git", "target", "build", ".vercel"];
fn walkdir_copy(path_from: &PathBuf, path_to: &PathBuf, options: &ProjectOptions) -> Result<()> {
    for entry in path_from.read_dir()? {
        let entry = entry?;
        let mut file_name = entry.file_name().into_string().unwrap();

        if file_name.starts_with("[") && file_name.contains("]") {
            let name_splitted = file_name.clone();
            let name_splitted: Vec<&str> = name_splitted.splitn(2, "]").collect();

            file_name = name_splitted[1].to_string();

            let statement: Vec<&str> = name_splitted[0].splitn(2, " ").collect();
            if !check_statement(&statement[0][1..], statement[1], options) {
                continue;
            }
        }

        let _path_to = path_to.join(&file_name);
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            if IGNORED_FILES.contains(&file_name.as_str()) {
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
            // OPTIONAL:           3 - &&,||    4,5 - second if
            let line_trimmed = line_trimmed.replace("]]", "");
            let splitted_args: Vec<&str> = line_trimmed.split(" ").collect();

            // If the last IF statement was false, then this one is also false
            ifs.push(if_statement(splitted_args, options) && ifs.last() != Some(&false));

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

fn if_statement(args: Vec<&str>, options: &ProjectOptions) -> bool {
    if args.len() == 3 {
        return check_statement(args[1], args[2], options);
    } else if args.len() == 6 {
        let statement_1 = check_statement(args[1], args[2], options);
        let statement_2 = check_statement(args[4], args[4], options);

        return match args[3] {
            "&&" => statement_1 && statement_2,
            "||" => statement_1 || statement_1,
            _ => panic!("Invalid IF statement!"),
        };
    }
    panic!("Invalid IF statement!");
}

fn check_statement(key: &str, value: &str, options: &ProjectOptions) -> bool {
    if key.starts_with("VERCEL_") {
        return options
            .vercel_settings
            .check_statement(key.trim_start_matches("VERCEL_"), value);
    }

    match key {
        "DATABASE" => Database::from_str(value) == options.database,
        "WEBSOCKET" => WebsocketServer::from_str(value) == options.websocket_server,
        "WEBSERVER" => {
            WebServer::from_str(value).expect("Wrong webserver name") == options.web_server
        }
        "DOCKER" => value == "true" && options.docker,
        _ => false,
    }
}
