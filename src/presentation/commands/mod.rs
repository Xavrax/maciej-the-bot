pub mod help;
pub mod music;

use crate::domain;
use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum CommandError {
//     #[error("Unsupported music source provided: {0}")]
//     UnsupportedMusicSource(domain::MusicInput),
// }
