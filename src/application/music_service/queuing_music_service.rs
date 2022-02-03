use crate::application::music_service::{ActionInfo, MusicService};
use crate::domain::music_library::{MusicInput, MusicLibrary};
use crate::Error;
use async_trait::async_trait;
use songbird::input::Input;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct QueuingMusicService<L>
where
    L: MusicLibrary,
{
    queue: Arc<Mutex<VecDeque<MusicInput>>>,
    library: L,
}

impl<L> QueuingMusicService<L>
where
    L: MusicLibrary,
{
    pub fn new(library: L) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            library,
        }
    }
}

#[async_trait]
impl<L> MusicService for QueuingMusicService<L>
where
    L: MusicLibrary,
{
    async fn play_song(&self, to_play: &str) -> Result<ActionInfo, crate::Error> {
        let music_input = if to_play.contains("https://") {
            self.library.get_song_by_url(to_play).await?
        } else {
            self.library.get_song_by_name(to_play).await?
        };

        let mut queue_lock = self.queue.clone().lock()?;
        queue_lock.push_back(music_input);

        Ok(ActionInfo(format!("Added to queue, mate!")))
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use crate::domain::music_library::{DummyMusicLibrary, MusicInput};

    #[tokio::test]
    async fn play_song_with_provided_name() {
        let song_name = "some song name";

        let library_mock = DummyMusicLibrary;

        let sut = QueuingMusicService::new(library_mock);

        let song = sut.play_song(song_name).await;

        assert!(song.is_ok())
    }

    #[tokio::test]
    async fn play_song_by_url() {
        let song_name = "https://some_link/";

        let library_mock = DummyMusicLibrary;

        let sut = QueuingMusicService::new(library_mock);

        let song = sut.play_song(song_name).await;

        assert!(song.is_ok())
    }
}
