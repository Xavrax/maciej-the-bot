use crate::discord_facade::DiscordFacade;
use anyhow::Result;
use async_trait::async_trait;

pub mod help;

#[async_trait]
pub trait Command {
    async fn execute<D>(self, discord: D) -> Result<()>
    where
        D: DiscordFacade + Send + 'static;
}
