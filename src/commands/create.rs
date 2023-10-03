use clap::Parser;

use crate::{
    config::Config,
    error::Result,
    terminal::{color::Colorize, message::write},
};

/// Create something
#[derive(Default, Parser)]
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
