use operator::HELP_COMMAND as OPHELP_COMMAND;
use serenity::client::Context;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use user::{HELP_COMMAND, H_COMMAND};

use crate::help::application::prefix_configurable_help_service::PrefixConfigurableHelpService;
use crate::help::application::{HelpLevel, HelpService};

#[group]
#[only_in(guilds)]
#[commands(h, help)]
struct Help;

pub mod user {
    use super::*;

    #[command]
    async fn help(ctx: &Context, msg: &Message) -> CommandResult {
        help_function(ctx, msg).await
    }

    #[command]
    async fn h(ctx: &Context, msg: &Message) -> CommandResult {
        help_function(ctx, msg).await
    }

    async fn help_function(ctx: &Context, msg: &Message) -> Result<(), crate::Error> {
        // todo: how to move this?
        let help_service = PrefixConfigurableHelpService::new("!", "op");
        let help_message = help_service.help(HelpLevel::User);

        msg.reply(ctx, help_message).await?;

        Ok(())
    }
}

#[group]
#[only_in(guilds)]
#[prefixes("op")]
#[commands(ophelp)]
struct OperatorHelp;

pub mod operator {
    use super::*;

    #[command]
    async fn help(ctx: &Context, msg: &Message) -> CommandResult {
        // todo: how to move this?
        let help_service = PrefixConfigurableHelpService::new("!", "op");
        let help_message = help_service.help(HelpLevel::Operator);

        msg.reply(ctx, help_message).await?;

        Ok(())
    }
}
