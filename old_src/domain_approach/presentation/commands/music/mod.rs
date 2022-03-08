use crate::application::help_service::prefix_configurable_help_service::PrefixConfigurableHelpService;
use crate::application::help_service::{HelpLevel, HelpService};
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

pub mod user {
    use super::*;
    use crate::domain::music::music_library::youtube_library::YoutubeLibrary;
    use crate::domain::music::music_repository::library_music_repository::LibraryMusicRepository;
    use crate::domain::music::music_repository::MusicRepository;
    use crate::domain::MusicInput;
    // use crate::presentation::commands::CommandError;
    use serenity::framework::standard::Args;
    use std::convert::Infallible;

    #[command]
    async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
        // play_function(ctx, msg, args).await
        todo!()
    }

    #[command]
    async fn p(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
        // play_function(ctx, msg, args).await
        todo!()
    }

    //     async fn play_function(
    //         ctx: &Context,
    //         msg: &Message,
    //         mut args: Args,
    //     ) -> Result<(), crate::Error> {
    //         let song_info = match args.single::<String>() {
    //             Ok(song_info) => song_info,
    //             Err(_) => {
    //                 msg.channel_id
    //                     .say(
    //                         &ctx.http,
    //                         "Must provide song name or url to play music, mate!",
    //                     )
    //                     .await;
    //
    //                 return Ok(());
    //             }
    //         };
    //
    //         let guild = msg.guild(&ctx.cache).await.unwrap();
    //
    //         let song_manager = songbird::get(ctx).await.unwrap().clone();
    //
    //         if let Some(handler_lock) = song_manager.get(guild.id) {
    //             let mut handler = handler_lock.lock().await;
    //
    //             let music_input = LibraryMusicRepository::new(YoutubeLibrary::default())
    //                 .get(&song_info)
    //                 .await?;
    //
    //             let source = match music_input {
    //                 MusicInput::Youtube(source) => Ok(source),
    //                 other => Err(CommandError::UnsupportedMusicSource(other)),
    //             }?;
    //
    //             handler.enqueue_source(source.into());
    //
    //             msg.channel_id.say(
    //                 &ctx.http,
    //                 format!("Added song to queue: position {}", handler.queue().len()),
    //             )
    //         } else {
    //             msg.channel_id
    //                 .say(&ctx.http, "Not in a voice channel")
    //                 .await
    //         }
    //
    //         Ok(())
    //     }
}

pub mod operator {
    use super::*;
}
