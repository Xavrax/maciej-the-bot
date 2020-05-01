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
use crate::utils::global::song_queue;
use crate::utils::model::song_data::SongData;

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

    // current_song().audio

    Ok(())
}