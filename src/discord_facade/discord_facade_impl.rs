use crate::discord_facade::DiscordFacade;
use anyhow::Result;
use async_trait::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

pub struct DiscordFacadeImpl<'a> {
    context: &'a Context,
    message: &'a Message,
}

impl<'a> DiscordFacadeImpl<'a> {
    pub fn new(context: &'a Context, message: &'a Message) -> Self {
        Self { context, message }
    }
}

#[async_trait]
impl<'a> DiscordFacade for DiscordFacadeImpl<'a> {
    async fn reply(&self, content: &str) -> Result<()> {
        self.message.reply(self.context, content).await?;

        Ok(())
    }
}
