use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::options::{ProjectOptions, WebServer};

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

    pub fn generate(options: &ProjectOptions) -> Result<Self> {
        let mut config = Self::new();
        if options.web_server == WebServer::Cloudflare {
            config.watch_cmd = "kill $(ps -eo pid,cmd | grep wrangler | grep -v grep | awk '{print $1}') ; sleep 2 ; wrangler dev --local".into();
        } else if options.web_server == WebServer::Vercel {
            config.watch_cmd = "vercel dev".into();
        }

        Ok(config)
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
