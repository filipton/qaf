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

pub async fn run() -> Result<()> {
    _ = which("docker").map_err(|_| anyhow!("Docker is not installed!"))?;

    let dir = std::env::current_dir()?;
    let image_name = dir.file_name().unwrap().to_str().unwrap();

    println!("Running image: {}...", image_name);
    println!("Press Ctrl+C to stop the server.\n\n");

    _ = Command::new("docker")
        .arg("run")
        .arg("-p")
        .arg("8080:8080")
        .arg("-e")
        .arg("BIND_ADDRESS=0.0.0.0:8080")
        .arg("--rm")
        .arg("-it")
        .arg(image_name)
        .exec();

    Ok(())
}
