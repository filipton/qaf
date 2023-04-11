use std::{os::unix::process::CommandExt, process::Command};

use anyhow::{anyhow, Result};
use which::which;

pub async fn build() -> Result<()> {
    _ = which("docker").map_err(|_| anyhow!("Docker is not installed!"))?;

    let dir = std::env::current_dir()?;
    let dockerfile = dir.join("Dockerfile");
    let image_name = dir.file_name().unwrap().to_str().unwrap();

    _ = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(image_name)
        .arg("-f")
        .arg(dockerfile)
        .arg(dir)
        .exec();

    Ok(())
}

pub async fn build_cached() -> Result<()> {
    _ = which("docker").map_err(|_| anyhow!("Docker is not installed!"))?;

    let dir = std::env::current_dir()?;
    let dockerfile = dir.join("cache.Dockerfile");
    let image_name = dir.file_name().unwrap().to_str().unwrap();

    _ = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .spawn()?
        .wait()?;

    _ = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(image_name)
        .arg("-f")
        .arg(dockerfile)
        .arg(dir)
        .exec();

    Ok(())
}

pub async fn run() -> Result<()> {
    _ = which("docker").map_err(|_| anyhow!("Docker is not installed!"))?;

    let dir = std::env::current_dir()?;
    let env_file = dir.join(".env");
    let image_name = dir.file_name().unwrap().to_str().unwrap();

    println!("Running image: {}...", image_name);
    println!("Press Ctrl+C to stop the server.\n\n");

    let mut cmd_run = Command::new("docker");
    cmd_run.arg("run");

    // TODO: MESS
    if env_file.exists() {
        let env_file = std::fs::read_to_string(&env_file)?;
        let port = env_file
            .lines()
            .find(|e| e.starts_with("BIND_ADDRESS="))
            .map(|e| e.split(":").nth(1))
            .expect("map error")
            .expect("bind address empty?");

        cmd_run
            .arg("--env-file")
            .arg(".env")
            .arg("-p")
            .arg(format!("{}:{}", port, port));
    } else {
        cmd_run
            .arg("-p")
            .arg("8080:8080")
            .arg("-e")
            .arg("BIND_ADDRESS=0.0.0.0:8080");
    }

    cmd_run.arg("--rm").arg("-it").arg(image_name).exec();

    Ok(())
}
