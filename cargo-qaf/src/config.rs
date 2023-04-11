use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct QafConfig {
    pub watch_cmd: String,
    pub watch_dir: String,
}

impl Default for QafConfig {
    fn default() -> Self {
        Self {
            watch_cmd: "cargo run".to_string(),
            watch_dir: "./".to_string(),
        }
    }
}

impl QafConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_file(path: PathBuf) -> Result<Self> {
        let file = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&file)?;

        Ok(config)
    }

    pub fn to_file(data: Self, path: PathBuf) -> Result<()> {
        let config = serde_json::to_string_pretty(&data)?;
        std::fs::write(path, config)?;

        Ok(())
    }
}
