use anyhow::Result;
use std::{io::Write, os::unix::process::CommandExt, path::PathBuf, process::Command};

pub async fn kill() -> Result<()> {
    let cargo_toml = PathBuf::from("Cargo.toml");
    let cargo_toml = std::fs::read_to_string(cargo_toml)?;

    let project_name = cargo_toml
        .lines()
        .find(|line| line.starts_with("name = "))
        .unwrap()
        .split(" = ")
        .nth(1)
        .unwrap()
        .replace("\"", "");

    _ = Command::new("tmux")
        .arg("-L")
        .arg("fnstack")
        .arg("kill-server")
        .output();

    _ = Command::new("bash")
        .arg("-c")
        .arg(format!("kill -9 $(pidof {})", project_name))
        .output();

    Ok(())
}

pub async fn dev() -> Result<()> {
    let config_path = PathBuf::from("fnstack.json");
    let config = crate::config::BuildrsConfig::from_file(config_path)?;

    kill().await?;
    _ = Command::new("tmux")
        .arg("-L")
        .arg("fnstack")
        .arg("-f")
        .arg("tmux.conf")
        .arg("new-session")
        .arg("-c")
        .arg("./")
        .arg("-d")
        .arg("cargo watch -x run")
        .output();

    if config.tmux_single_window {
        _ = Command::new("tmux")
            .arg("-L")
            .arg("fnstack")
            .arg("split-window")
            .arg("-h")
            .arg("-c")
            .arg("./")
            .arg("btop")
            .output();
    } else {
        _ = Command::new("tmux")
            .arg("-L")
            .arg("fnstack")
            .arg("new-window")
            .arg("-c")
            .arg("./")
            .arg("btop")
            .output();
    }

    print!("To detach from tmux use C^f + d. Click enter to continue... ");
    std::io::stdout().flush()?;

    let mut string = String::new();
    std::io::stdin().read_line(&mut string)?;
    drop(string);

    _ = Command::new("tmux")
        .arg("-L")
        .arg("fnstack")
        .arg("attach")
        .exec();

    Ok(())
}
