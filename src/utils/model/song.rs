use serenity::voice::Audio;
use std::sync::Arc;
use parking_lot::Mutex;

pub struct Song {
    pub audio : Arc<Mutex<Audio>>,
    pub added_by : String,
    pub info : String
}

impl Song {
    pub fn new(audio : Arc<Mutex<Audio>>, added_by : String, info : String) -> Song {
        Song {
            audio,
            added_by,
            info
        }
    }
}