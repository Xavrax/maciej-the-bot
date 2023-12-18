use anyhow::{anyhow, Result};
use serenity::{
    all::Message,
    framework::StandardFramework,
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};

mod commands;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Do nothing
    }
}

pub struct Bot {
    client: Client,
}

impl Bot {
    pub async fn initialize(token: &str, prefix: &str) -> Result<Self> {
        let intents = GatewayIntents::non_privileged();

        let client = Client::builder(token, intents)
            .event_handler(Handler)
            .await?;

        Ok(Self { client })
    }

    pub async fn run(&mut self) -> Result<()> {
        self.client.start().await.map_err(|e| anyhow!(e))
    }
}
