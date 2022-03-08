use crate::config_repository::ConfigRepository;
use async_trait::async_trait;
use serenity::client::EventHandler;
use serenity::framework::StandardFramework;
use serenity::Client;

pub mod config_repository;
mod help;
mod minecraft;

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
            .group(&help::OPERATORHELP_GROUP)
            .group(&minecraft::MINECRAFT_GROUP);

        let mut client = Client::builder(bot_builder.token)
            .event_handler(Handler)
            .framework(serenity_framework)
            .await?;

        {
            let mut data = client.data.write().await;

            data.insert::<ConfigRepository>(ConfigRepository::create(
                bot_builder.ftp_ip,
                bot_builder.ftp_login,
                bot_builder.ftp_passwd,
            ))
        }

        client.start().await?;

        Ok(())
    }
}

#[derive(Default)]
pub struct BotBuilder {
    token: String,

    prefix: String,
    operator_prefix: String,

    ftp_login: String,
    ftp_passwd: String,
    ftp_ip: String,
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

    pub fn with_ftp_credentials<S>(mut self, ip: S, login: S, passwd: S) -> Self
    where
        S: Into<String>,
    {
        self.ftp_ip = ip.into();
        self.ftp_login = login.into();
        self.ftp_passwd = passwd.into();

        self
    }

    pub async fn run(self) -> Result<(), crate::Error> {
        Bot::run(self).await?;

        Ok(())
    }
}

pub type Error = Box<dyn std::error::Error + Sync + std::marker::Send + 'static>;
