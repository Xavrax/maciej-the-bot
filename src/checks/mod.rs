use crate::discord_facade::DiscordFacade;
use anyhow::Result;
use async_trait::async_trait;

pub mod feature_check;

#[async_trait]
pub trait Check {
    async fn check<D>(self, discord: D) -> Result<bool>
    where
        D: DiscordFacade;
}
