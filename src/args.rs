use std::path::PathBuf;

use clap::Parser;

use crate::commands::{create, info};

#[derive(Parser)]
#[clap(version, about)]
pub struct Args {
    /// Config directory
    #[clap(long, value_name = "DIR")]
    pub config_dir: Option<PathBuf>,

    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Parser)]
pub enum SubCommand {
    Create(create::Args),
    Info(info::Args),
}
