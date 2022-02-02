use crate::domain::music_player::{MusicPlayer, PlayerError};
use async_trait::async_trait;
use songbird::input::{ytdl_search, Input};
use songbird::ytdl;

#[derive(Default)]
pub struct YoutubePlayer;

#[async_trait]
impl MusicPlayer for YoutubePlayer {
    async fn get_song_by_name(&self, song_name: &str) -> Result<Input, PlayerError> {
        match ytdl_search(song_name).await {
            Ok(song) => Ok(song),
            Err(err) => Err(PlayerError::CannotDownload(err.to_string())),
        }
    }

    async fn get_song_by_url(&self, url: &str) -> Result<Input, PlayerError> {
        match ytdl(url).await {
            Ok(song) => Ok(song),
            Err(err) => Err(PlayerError::CannotDownload(err.to_string())),
        }
    }
}

#[cfg(test)]
mod should {
    use super::*;

    // Ignored: Tests below are ignored because they are need internet connection
    //          to work and they are not as quick as UT should be.

    #[ignore]
    #[tokio::test]
    async fn search_for_song_with_provided_name() {
        let song_name = "never gonna give you up";

        let sut = YoutubePlayer::default();

        let song = sut.get_song_by_name(song_name).await;

        assert!(song.is_ok())
    }

    #[ignore]
    #[tokio::test]
    async fn search_for_song_with_provided_url() {
        let song_name = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";

        let sut = YoutubePlayer::default();

        let song = sut.get_song_by_url(song_name).await;

        assert!(song.is_ok())
    }
}
