use thiserror::Error;

pub mod domain;
pub mod infrastructure;
pub mod presentation;

pub use presentation::MINECRAFT_GROUP;

#[derive(Error, Debug)]
pub enum MinecraftError {
    #[error("FTP error: {0}")]
    FtpError(String),
}
