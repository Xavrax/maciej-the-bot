use anyhow::Result;
use async_trait::async_trait;
use serenity::model::guild::Guild;
use serenity::prelude::TypeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod discord_facade_impl;

#[mockall::automock]
#[async_trait]
pub trait DiscordFacade: Send + Sync {
    async fn reply(&self, content: &str) -> Result<()>;
    fn get_data(&self) -> Arc<RwLock<TypeMap>>;
    fn get_guild(&self) -> Guild;
}
