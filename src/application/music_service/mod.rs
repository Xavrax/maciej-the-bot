use async_trait::async_trait;

pub mod queuing_music_service;

pub struct ActionInfo(String);

#[async_trait]
pub trait MusicService {
    async fn play_song(&self, to_play: &str) -> Result<ActionInfo, crate::Error>;
}
