use cucumber_rust::{then, when};


use tokio::io::AsyncReadExt;

use crate::dsl::ScenarioEnvironment;
use maciej_the_bot::command::help::{HelpLevel};



use crate::dsl::help::dsl::{trigger_help_message};


#[when("command \"help\" is triggered")]
async fn trigger_user_help_message(env: &mut ScenarioEnvironment) {
    trigger_help_message(env, "help.txt", HelpLevel::User).await;
}

#[when("operator command \"help\" is triggered")]
async fn trigger_operator_help_message(env: &mut ScenarioEnvironment) {
    trigger_help_message(env, "op_help.txt", HelpLevel::Operator).await;
}

#[then("message should include help from correct file")]
async fn check_if_help_message_includes_help_file(env: &mut ScenarioEnvironment) {
    assert!(env.command_result.take().unwrap().is_ok())
}

mod dsl {
    use crate::dsl::ScenarioEnvironment;
    use maciej_the_bot::command::help::{HelpCommand, HelpLevel};
    use maciej_the_bot::command::Command;
    use maciej_the_bot::data::client_configuration::ClientConfiguration;
    use maciej_the_bot::discord_facade::MockDiscordFacade;
    use serenity::prelude::TypeMap;
    use std::str::from_utf8;
    use std::sync::Arc;
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;
    use tokio::sync::RwLock;

    pub async fn add_prefix_to_mock(
        mock: &mut MockDiscordFacade,
        prefix: String,
        op_prefix: String,
    ) {
        let data = Arc::new(RwLock::new(TypeMap::new()));

        {
            let mut client_data = data.write().await;

            client_data.insert::<ClientConfiguration>(ClientConfiguration::new(prefix, op_prefix))
        }

        mock.expect_get_data().times(1).return_once(|| data);
    }

    pub async fn trigger_help_message(
        env: &mut ScenarioEnvironment,
        filename: &str,
        help_level: HelpLevel,
    ) {
        let prefix = "!".to_owned();
        let op_prefix = "op".to_owned();

        let mut help_file = File::open(format!(
            "{}/messages/{}",
            std::env!("CARGO_MANIFEST_DIR"),
            filename,
        ))
        .await
        .unwrap();

        let mut content = vec![];
        help_file.read_to_end(&mut content).await.unwrap();

        let fixed_content = from_utf8(&content)
            .unwrap()
            .replace("{prefix}", &prefix)
            .replace("{op_prefix}", &op_prefix);

        let mut discord_mock = MockDiscordFacade::new();

        discord_mock
            .expect_reply()
            .withf(move |function_content| function_content == fixed_content)
            .returning(|_| Ok(()))
            .times(1);

        add_prefix_to_mock(&mut discord_mock, prefix, op_prefix).await;

        env.command_result = Some(HelpCommand::new(help_level).execute(discord_mock).await)
    }
}
