use crate::command::Command;
use crate::data::client_configuration::ClientConfiguration;
use crate::discord_facade::DiscordFacade;
use anyhow::anyhow;
use async_trait::async_trait;

pub struct HelpCommand;

#[async_trait]
impl Command for HelpCommand {
    async fn execute<D>(self, discord: D) -> anyhow::Result<()>
    where
        D: DiscordFacade,
    {
        let prefix = {
            discord
                .get_data()
                .read()
                .await
                .get::<ClientConfiguration>()
                .ok_or_else(|| anyhow!("Client configuration not initialized!"))?
                .read()
                .await
                .prefix
                .clone()
        };

        discord
            .reply(
                include_str!("../../messages/help.txt")
                    .replace("{prefix}", &prefix)
                    .as_str(),
            )
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod should {
    use dsl::*;

    #[tokio::test]
    async fn send_help_message() {
        let mut env = ScenarioEnvironment::default();

        given_discord_facade(&mut env).await;
        when_command_is_triggered(&mut env).await;
        then_should_reply_with_help(&mut env);
    }

    #[tokio::test]
    async fn replace_placeholders_to_prefix() {
        let mut env = ScenarioEnvironment::default();

        given_prefix(&mut env);
        and_given_discord_facade_expecting_prefix(&mut env).await;
        when_command_is_triggered(&mut env).await;
        then_prefix_should_be_included_to_message(&mut env);
    }

    mod dsl {
        use crate::command::help::HelpCommand;
        use crate::command::Command;
        use crate::data::client_configuration::ClientConfiguration;
        use crate::discord_facade::MockDiscordFacade;
        use anyhow::Result;
        use serenity::prelude::TypeMap;
        use std::sync::Arc;
        use tokio::sync::RwLock;

        #[derive(Default)]
        pub struct ScenarioEnvironment {
            pub facade: Option<MockDiscordFacade>,
            pub prefix: Option<String>,

            pub result: Option<Result<()>>,
        }

        pub async fn given_discord_facade(env: &mut ScenarioEnvironment) {
            env.facade = Some({
                let mut mock = MockDiscordFacade::new();
                mock.expect_reply().times(1).return_once(|_| Ok(()));
                add_prefix_to_mock(&mut mock, "!".into()).await;

                mock
            })
        }

        pub async fn and_given_discord_facade_expecting_prefix(env: &mut ScenarioEnvironment) {
            env.facade = Some({
                let prefix = env.prefix.take().unwrap();
                let cloned_prefix = prefix.clone();

                let mut mock = MockDiscordFacade::new();
                mock.expect_reply()
                    .withf(move |content| content.contains(&cloned_prefix))
                    .times(1)
                    .return_once(|_| Ok(()));

                add_prefix_to_mock(&mut mock, prefix).await;

                mock
            })
        }

        pub fn given_prefix(env: &mut ScenarioEnvironment) {
            env.prefix = Some("$$".into());
        }

        pub async fn when_command_is_triggered(env: &mut ScenarioEnvironment) {
            env.result = Some(HelpCommand.execute(env.facade.take().unwrap()).await);
        }

        pub fn then_should_reply_with_help(env: &mut ScenarioEnvironment) {
            assert!(env.result.take().unwrap().is_ok());
        }

        pub fn then_prefix_should_be_included_to_message(env: &mut ScenarioEnvironment) {
            assert!(env.result.take().unwrap().is_ok());
        }

        async fn add_prefix_to_mock(mock: &mut MockDiscordFacade, prefix: String) {
            let data = Arc::new(RwLock::new(TypeMap::new()));

            {
                let mut client_data = data.write().await;

                client_data.insert::<ClientConfiguration>(ClientConfiguration::new(prefix))
            }

            mock.expect_get_data().times(1).return_once(|| data);
        }
    }
}
