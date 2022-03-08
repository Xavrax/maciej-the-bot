use crate::application::help_service::prefix_configurable_help_service::PrefixConfigurableHelpService;
use crate::application::help_service::{HelpLevel, HelpService};
use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;

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
