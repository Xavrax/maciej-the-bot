use crate::infra::commands::PING_COMMAND;
use anyhow::{anyhow, Result};
use serenity::{
    all::{Message, Ready},
    framework::{
        standard::{
            macros::{group},
            Configuration,
        },
        StandardFramework,
    },
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};

mod hooks;

struct Handler;

#[group]
#[commands(ping)]
struct General;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, _: Context, msg: Message) {
        log::debug!("Got message by {}: '{:?}'", msg.author.name, msg);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);
    }
}

pub struct Bot {
    client: Client,
}

impl Bot {
    pub async fn initialize(token: &str, prefix: &str) -> Result<Self> {
        let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

        let framework = StandardFramework::new()
            .after(hooks::after)
            .before(hooks::before)
            .unrecognised_command(hooks::unknown_command)
            .group(&GENERAL_GROUP);

        framework.configure(Configuration::new().with_whitespace(false).prefix(prefix));

        let client = Client::builder(token, intents)
            .framework(framework)
            .event_handler(Handler)
            .await?;

        Ok(Self { client })
    }

    pub async fn run(&mut self) -> Result<()> {
        log::info!("Starting bot");
        self.client.start().await.map_err(|e| anyhow!(e))
    }
}
