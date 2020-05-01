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
}};
use discord::commands::{
    audio::{DEAFEN_COMMAND, UNDEAFEN_COMMAND, MUTE_COMMAND, UNMUTE_COMMAND},
    channeling::{JOIN_COMMAND, LEAVE_COMMAND},
    player::{PLAY_COMMAND, SKIP_COMMAND},
    utils::{PING_COMMAND, PREFIX_COMMAND}
};
use crate::utils::global::{is_working, song_queue, lazy_init, current_song};
use crate::utils::music::play_next;

#[group]
#[commands(deafen, join, leave, mute, play, ping, undeafen, unmute, prefix, skip)]
struct General;

fn main() {
    let token_key = "MACIEJ_TOKEN";
    let prefix_key = "MACIEJ_PREFIX";

    lazy_init();

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
    while *is_working() {
        if song_queue().is_empty() {
            thread::sleep(Duration::from_secs(1));
            continue;
        }
        if !current_song().audio.lock().finished {
            thread::sleep(Duration::from_secs(1));
            continue;
        }

        play_next();
    }
}