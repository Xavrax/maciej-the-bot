use serenity::voice::AudioSource;

pub struct SongData {
    pub info : String,
    pub added_by : String,
    pub audio : Box<dyn AudioSource>
}

impl SongData {
    pub fn new(info : String, added_by : String, audio : Box<dyn AudioSource>) -> SongData {
        SongData{
            info,
            added_by,
            audio
        }
    }
}