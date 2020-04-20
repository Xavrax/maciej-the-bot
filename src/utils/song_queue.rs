use crate::utils::model::song_data::SongData;
use lazy_static::*;
use std::sync::{MutexGuard, Mutex};

lazy_static! {
    static ref SONG_QUEUE : Mutex<Vec<SongData>> = Mutex::new(Vec::new());
}

pub fn song_queue() -> MutexGuard<'static, Vec<SongData>> {
    SONG_QUEUE.lock().unwrap()
}