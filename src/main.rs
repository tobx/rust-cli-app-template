#![forbid(unsafe_code)]

mod args;
mod commands;
mod config;
mod error;
mod terminal;

use std::fs;

use clap::Parser;
use directories::ProjectDirs;

use crate::{
    args::Args,
    config::{Config, CONFIG_FILE_NAME, DEFAULT_CONFIG_FILE_CONTENT},
    error::Result,
    terminal::message::write,
};

// Those constants will be used for:
// https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.from
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_ORGANIZATION: &str = "";
pub const APP_QUALIFIER: &str = "";

struct AppInfo {
    name: String,
    homepage: &'static str,
    version: &'static str,
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            name: APP_NAME.into(),
            homepage: env!("CARGO_PKG_HOMEPAGE"),
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}

fn run() -> Result<()> {
    let mut options = Args::parse();
    let config_dir = options.config_dir.get_or_insert_with(|| {
        ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME)
            .expect("cannot retrieve valid home directory from the operating system")
            .config_dir()
            .to_path_buf()
    });
    let config_file = config_dir.join(CONFIG_FILE_NAME);
    if !config_file.exists() {
        fs::create_dir_all(config_dir)?;
        fs::write(&config_file, DEFAULT_CONFIG_FILE_CONTENT)?;
    }
    let config = Config::load(&config_file)?;
    args::route(&config, options)
}

fn main() {
    if let Err(error) = run() {
        write::error(error).expect("cannot write error to stderr");
        std::process::exit(1);
    }
}
