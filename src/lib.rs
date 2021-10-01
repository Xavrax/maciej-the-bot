use crate::command::help::HelpCommand;
use crate::command::Command;
use crate::discord_facade::discord_facade_impl::DiscordFacadeImpl;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

pub mod command;
pub mod discord_facade;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    HelpCommand
        .execute(DiscordFacadeImpl::new(ctx, msg))
        .await?;

    Ok(())
}
