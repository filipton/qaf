use anyhow::Result;
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::process::Command;

use crate::config;

pub async fn dev() -> Result<()> {
    let config = config::QafConfig::from_file(PathBuf::from("./qaf.json"))?;
    let path = PathBuf::from(config.watch_dir);

    let stop: bool = false;
    let mut last_modify = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let mut cmd = Command::new("bash")
        .arg("-c")
        .arg(&config.watch_cmd)
        .spawn()
        .unwrap();

    while !stop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let last = walkdir_modify_date(&path)?;
        if last > last_modify {
            last_modify = last;

            cmd.kill().await.unwrap();
            cmd = Command::new("bash")
                .arg("-c")
                .arg(&config.watch_cmd)
                .spawn()
                .unwrap();
        }
    }

    Ok(())
}

const EXCLUDED_NAMES: [&str; 2] = ["target", "build"];
fn walkdir_modify_date(path: &PathBuf) -> Result<u128> {
    let mut latest_modify = 0u128;

    for entry in path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            if EXCLUDED_NAMES.contains(&entry.file_name().to_str().unwrap()) {
                continue;
            }

            if entry.file_type().unwrap().is_file() {
                let modify_time = entry
                    .metadata()?
                    .modified()?
                    .duration_since(UNIX_EPOCH)?
                    .as_millis();

                latest_modify = latest_modify.max(modify_time);
            } else {
                latest_modify = latest_modify.max(walkdir_modify_date(&entry.path())?);
            }
        }
    }

    return Ok(latest_modify);
}
