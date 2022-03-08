use crate::domain::music::music_library::MusicLibrary;
use crate::domain::music::music_repository::MusicRepository;
use crate::domain::MusicInput;
use crate::Error;
use async_trait::async_trait;
use songbird::input::Input;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct LibraryMusicRepository<L>
where
    L: MusicLibrary,
{
    library: L,
}

impl<L> LibraryMusicRepository<L>
where
    L: MusicLibrary,
{
    pub fn new(library: L) -> Self {
        Self { library }
    }
}

#[async_trait]
impl<L> MusicRepository for LibraryMusicRepository<L>
where
    L: MusicLibrary + Send + Sync,
{
    async fn get(&mut self, to_play: &str) -> Result<MusicInput, crate::Error> {
        Ok(if to_play.contains("https://") {
            self.library.get_song_by_url(to_play).await?
        } else {
            self.library.get_song_by_name(to_play).await?
        })
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use crate::domain::music::music_library::DummyMusicLibrary;
    use crate::domain::MusicInput;

    #[tokio::test]
    async fn play_song_with_provided_name() {
        let song_name = "some song name";

        let library_mock = DummyMusicLibrary;

        let mut sut = LibraryMusicRepository::new(library_mock);

        // let song = sut.get(song_name).await;

        // assert!(song.is_ok());
        // assert_eq!(song.unwrap(), MusicInput::Dummy);
    }

    #[tokio::test]
    async fn play_song_by_url() {
        let song_name = "https://some_link/";

        let library_mock = DummyMusicLibrary;

        let mut sut = LibraryMusicRepository::new(library_mock);

        // let song = sut.play_song(song_name).await;

        // assert!(song.is_ok());
        // assert_eq!(song.unwrap(), MusicInput::Dummy);
    }
}
