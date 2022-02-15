use crate::domain::music::music_library::{MusicInput, MusicLibrary};
use crate::domain::music::music_player::{ActionInfo, MusicPlayer};
use crate::Error;
use async_trait::async_trait;
use songbird::input::Input;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct ChannelingMusicService<L>
where
    L: MusicLibrary,
{
    queue: VecDeque<MusicInput>,
    library: L,
}

impl<L> ChannelingMusicService<L>
where
    L: MusicLibrary,
{
    pub fn new(library: L) -> Self {
        Self {
            queue: VecDeque::new(),
            library,
        }
    }
}

#[async_trait]
impl<L> MusicPlayer for ChannelingMusicService<L>
where
    L: MusicLibrary + Send + Sync,
{
    async fn play_song(&mut self, to_play: &str) -> Result<ActionInfo, crate::Error> {
        let music_input = if to_play.contains("https://") {
            self.library.get_song_by_url(to_play).await?
        } else {
            self.library.get_song_by_name(to_play).await?
        };

        self.queue.push_back(music_input);

        Ok(ActionInfo(format!("Added to queue, mate!")))
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use crate::domain::music::music_library::DummyMusicLibrary;

    #[tokio::test]
    async fn play_song_with_provided_name() {
        let song_name = "some song name";

        let library_mock = DummyMusicLibrary;

        let mut sut = ChannelingMusicService::new(library_mock);

        let song = sut.play_song(song_name).await;

        assert!(song.is_ok())
    }

    #[tokio::test]
    async fn play_song_by_url() {
        let song_name = "https://some_link/";

        let library_mock = DummyMusicLibrary;

        let mut sut = ChannelingMusicService::new(library_mock);

        let song = sut.play_song(song_name).await;

        assert!(song.is_ok())
    }
}
