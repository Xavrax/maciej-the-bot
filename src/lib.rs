use crate::command::help::{HelpCommand, HelpLevel};
use crate::command::Command;
use crate::discord_facade::discord_facade_impl::DiscordFacadeImpl;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

pub mod command;
pub mod data;
pub mod discord_facade;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    HelpCommand::new(HelpLevel::User)
        .execute(DiscordFacadeImpl::new(ctx, msg))
        .await?;

    Ok(())
}

#[command]
async fn h(ctx: &Context, msg: &Message) -> CommandResult {
    HelpCommand::new(HelpLevel::User)
        .execute(DiscordFacadeImpl::new(ctx, msg))
        .await?;

    Ok(())
}

#[command]
async fn op_help(ctx: &Context, msg: &Message) -> CommandResult {
    HelpCommand::new(HelpLevel::Operator)
        .execute(DiscordFacadeImpl::new(ctx, msg))
        .await?;

    Ok(())
}
