use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

pub mod command;
pub mod discord_facade;

#[command]
async fn help(_ctx: &Context, _msg: &Message) -> CommandResult {
    msg.reply(ctx, help).await?;

    Ok(())
}
