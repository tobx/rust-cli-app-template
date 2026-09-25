pub mod commands;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{config::Config, error::Result};

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Config directory
    #[arg(long, value_name = "DIR")]
    pub config_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Create(commands::create::Args),
    Info(commands::info::Args),
}

pub fn route(config: &Config, command: Command) -> Result<()> {
    use Command::{Create, Info};

    match command {
        Create(options) => {
            commands::create::run(config, &options)?;
        }
        Info(_) => {
            commands::info::run(config)?;
        }
    }
    Ok(())
}
