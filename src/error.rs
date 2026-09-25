use crate::{
    args::{create, info},
    config,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("config error: {0}")]
    Config(#[from] config::Error),
    #[error("info error: {0}")]
    Info(#[from] info::Error),
    #[error("playlists error: {0}")]
    Create(#[from] create::Error),
    #[error("system error: {0}")]
    System(String),
}
