use cucumber_rust::{then, when};

use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::dsl::ScenarioEnvironment;
use maciej_the_bot::command::help::HelpCommand;
use maciej_the_bot::command::Command;
use maciej_the_bot::discord_facade::MockDiscordFacade;

use std::str::from_utf8;
use crate::dsl::help::dsl::add_prefix_to_mock;

#[when("command \"help\" is triggered")]
async fn trigger_help_message(env: &mut ScenarioEnvironment) {
    let prefix = "!".to_owned();
    let mut help_file = File::open(format!("{}/help.txt", std::env!("CARGO_MANIFEST_DIR")))
        .await
        .unwrap();

    let mut content = vec![];
    help_file.read_to_end(&mut content).await.unwrap();

    let mut discord_mock = MockDiscordFacade::new();

    let fixed_content = from_utf8(&content).unwrap().replace("{}", &prefix);

    discord_mock
        .expect_reply()
        .withf(move |function_content| function_content == fixed_content)
        .returning(|_| Ok(()))
        .times(1);

    add_prefix_to_mock(&mut discord_mock, prefix).await;

    env.command_result = Some(HelpCommand.execute(discord_mock).await)
}

#[then("message should include \"help.txt\"")]
async fn check_if_help_message_includes_help_file(env: &mut ScenarioEnvironment) {
    assert!(env.command_result.take().unwrap().is_ok())
}

mod dsl {
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use serenity::prelude::TypeMap;
    use maciej_the_bot::data::client_configuration::ClientConfiguration;
    use maciej_the_bot::discord_facade::MockDiscordFacade;

    pub async fn add_prefix_to_mock(mock: &mut MockDiscordFacade, prefix: String) {
        let data = Arc::new(RwLock::new(TypeMap::new()));

        {
            let mut client_data = data.write().await;

            client_data.insert::<ClientConfiguration>(ClientConfiguration::new(prefix))
        }

        mock.expect_get_data()
            .times(1)
            .return_once(|| data);
    }
}