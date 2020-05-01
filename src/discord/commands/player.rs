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

#[command]
pub fn play(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let impl_song = args.message();

    song_queue().push_back(
        SongData::new(impl_song.to_string(), ctx.clone(), msg.clone())
    );

    check_msg(msg.channel_id.say(&ctx.http, format!("Song {} added to queue!", impl_song.to_string())));

    Ok(())
}

#[command]
pub fn skip(ctx: &mut Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, "Skipping..."));

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
    check_msg(msg.channel_id.say(&ctx.http, "Pausing..."));

    if !current_song().audio.lock().finished {
        current_song().audio.lock().pause();
    }

    Ok(())
}

#[command]
pub fn resume(ctx: &mut Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, "Resuming..."));

    if !current_song().audio.lock().finished {
        current_song().audio.lock().play();
    }

    Ok(())
}

#[command]
pub fn now(ctx: &mut Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, format!("Now playing: {} added by {}", current_song().info, current_song().added_by)));

    Ok(())
}

#[command]
pub fn queue(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut answer = String::from("Queue:\n");

    song_queue()
        .iter()
        .for_each(|song| {
            answer = format!("{}{}\n", answer, song.info)
        });

    check_msg(msg.channel_id.say(&ctx.http, answer));
    Ok(())
}

#[command]
pub fn shuffle(ctx: &mut Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, "Shuffling..."));

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