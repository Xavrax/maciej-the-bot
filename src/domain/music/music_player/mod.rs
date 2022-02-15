use async_trait::async_trait;

pub mod channeling_music_service;

pub struct ActionInfo(String);

#[async_trait]
pub trait MusicPlayer {
    async fn play_song(&mut self, to_play: &str) -> Result<ActionInfo, crate::Error>;
}
