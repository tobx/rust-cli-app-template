use std::{
    fs, io,
    path::{Path, PathBuf},
};

use config::{ConfigError, File, FileFormat};
use serde::Deserialize;

pub type Result<T> = std::result::Result<T, Error>;

const CONFIG_FILE_NAME: &str = "config.toml";

const DEFAULT_CONFIG_FILE_CONTENT: &str = include_str!("default.toml");

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("config error: {0}")]
    Config(#[from] ConfigError),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
}

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
                DEFAULT_CONFIG_FILE_CONTENT,
                FileFormat::Toml,
            ))
            .add_source(File::from(file).format(FileFormat::Toml))
            .build()?;
        let mut config: Config = config.try_deserialize()?;
        config.dir = dir.into();
        Ok(config)
    }
}
