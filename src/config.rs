use anyhow::Result;
use std::path::PathBuf;

pub struct Config {
    pub data_dir: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let data_dir = if cfg!(target_os = "windows") {
            std::env::var_os("APPDATA")
                .map(PathBuf::from)
                .ok_or_else(|| anyhow::anyhow!("APPDATA is not available"))?
                .join("MiniPort")
        } else if cfg!(target_os = "macos") {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .ok_or_else(|| anyhow::anyhow!("HOME is not available"))?
                .join("Library")
                .join("Application Support")
                .join("MiniPort")
        } else {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .ok_or_else(|| anyhow::anyhow!("HOME is not available"))?
                .join(".local")
                .join("share")
                .join("miniport")
        };

        std::fs::create_dir_all(&data_dir)?;

        Ok(Self { data_dir })
    }
}