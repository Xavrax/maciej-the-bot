use crate::utils::model::song_data::SongData;
use serenity::voice::Audio;
use std::sync::Arc;
use parking_lot::Mutex;

pub struct Song {
    pub audio : Arc<Mutex<Audio>>,
    pub added_by : String
}

impl Song {
    pub fn new(audio : Arc<Mutex<Audio>>, added_by : String) -> Song {
        Song {
            audio,
            added_by
        }
    }
}