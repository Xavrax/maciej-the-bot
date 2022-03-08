use crate::domain::MusicInput;
use async_trait::async_trait;
use songbird::input as songbird;
use thiserror::Error;

pub mod youtube_library;

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("Downloading song failed: {0}")]
    CannotDownload(String),
}

#[async_trait]
pub trait MusicLibrary {
    async fn get_song_by_name(&self, song_name: &str) -> Result<MusicInput, PlayerError>;
    async fn get_song_by_url(&self, url: &str) -> Result<MusicInput, PlayerError>;
}

#[cfg(test)]
pub struct DummyMusicLibrary;

#[cfg(test)]
#[async_trait]
impl MusicLibrary for DummyMusicLibrary {
    async fn get_song_by_name(&self, _song_name: &str) -> Result<MusicInput, PlayerError> {
        Ok(MusicInput::Dummy)
    }

    async fn get_song_by_url(&self, _url: &str) -> Result<MusicInput, PlayerError> {
        Ok(MusicInput::Dummy)
    }
}
