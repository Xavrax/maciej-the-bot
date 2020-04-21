#[macro_use]
// Modules
mod discord;
mod utils;
//

use std::{env, sync::Arc, thread, time::Duration};
use crate::discord::model::{
    managers::VoiceManager,
    events::Handler
};
use serenity::{prelude::*, framework::{
    StandardFramework,
    standard::macros::group
}, voice};
use discord::commands::{
    audio::{DEAFEN_COMMAND, UNDEAFEN_COMMAND, MUTE_COMMAND, UNMUTE_COMMAND},
    channeling::{JOIN_COMMAND, LEAVE_COMMAND},
    player::{PLAY_COMMAND},
    utils::{PING_COMMAND, PREFIX_COMMAND}
};
use crate::utils::song_queue::{is_working, song_queue};
use serenity::voice::{LockedAudio, Audio};
use crate::discord::model::errors::check_msg;

#[group]
#[commands(deafen, join, leave, mute, play, ping, undeafen, unmute, prefix)]
struct General;

fn main() {
    let token_key = "MACIEJ_TOKEN";
    let prefix_key = "MACIEJ_PREFIX";

    let token = env::var(token_key)
        .expect("Expected a token in the environment");
    let prefix = env::var(prefix_key)
        .expect("Expected a prefix in the environment");
    let mut client = Client::new(&token, Handler)
        .expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .prefix(&prefix))
        .group(&GENERAL_GROUP));

    let player_thread = thread::spawn(move || {
        player_job()
    });

    let _ = client.start().map_err(|why| println!("Client ended: {:?}", why));
    player_thread.join();
}

fn player_job() {
    // dummy song
    let mut current_song = Arc::new(
        Mutex::new(
            Audio::new(
                voice::ytdl("https://www.youtube.com/watch?v=0k60dfpdRy0").unwrap()
            )
        )
    );
    (*current_song).lock().finished = true;

    while *is_working() {
        if song_queue().is_empty() {
            thread::sleep(Duration::from_secs(1));
            continue;
        }
        if !current_song.lock().finished {
            thread::sleep(Duration::from_secs(1));
            continue;
        }

        let new_song = match song_queue().pop_front() {
            Some(s) => s,
            None => {
                panic!();
            },
        };

        let guild_id = match new_song.ctx.cache.read().guild_channel(new_song.msg.channel_id) {
            Some(channel) => channel.read().guild_id,
            None => {
                check_msg(new_song.msg.channel_id.say(&new_song.ctx.http, "Error finding channel info"));
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

                    check_msg(new_song.msg.channel_id.say(&new_song.ctx.http, "Error sourcing ffmpeg"));
                    return;
                },
            };

            current_song = handler.play_returning(source);

            check_msg(new_song.msg.channel_id.say(&new_song.ctx.http, "Playing song"));
        } else {
            check_msg(new_song.msg.channel_id.say(&new_song.ctx.http, "Not in a voice channel to play in"));
        }
    }
}