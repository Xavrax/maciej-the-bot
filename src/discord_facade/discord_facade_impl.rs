use crate::discord_facade::DiscordFacade;
use async_trait::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use std::fmt::Display;

pub struct DiscordFacadeImpl<'a> {
    context: &'a Context,
    message: &'a Message,
}

#[async_trait]
impl<'a> DiscordFacade for DiscordFacadeImpl<'a> {
    async fn reply(&self, content: &str) {
        self.message.reply(self.context, content).await;
    }
}
