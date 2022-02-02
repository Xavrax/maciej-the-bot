use async_trait::async_trait;
use songbird::input as songbird;
use thiserror::Error;

pub mod youtube_player;

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("Downloading song failed: {0}")]
    CannotDownload(String),
}

#[async_trait]
pub trait MusicPlayer {
    async fn get_song_by_name(&self, song_name: &str) -> Result<songbird::Input, PlayerError>;
    async fn get_song_by_url(&self, url: &str) -> Result<songbird::Input, PlayerError>;
}
