use crate::command::Command;
use crate::discord_facade::DiscordFacade;
use async_trait::async_trait;

pub struct HelpCommand;

#[async_trait]
impl Command for HelpCommand {
    async fn execute<D>(self, discord: D) -> anyhow::Result<()>
    where
        D: DiscordFacade,
    {
        // todo: resource
        let prefix = "!";

        discord
            .reply(
                include_str!("../../help.txt")
                    .replace("{}", prefix)
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

        given_discord_facade(&mut env);
        when_command_is_triggered(&mut env).await;
        then_should_reply_with_help(&mut env);
    }

    #[tokio::test]
    async fn replace_placeholders_to_prefix() {
        let mut env = ScenarioEnvironment::default();

        given_prefix(&mut env);
        and_given_discord_facade_expecting_prefix(&mut env);
        when_command_is_triggered(&mut env).await;
        then_prefix_should_be_included_to_message(&mut env);
    }



    mod dsl {
        use crate::discord_facade::MockDiscordFacade;
        use anyhow::Result;
        use crate::command::help::HelpCommand;
        use crate::command::Command;

        #[derive(Default)]
        pub struct ScenarioEnvironment {
            pub facade: Option<MockDiscordFacade>,
            pub prefix: Option<String>,

            pub result: Option<Result<()>>,
        }

        pub fn given_discord_facade(env: &mut ScenarioEnvironment) {
            env.facade = Some({
                let mut mock = MockDiscordFacade::new();
                mock.expect_reply().times(1).return_once(|_| Ok(()));

                mock
            })
        }

        pub fn and_given_discord_facade_expecting_prefix(env: &mut ScenarioEnvironment) {
            env.facade = Some({
                let prefix = env.prefix.take().unwrap();

                let mut mock = MockDiscordFacade::new();
                mock.expect_reply().withf(move |content| content.contains(&prefix) ).times(1).return_once(|_| Ok(()));

                mock
            })
        }

        pub fn given_prefix(env: &mut ScenarioEnvironment) {
            env.prefix = Some("!".into());
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
    }
}