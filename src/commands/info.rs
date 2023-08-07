use clap::Parser;

use crate::{
    config::Config,
    error::Result,
    terminal::{color::Colorize, writeln},
    AppInfo,
};

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
