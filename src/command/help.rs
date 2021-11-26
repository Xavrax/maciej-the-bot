use crate::command::Command;
use crate::data::client_configuration::ClientConfiguration;
use crate::discord_facade::DiscordFacade;
use anyhow::anyhow;
use async_trait::async_trait;

pub enum HelpLevel {
    User,
    Operator,
}

pub struct HelpCommand {
    level: HelpLevel,
}

impl HelpCommand {
    pub fn new(level: HelpLevel) -> Self {
        Self { level }
    }
}

#[async_trait]
impl Command for HelpCommand {
    async fn execute<D>(self, discord: D) -> anyhow::Result<()>
    where
        D: DiscordFacade,
    {
        let (prefix, operator_prefix) = {
            let config = discord
                .get_data()
                .read()
                .await
                .get::<ClientConfiguration>()
                .ok_or_else(|| anyhow!("Client configuration not initialized!"))?
                .clone();

            let prefix = config.read().await.prefix.clone();
            let operator_prefix = config.read().await.operator_prefix.clone();

            (prefix, operator_prefix)
        };

        let message = match self.level {
            HelpLevel::User => include_str!("../../messages/help.txt").replace("{prefix}", &prefix),
            HelpLevel::Operator => include_str!("../../messages/op_help.txt")
                .replace("{prefix}", &prefix)
                .replace("{op_prefix}", &operator_prefix),
        };

        discord.reply(message.as_str()).await?;

        Ok(())
    }
}

#[cfg(test)]
mod should {
    use crate::command::help::{HelpCommand, HelpLevel};
    use crate::command::Command;
    use crate::data::client_configuration::ClientConfiguration;
    use crate::discord_facade::MockDiscordFacade;
    use serenity::prelude::TypeMap;
    use std::ops::Add;
    use std::sync::Arc;
    use test_case::test_case;
    use tokio::sync::RwLock;

    #[test_case(HelpLevel::User; "on user level")]
    #[test_case(HelpLevel::Operator; "on operator level")]
    #[tokio::test]
    async fn send_help_message(help_level: HelpLevel) {
        let prefix: String = "$$".into();
        let op_prefix: String = "##".into();

        let facade = create_discord_facade(&help_level, prefix, &op_prefix).await;

        let result = HelpCommand::new(help_level).execute(facade).await;

        assert!(result.is_ok());
    }

    async fn create_discord_facade(
        help_level: &HelpLevel,
        prefix: String,
        op_prefix: &String,
    ) -> MockDiscordFacade {
        let full_prefix = match help_level {
            HelpLevel::User => prefix.clone(),
            HelpLevel::Operator => prefix.clone().add(&op_prefix),
        };

        let mut mock = MockDiscordFacade::new();
        mock.expect_reply()
            .withf(move |content| content.contains(&full_prefix))
            .times(1)
            .return_once(|_| Ok(()));

        add_prefix_to_mock(&mut mock, prefix, op_prefix.clone()).await;

        mock
    }

    async fn add_prefix_to_mock(mock: &mut MockDiscordFacade, prefix: String, op_prefix: String) {
        let data = Arc::new(RwLock::new(TypeMap::new()));

        {
            let mut client_data = data.write().await;

            client_data.insert::<ClientConfiguration>(ClientConfiguration::new(prefix, op_prefix))
        }

        mock.expect_get_data().times(1).return_once(|| data);
    }
}
