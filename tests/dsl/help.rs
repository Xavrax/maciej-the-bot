use cucumber_rust::{then, when};

use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::dsl::ScenarioEnvironment;
use maciej_the_bot::command::help::HelpCommand;
use maciej_the_bot::command::Command;
use maciej_the_bot::discord_facade::MockDiscordFacade;


use std::str::from_utf8;

#[when("command \"help\" is triggered")]
async fn trigger_help_message(env: &mut ScenarioEnvironment) {
    let mut help_file = File::open(format!("{}/help.txt", std::env!("CARGO_MANIFEST_DIR")))
        .await
        .unwrap();

    let mut content = vec![];
    help_file.read_to_end(&mut content).await.unwrap();

    let mut discord_mock = MockDiscordFacade::new();

    let fixed_content = from_utf8(&content).unwrap().replace("{}", "!");

    discord_mock
        .expect_reply()
        .withf(move |function_content| function_content == fixed_content)
        .returning(|_| {})
        .times(1);

    env.command_result = Some(HelpCommand.execute(discord_mock).await)
}

#[then("message should include \"help.txt\"")]
async fn check_if_help_message_includes_help_file(env: &mut ScenarioEnvironment) {
    assert!(env.command_result.take().unwrap().is_ok())
}
