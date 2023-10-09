use std::path::{Path, PathBuf};

use config::{ConfigError, File, FileFormat};
use serde::Deserialize;

pub const CONFIG_FILE_NAME: &str = "config.toml";

pub const DEFAULT_CONFIG_FILE_CONTENT: &str = include_str!("default.toml");

#[derive(Deserialize)]
pub struct Config {
    pub data_dir: PathBuf,

    #[serde(skip)]
    pub dir: PathBuf,
}

impl Config {
    pub fn load(path: &Path) -> std::result::Result<Self, ConfigError> {
        let config = config::Config::builder()
            .add_source(File::from_str(
                include_str!("default.toml"),
                FileFormat::Toml,
            ))
            .add_source(File::from(path).format(FileFormat::Toml))
            .build()?;
        let mut config: Config = config.try_deserialize()?;
        config.dir = path.parent().unwrap().into();
        Ok(config)
    }
}
