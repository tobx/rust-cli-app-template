use std::io;

use crate::{
    config::Config,
    terminal::{color::Colorize, message::write},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
}

/// Create something
#[derive(clap::Args)]
pub struct Args {
    /// The title to use
    #[arg(long, short)]
    pub title: String,
}

pub fn run(config: &Config, options: &Args) -> Result<()> {
    write::info(format!(
        "creating something in data dir '{}'...",
        config.data_dir.to_string_lossy().yellow()
    ))?;
    write::success(format!(
        "created something with title '{}'",
        options.title.yellow()
    ))?;
    Ok(())
}
