use crate::utils::global::{song_queue, current_song};
use crate::discord::model::errors::check_msg;
use crate::discord::model::managers::VoiceManager;
use serenity::voice;
use crate::discord::model::messages::say;

pub fn play_next() {
    let new_song = match song_queue().pop_front() {
        Some(s) => s,
        None => {
            panic!();
        },
    };

    let guild_id = match new_song.ctx.cache.read().guild_channel(new_song.msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(new_song.msg.channel_id.say(&new_song.ctx.http, say("Error finding channel info")));
            return;
        },
    };

    let manager_lock = new_song.ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) {
        let audio = if new_song.info.starts_with("http") {
            voice::ytdl(&new_song.info)
        } else {
            voice::ytdl_search(&new_song.info)
        };

        let source = match audio {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(new_song.msg.channel_id.say(&new_song.ctx.http, say("Error sourcing ffmpeg")));
                return;
            },
        };

        current_song().added_by = new_song.msg.author.name.clone();
        current_song().info = new_song.info.clone();
        current_song().audio = handler.play_only(source);

        check_msg(new_song.msg.channel_id.say(&new_song.ctx.http, say(format!("Playing song {}", new_song.info))));
    } else {
        check_msg(new_song.msg.channel_id.say(&new_song.ctx.http, say("Not in a voice channel to play in")));
    }
}