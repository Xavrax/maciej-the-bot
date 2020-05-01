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