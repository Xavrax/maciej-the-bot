use crate::discord_facade::DiscordFacade;
use anyhow::Result;
use async_trait::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::guild::Guild;
use serenity::prelude::{RwLock, TypeMap};
use std::sync::Arc;

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

    fn get_data(&self) -> Arc<RwLock<TypeMap>> {
        self.context.data.clone()
    }

    fn get_guild(&self) -> Guild {
        // self.message.guild_id.
    }
}
