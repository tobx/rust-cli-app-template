use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{
    commands::{self, create, info},
    config::Config,
    error::Result,
};

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

pub fn route(config: &Config, options: Args) -> Result<()> {
    use Command::{Create, Info};

    match options.command {
        Create(options) => commands::create::run(config, &options),
        Info(_) => commands::info::run(config),
    }
}
