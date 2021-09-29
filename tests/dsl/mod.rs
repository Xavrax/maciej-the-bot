use cucumber_rust::{async_trait, World, WorldInit};
use std::convert::Infallible;

mod help;

#[derive(WorldInit, Default)]
pub struct ScenarioEnvironment {
    output: String,
}

#[async_trait(?Send)]
impl World for ScenarioEnvironment {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(Self { ..Self::default() })
    }
}
