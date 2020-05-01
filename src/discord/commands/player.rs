use crate::discord::model::{
    errors::check_msg
};
use serenity::{
    prelude::Context,
    model::prelude::Message,
    framework::standard::{
        Args,
        CommandResult,
        macros::command
    }
};
use crate::utils::global::{song_queue, current_song};
use crate::utils::model::song_data::SongData;
use crate::utils::music::play_next;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use crate::discord::model::messages::say;

#[command]
pub fn play(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let impl_song = args.message();

    song_queue().push_back(
        SongData::new(impl_song.to_string(), ctx.clone(), msg.clone())
    );

    check_msg(msg.channel_id.say(&ctx.http, say(format!("Song {} added to queue!", impl_song.to_string()))));

    Ok(())
}

#[command]
pub fn skip(ctx: &mut Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, say("Skipping...")));

    if song_queue().is_empty() {
        current_song().audio.lock().pause();
        current_song().audio.lock().finished = true;
    } else {
        play_next();
    }

    Ok(())
}

#[command]
pub fn pause(ctx: &mut Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, say("Pausing...")));

    if !current_song().audio.lock().finished {
        current_song().audio.lock().pause();
    }

    Ok(())
}

#[command]
pub fn resume(ctx: &mut Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, say("Resuming...")));

    if !current_song().audio.lock().finished {
        current_song().audio.lock().play();
    }

    Ok(())
}

#[command]
pub fn now(ctx: &mut Context, msg: &Message) -> CommandResult {
    let info = current_song().info.clone();
    let added_by = current_song().added_by.clone();

    if !current_song().audio.lock().finished {
        check_msg(msg.channel_id.say(&ctx.http, say(format!("Now playing: {} *(added by {})*", info, added_by))));
    } else {
        check_msg(msg.channel_id.say(&ctx.http, say("Nothing is playing...")));
    }

    Ok(())
}

#[command]
pub fn queue(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut answer = String::from("Queue:\n");

    song_queue()
        .iter()
        .for_each(|song| {
            answer = say(format!("{}{} *(added by {})*\n", answer, song.info.clone(), song.msg.author.name.clone()));
        });

    check_msg(msg.channel_id.say(&ctx.http, say(answer)));
    Ok(())
}

#[command]
pub fn shuffle(ctx: &mut Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, say("Shuffling...")));

    let mut tmp = Vec::new();
    while !song_queue().is_empty() {
        tmp.push(song_queue().pop_front().unwrap());
    }
    tmp.shuffle(&mut thread_rng());
    while !tmp.is_empty() {
        song_queue().push_back(tmp.pop().unwrap());
    }

    Ok(())
}