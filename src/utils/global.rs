use crate::utils::model::song_data::SongData;
use crate::utils::model::song::Song;
use lazy_static::*;
use std::sync::{MutexGuard, Mutex, Arc};
use std::collections::VecDeque;
use serenity::voice;
use serenity::voice::Audio;

lazy_static! {
    static ref SONG_QUEUE : Mutex<VecDeque<SongData>> = Mutex::new(VecDeque::new());
}

lazy_static! {
    static ref WORKING : Mutex<bool> = Mutex::new(true);
}

lazy_static! {
    static ref CURRENT_SONG : Mutex<Song> = Mutex::new(
        Song::new(
            Arc::new(
                parking_lot::Mutex::new(
                    Audio::new(
                        voice::ytdl("https://www.youtube.com/watch?v=0k60dfpdRy0").unwrap()
                    )
                )
            ),
            String::from("None")
        )
    );
}

pub fn lazy_init() {
    current_song().audio.lock().finished = true;
}

pub fn is_working() -> MutexGuard<'static, bool> {
    WORKING.lock().unwrap()
}

pub fn song_queue() -> MutexGuard<'static, VecDeque<SongData>> {
    SONG_QUEUE.lock().unwrap()
}

pub fn current_song() -> MutexGuard<'static, Song> {
    CURRENT_SONG.lock().unwrap()
}
