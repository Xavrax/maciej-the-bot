use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

pub mod command;
pub mod discord_facade;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    // msg.reply(ctx, help).await?;

    Ok(())
}
