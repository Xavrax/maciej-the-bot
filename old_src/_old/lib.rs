use crate::command::help::{HelpCommand, HelpLevel};
use crate::command::Command;
use crate::discord_facade::discord_facade_impl::DiscordFacadeImpl;
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

pub mod checks;
pub mod command;
pub mod data;
pub mod discord_facade;

pub mod user_commands {
    use super::*;
    use serenity::framework::standard::CommandError;

    #[command]
    async fn help(ctx: &Context, msg: &Message) -> CommandResult {
        help_function(ctx, msg).await
    }

    #[command]
    async fn h(ctx: &Context, msg: &Message) -> CommandResult {
        help_function(ctx, msg).await
    }

    async fn help_function(ctx: &Context, msg: &Message) -> Result<(), CommandError> {
        HelpCommand::new(HelpLevel::User)
            .execute(DiscordFacadeImpl::new(ctx, msg))
            .await?;

        Ok(())
    }
}

pub mod operator_commands {
    use super::*;
    #[command]
    async fn help(ctx: &Context, msg: &Message) -> CommandResult {
        HelpCommand::new(HelpLevel::Operator)
            .execute(DiscordFacadeImpl::new(ctx, msg))
            .await?;

        Ok(())
    }
}

pub mod discord_checks {}
