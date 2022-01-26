use serenity::framework::StandardFramework;
use serenity::Client;

mod commands;
mod groups;
mod handlers;

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
            .group(&groups::GENERAL_GROUP)
            .group(&groups::OPERATOR_GROUP);

        let mut client = Client::builder(bot_builder.token)
            .event_handler(handlers::Handler)
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
