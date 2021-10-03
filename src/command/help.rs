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
    use crate::command::help::HelpLevel;
    use dsl::*;
    use test_case::test_case;

    #[test_case(HelpLevel::User; "on user level")]
    #[test_case(HelpLevel::Operator; "on operator level")]
    #[tokio::test]
    async fn send_help_message(level: HelpLevel) {
        let mut env = ScenarioEnvironment::default();

        given_help_level(level, &mut env);
        and_given_prefix(&mut env);
        and_given_discord_facade(&mut env).await;
        when_command_is_triggered(&mut env).await;
        then_should_reply_with_help(&mut env);
        and_then_prefix_should_be_included_to_message(&mut env);
    }

    mod dsl {
        use crate::command::help::{HelpCommand, HelpLevel};
        use crate::command::Command;
        use crate::data::client_configuration::ClientConfiguration;
        use crate::discord_facade::MockDiscordFacade;
        use anyhow::Result;
        use serenity::prelude::TypeMap;
        use std::ops::Add;
        use std::sync::Arc;
        use tokio::sync::RwLock;

        #[derive(Default)]
        pub struct ScenarioEnvironment {
            pub facade: Option<MockDiscordFacade>,
            pub prefix: Option<String>,
            pub op_prefix: Option<String>,
            pub help_level: Option<HelpLevel>,

            pub result: Option<Result<()>>,
        }

        pub fn given_help_level(level: HelpLevel, env: &mut ScenarioEnvironment) {
            env.help_level = Some(level);
        }

        pub async fn and_given_discord_facade(env: &mut ScenarioEnvironment) {
            env.facade = Some({
                let full_prefix = match env.help_level.as_ref().unwrap() {
                    HelpLevel::User => env.prefix.as_ref().unwrap().clone(),
                    HelpLevel::Operator => env
                        .prefix
                        .as_ref()
                        .unwrap()
                        .clone()
                        .add(env.op_prefix.as_ref().unwrap().as_str()),
                };

                let mut mock = MockDiscordFacade::new();
                mock.expect_reply()
                    .withf(move |content| content.contains(&full_prefix))
                    .times(1)
                    .return_once(|_| Ok(()));

                add_prefix_to_mock(
                    &mut mock,
                    env.prefix.take().unwrap(),
                    env.op_prefix.take().unwrap(),
                )
                .await;

                mock
            })
        }

        pub fn and_given_prefix(env: &mut ScenarioEnvironment) {
            env.prefix = Some("$$".into());
            env.op_prefix = Some("##".into());
        }

        pub async fn when_command_is_triggered(env: &mut ScenarioEnvironment) {
            env.result = Some(
                HelpCommand::new(env.help_level.take().unwrap())
                    .execute(env.facade.take().unwrap())
                    .await,
            );
        }

        pub fn then_should_reply_with_help(env: &mut ScenarioEnvironment) {
            assert!(env.result.as_ref().unwrap().is_ok());
        }

        pub fn and_then_prefix_should_be_included_to_message(env: &mut ScenarioEnvironment) {
            assert!(env.result.take().unwrap().is_ok());
        }

        async fn add_prefix_to_mock(
            mock: &mut MockDiscordFacade,
            prefix: String,
            op_prefix: String,
        ) {
            let data = Arc::new(RwLock::new(TypeMap::new()));

            {
                let mut client_data = data.write().await;

                client_data
                    .insert::<ClientConfiguration>(ClientConfiguration::new(prefix, op_prefix))
            }

            mock.expect_get_data().times(1).return_once(|| data);
        }
    }
}
