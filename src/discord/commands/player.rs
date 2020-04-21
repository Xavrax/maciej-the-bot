use crate::discord::model::{
    managers::VoiceManager,
    errors::check_msg
};
use serenity::{
    prelude::Context,
    model::prelude::Message,
    voice,
    framework::standard::{
        Args,
        CommandResult,
        macros::command
    }
};
use crate::utils::song_queue::song_queue;
use crate::utils::model::song_data::SongData;

#[command]
pub fn play(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let impl_song = args.message();
    // let impl_song = match args.single::<String>() {
    //     Ok(impl_song) => impl_song,
    //     Err(_) => {
    //         check_msg(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio"));
    //
    //         return Ok(());
    //     },
    // };

    song_queue().push_back(
        SongData::new(impl_song.to_string(), ctx.clone(), msg.clone())
    );

    check_msg(msg.channel_id.say(&ctx.http, format!("Song {} added to queue!", impl_song.to_string())));

    Ok(())
}
