// Modules
mod discord;
mod utils;
//

use std::{env, sync::Arc};
use crate::discord::model::{
    managers::VoiceManager,
    events::Handler
};
use serenity::{
    prelude::*,
    framework::{
        StandardFramework,
        standard::macros::group
    }
};
use discord::commands::{
    audio::{DEAFEN_COMMAND, UNDEAFEN_COMMAND, MUTE_COMMAND, UNMUTE_COMMAND},
    channeling::{JOIN_COMMAND, LEAVE_COMMAND},
    player::{PLAY_COMMAND},
    utils::{PING_COMMAND}
};

#[group]
#[commands(deafen, join, leave, mute, play, ping, undeafen, unmute)]
struct General;

fn main() {
    let token = "tmp";
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .prefix("~"))
        .group(&GENERAL_GROUP));

    let _ = client.start().map_err(|why| println!("Client ended: {:?}", why));
}
