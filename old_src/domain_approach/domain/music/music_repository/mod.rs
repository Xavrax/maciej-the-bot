use crate::domain::MusicInput;
use async_trait::async_trait;

pub mod library_music_repository;

#[async_trait]
pub trait MusicRepository {
    async fn get(&mut self, to_play: &str) -> Result<MusicInput, crate::Error>;
}
