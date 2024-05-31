use std::{
    fs,
    path::{Path, PathBuf},
};

use config::{File, FileFormat};
use serde::Deserialize;

use crate::error::Result;

const CONFIG_FILE_NAME: &str = "config.toml";

const DEFAULT_CONFIG_FILE_CONTENT: &str = include_str!("default.toml");

#[derive(Deserialize)]
pub struct Config {
    pub data_dir: PathBuf,

    #[serde(skip)]
    pub dir: PathBuf,
}

impl Config {
    pub fn load(dir: &Path) -> Result<Config> {
        let file = dir.join(CONFIG_FILE_NAME);
        if !file.exists() {
            fs::create_dir_all(dir)?;
            fs::write(&file, DEFAULT_CONFIG_FILE_CONTENT)?;
        }
        let config = config::Config::builder()
            .add_source(File::from_str(
                include_str!("default.toml"),
                FileFormat::Toml,
            ))
            .add_source(File::from(file).format(FileFormat::Toml))
            .build()?;
        let mut config: Config = config.try_deserialize()?;
        config.dir = dir.into();
        Ok(config)
    }
}
