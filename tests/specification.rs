// use cucumber_rust::WorldInit;
// use std::convert::Infallible;
// use maciej_the_bot::command::help::HelpLevel;
// use maciej_the_bot::command::Command;
//
// #[derive(Debug, WorldInit)]
// struct ScenarioEnvironment {}
//
// #[async_trait(?Send)]
// impl cucumber::World for ScenarioEnvironment {
//     type Error = Infallible;
//
//     async fn new() -> Result<Self, Self::Error> {
//         Ok(Self {})
//     }
// }
//
// #[when(regex = r"command `help` is triggered")]
// async fn trigger_help_command(w: &mut World, count: usize) {
//     let help_command = maciej_the_bot::command::help::HelpCommand::new(HelpLevel::User);
//
//     help_command.execute();
// }
//
// #[when(regex = r"operator command `help` is triggered")]
// async fn trigger_op_help_command(w: &mut World, count: usize) {
//
// }
//
//
// #[then(regex = r"message should include help from correct file")]
// async fn assert_file(w: &mut World) {
//
// }
//
// #[tokio::test]
// async fn specification() {
//     dsl::ScenarioEnvironment::init(&["./features"])
//         .run_and_exit()
//         .await;
// }
