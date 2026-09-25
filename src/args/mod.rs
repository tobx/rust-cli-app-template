pub mod create;
pub mod info;

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
    Create(create::Args),
    Info(info::Args),
}

pub fn route(config: &Config, command: Command) -> Result<()> {
    use Command::{Create, Info};

    match command {
        Create(options) => {
            create::run(config, &options)?;
        }
        Info(_) => {
            info::run(config)?;
        }
    }
    Ok(())
}
