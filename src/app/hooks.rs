use rand::prelude::IteratorRandom;
use serenity::{
    all::Message,
    framework::standard::{macros::hook, CommandResult, Reason},
    futures::TryFutureExt,
    prelude::Context,
};

#[hook]
pub async fn before(_ctx: &Context, msg: &Message, cmd_name: &str) -> bool {
    log::info!(
        "Processing command '{}' for user '{}'",
        cmd_name,
        msg.author.name
    );

    true
}

#[hook]
pub async fn after(ctx: &Context, msg: &Message, cmd_name: &str, result: CommandResult) {
    result
        .map(|_| {
            log::info!(
                "Processed command '{}' for user '{}'",
                cmd_name,
                msg.author.name
            )
        })
        .map_err(|e| async move {
            log::error!(
                "Command '{}' from '{}' returned error {:?}",
                cmd_name,
                msg.author.name,
                e
            );
            msg.channel_id
                .say(&ctx.http, format!("Error: {:?}", e))
                .await
                .map(|_| ())
                .unwrap_or_else(|e| log::error!("Error sending message: {:?}", e))
        });
}

#[hook]
pub async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    log::warn!(
        "Unknown command '{}' from user '{}'",
        unknown_command_name,
        msg.author.name
    );

    let answer = include_str!("../../assets/unknown_command.txt")
        .split("\n")
        .choose(&mut rand::thread_rng())
        .unwrap_or("Sorry, I don't know that one.");

    msg.channel_id
        .say(&ctx.http, answer)
        .await
        .map(|_| ())
        .unwrap_or_else(|e| log::error!("Error sending message: {:?}", e));
}
