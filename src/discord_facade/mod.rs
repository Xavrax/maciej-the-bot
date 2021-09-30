use async_trait::async_trait;
use std::fmt::Display;

mod discord_facade_impl;

#[mockall::automock]
#[async_trait]
pub trait DiscordFacade: Send + Sync {
    async fn reply(&self, content: &str);
}
