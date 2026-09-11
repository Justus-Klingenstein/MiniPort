use crate::config::Config;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    pub id: Uuid,
    pub name: String,
}

impl Identity {
    pub fn load_or_create(config: &Config) -> Result<Self> {
        let path = identity_path(config);

        if path.exists() {
            let data = fs::read(&path)?;
            let identity: Identity = bincode::deserialize(&data)?;
            return Ok(identity);
        }

        let name = hostname();

        let identity = Identity {
            id: Uuid::new_v4(),
            name,
        };

        let data = bincode::serialize(&identity)?;
        fs::write(path, data)?;

        Ok(identity)
    }
}

fn identity_path(config: &Config) -> PathBuf {
    config.data_dir.join("identity")
}

fn hostname() -> String {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "Unknown Computer".to_string())
}