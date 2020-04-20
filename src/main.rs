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
    // Configure the client with your Discord bot token in the environment.
    let token = "tmp";
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Obtain a lock to the data owned by the client, and insert the client's
    // voice manager into it. This allows the voice manager to be accessible by
    // event handlers and framework commands.
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
