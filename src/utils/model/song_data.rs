use serenity::voice::AudioSource;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub struct SongData {
    pub info : String,
    pub ctx : Context,
    pub msg : Message,
}

impl SongData {
    pub fn new(info : String, ctx : Context, msg : Message) -> SongData {
        SongData{
            info,
            ctx,
            msg
        }
    }
}