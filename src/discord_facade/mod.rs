use async_trait::async_trait;
use anyhow::Result;

mod discord_facade_impl;

#[mockall::automock]
#[async_trait]
pub trait DiscordFacade: Send + Sync {
    async fn reply(&self, content: &str) -> Result<()>;
}
