use crate::data::server_configuration::Feature;
use anyhow::Result;

pub struct FeatureCheck {
    feature: Feature,
}

#[cfg(test)]
mod should {
    use dsl::*;

    #[tokio::test]
    async fn return_ok_if_server_enables_feature() {
        let mut env = ScenarioEnvironment::default();

        given_server_id(&mut env);
        and_given_feature(&mut env);
        and_given_server_config_with_enabled_feature(&mut env).await;
        when_check_is_triggered(&mut env);
        then_should_return_ok(&mut env);
    }

    mod dsl {
        use crate::checks::feature_check::*;
        use crate::data::server_configuration::{
            Feature, ServerConfiguration, ServersConfiguration,
        };
        use crate::discord_facade::MockDiscordFacade;

        use serenity::prelude::TypeMap;
        use std::sync::Arc;
        use tokio::sync::RwLock;

        #[derive(Default)]
        pub struct ScenarioEnvironment {
            pub server_id: u32,
            pub server_config: u32,
            pub feature: Option<Feature>,
            pub facade: Option<MockDiscordFacade>,

            pub result: Option<Result<()>>,
        }

        pub fn given_server_id(env: &mut ScenarioEnvironment) {
            env.server_id = 0;
        }

        pub fn and_given_feature(env: &mut ScenarioEnvironment) {
            env.feature = Some(Feature::YouTube);
        }

        pub async fn and_given_server_config_with_enabled_feature(env: &mut ScenarioEnvironment) {
            let data = Arc::new(RwLock::new(TypeMap::new()));

            {
                let mut client_data = data.write().await;

                let mut config = ServersConfiguration::new();

                let mut concrete_config = ServerConfiguration::default();
                concrete_config.features.push(env.feature.unwrap());

                config.write().await.0.insert(0, concrete_config);

                client_data.insert::<ServersConfiguration>(config)
            }
            let mut mock = MockDiscordFacade::new();

            mock.expect_get_data().times(1).return_once(|| data);

            env.facade = Some(mock);
        }

        pub fn when_check_is_triggered(env: &mut ScenarioEnvironment) {
            let check = FeatureCheck::new(env.server_id, Feature::YouTube);

            env.result = Some(check.check(env.facade.take().unwrap()));
        }

        pub fn then_should_return_ok(env: &mut ScenarioEnvironment) {
            assert!(env.result.take().unwrap().is_ok());
        }
    }
}
