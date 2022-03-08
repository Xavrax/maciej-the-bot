use async_trait::async_trait;
use serenity::client::EventHandler;
use serenity::framework::StandardFramework;
use serenity::Client;

pub mod help;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {}

pub struct Bot;

impl Bot {
    pub fn configure<S>(token: S) -> BotBuilder
    where
        S: Into<String>,
    {
        BotBuilder {
            token: token.into(),
            ..Default::default()
        }
    }

    pub async fn run(bot_builder: BotBuilder) -> Result<(), crate::Error> {
        let serenity_framework = StandardFramework::new()
            .configure(|c| c.prefix(&bot_builder.prefix))
            .group(&help::HELP_GROUP)
            .group(&help::OPERATORHELP_GROUP);

        let mut client = Client::builder(bot_builder.token)
            .event_handler(Handler)
            .framework(serenity_framework)
            .await?;

        client.start().await?;

        Ok(())
    }
}

#[derive(Default)]
pub struct BotBuilder {
    token: String,
    prefix: String,
    operator_prefix: String,
}

impl BotBuilder {
    pub fn with_prefix<S>(mut self, prefix: S) -> Self
    where
        S: Into<String>,
    {
        self.prefix = prefix.into();

        self
    }

    pub fn with_operator_prefix<S>(mut self, operator_prefix: S) -> Self
    where
        S: Into<String>,
    {
        self.operator_prefix = operator_prefix.into();

        self
    }

    pub async fn run(self) -> Result<(), crate::Error> {
        Bot::run(self).await?;

        Ok(())
    }
}

pub type Error = Box<dyn std::error::Error + Sync + std::marker::Send + 'static>;
