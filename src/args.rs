use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::commands::{create, info};

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
