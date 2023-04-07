use anyhow::Result;
use std::path::PathBuf;
use tokio::process::Command;

pub async fn dev() -> Result<()> {
    let config_path = PathBuf::from("fnstack.json");
    let config = crate::config::FnstackConfig::from_file(config_path)?;

    let mut cmd = Command::new("cargo").arg("run").spawn().unwrap();
    loop {
        // check if files have changed
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        println!("Restart");

        cmd.kill().await.unwrap();
        cmd = Command::new("cargo").arg("run").spawn().unwrap();
    }

    Ok(())
}
