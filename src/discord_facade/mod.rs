use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use serenity::prelude::TypeMap;

pub mod discord_facade_impl;

#[mockall::automock]
#[async_trait]
pub trait DiscordFacade: Send + Sync {
    async fn reply(&self, content: &str) -> Result<()>;
    fn get_data(&self) -> Arc<RwLock<TypeMap>>;
}
