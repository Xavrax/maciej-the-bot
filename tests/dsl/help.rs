use cucumber_rust::{then, when};

use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::dsl::ScenarioEnvironment;
use maciej_the_bot::help_message;
use std::str::from_utf8;

#[when("binary should print commands help")]
async fn get_help_message(env: &mut ScenarioEnvironment) {
    env.output = help_message("!");
}

#[then("message should include \"help.txt\"")]
async fn check_if_help_message_includes_help_file(env: &mut ScenarioEnvironment) {
    let mut help_file = File::open(format!("{}/help.txt", std::env!("CARGO_MANIFEST_DIR")))
        .await
        .unwrap();

    let mut content = vec![];
    help_file.read_to_end(&mut content).await.unwrap();

    assert!(env
        .output
        .contains(&from_utf8(&content).unwrap().replace("{}", "!")));
}
