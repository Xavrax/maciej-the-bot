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

    mod dsl {
        use crate::discord_facade::MockDiscordFacade;
        use anyhow::Result;
        use crate::command::help::HelpCommand;
        use crate::command::Command;

        #[derive(Default)]
        pub struct ScenarioEnvironment {
            pub facade: Option<MockDiscordFacade>,

            pub result: Option<Result<()>>,
        }

        pub fn given_discord_facade(env: &mut ScenarioEnvironment) {
            env.facade = Some({
                let mut mock = MockDiscordFacade::new();
                mock.expect_reply().times(1).return_once(|_| Ok(()));

                mock
            })
        }

        pub async fn when_command_is_triggered(env: &mut ScenarioEnvironment) {
            env.result = Some(HelpCommand.execute(env.facade.take().unwrap()).await);
        }

        pub fn then_should_reply_with_help(env: &mut ScenarioEnvironment) {
            assert!(env.result.take().unwrap().is_ok());
        }

    }
}