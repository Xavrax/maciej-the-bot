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
    let impl_song = match args.single::<String>() {
        Ok(impl_song) => impl_song,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio"));

            return Ok(());
        },
    };

    // let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
    //     Some(channel) => channel.read().guild_id,
    //     None => {
    //         check_msg(msg.channel_id.say(&ctx.http, "Error finding channel info"));
    //
    //         return Ok(());
    //     },
    // };
    //
    // let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    // let mut manager = manager_lock.lock();

    // if let Some(handler) = manager.get_mut(guild_id) {
    //     let audio = if impl_song.starts_with("http") {
    //         voice::ytdl(&impl_song)
    //     } else {
    //         voice::ytdl_search(&impl_song)
    //     };
    //
    //     let source = match audio {
    //         Ok(source) => source,
    //         Err(why) => {
    //             println!("Err starting source: {:?}", why);
    //
    //             check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg"));
    //
    //             return Ok(());
    //         },
    //     };

        // handler.play(source);
        song_queue().push_back(
            SongData::new(impl_song, ctx.clone(), msg.clone())
        );

    //     check_msg(msg.channel_id.say(&ctx.http, "Playing song"));
    // } else {
    //     check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in"));
    // }

    Ok(())
}
