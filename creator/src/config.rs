use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct FnstackConfig {
    pub disable_scopes: bool,
    pub tmux_single_window: bool,
}

impl Default for FnstackConfig {
    fn default() -> Self {
        Self {
            disable_scopes: false,
            tmux_single_window: true,
        }
    }
}

impl FnstackConfig {
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
