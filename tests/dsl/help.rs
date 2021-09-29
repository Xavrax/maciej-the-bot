use cucumber_rust::{then, when};

use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::dsl::ScenarioEnvironment;
use std::str::from_utf8;

#[when("binary is ran with \"--help\" flag")]
async fn binary_with_help_flag(env: &mut ScenarioEnvironment) {
    let output = tokio::process::Command::new(std::env!("CARGO_BIN_EXE_maciej-the-bot"))
        .args(&["--help"])
        .kill_on_drop(true)
        .output()
        .await
        .unwrap()
        .stdout;

    env.output = from_utf8(&output).unwrap().to_owned();
}

#[then("message should include \"help.txt\"")]
async fn check_if_help_message_includes_help_file(env: &mut ScenarioEnvironment) {
    let mut help_file = File::open(format!("{}/help.txt", std::env!("CARGO_MANIFEST_DIR")))
        .await
        .unwrap();

    let mut content = vec![];
    help_file.read_to_end(&mut content).await;

    assert!(env.output.contains(from_utf8(&content).unwrap()));
}
