use anyhow::Result;
use cucumber_rust::{async_trait, World, WorldInit};
use std::convert::Infallible;

mod help;

#[derive(WorldInit, Default)]
pub struct ScenarioEnvironment {
    command_result: Option<Result<()>>,
}

#[async_trait(?Send)]
impl World for ScenarioEnvironment {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(Self { ..Self::default() })
    }
}
