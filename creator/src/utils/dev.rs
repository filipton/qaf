use anyhow::Result;
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::process::Command;

pub async fn dev() -> Result<()> {
    let path = PathBuf::from("./");
    //let config_path = PathBuf::from("fnstack.json");
    //let config = crate::config::FnstackConfig::from_file(config_path)?;

    let stop: bool = false;
    let mut last_modify = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let mut cmd = Command::new("cargo").arg("run").spawn().unwrap();
    while !stop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let last = walkdir_modify_date(&path)?;
        if last > last_modify {
            last_modify = last;

            cmd.kill().await.unwrap();
            cmd = Command::new("cargo").arg("run").spawn().unwrap();
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
