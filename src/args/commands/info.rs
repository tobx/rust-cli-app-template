use std::io;

use clap::Parser;

use crate::{
    AppInfo,
    config::Config,
    terminal::{color::Colorize, writeln},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
}

/// Show application info
#[derive(Default, Parser)]
pub struct Args;

pub fn run(config: &Config) -> Result<()> {
    let info = AppInfo::default();
    let mappings = [
        ("Name", info.name.as_str()),
        ("Version", info.version),
        ("Homepage", info.homepage),
        ("Config dir", &config.dir.to_string_lossy()),
    ];
    let width = mappings
        .iter()
        .fold(0, |acc, (name, _)| name.chars().count().max(acc));
    for (name, value) in mappings {
        writeln(format!(
            "{:>width$} {} {}",
            name,
            "·".green(),
            value,
            width = width
        ))?;
    }
    Ok(())
}
