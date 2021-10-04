use anyhow::Result;
use async_trait::async_trait;
use maciej_the_bot::{
    operator_commands::HELP_COMMAND as OPHELP_COMMAND,
    user_commands::{HELP_COMMAND, H_COMMAND},
};
use serenity::client::EventHandler;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::Client;
use structopt::StructOpt;

use maciej_the_bot::data::client_configuration::ClientConfiguration;

/// Maciej-the-bot is simple discord bot written in Rust
/// that uses [serenity-rs](https://github.com/serenity-rs/serenity) as a backend.
#[derive(StructOpt)]
struct Opt {
    /// Prefix, which triggers bot
    #[structopt(short, long, env, default_value = "!")]
    prefix: String,

    /// Token, used by bot to login to discord API
    #[structopt(short, long, env)]
    token: String,
}

#[group]
#[only_in(guilds)]
#[commands(h, help)]
struct General;

#[group]
#[only_in(guilds)]
#[prefixes("op")]
#[commands(ophelp)]
struct Operator;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Opt::from_args();

    // todo: add dynamic prefix
    let op_prefix = "op";

    let serenity_framework = StandardFramework::new()
        .configure(|c| c.prefix(&config.prefix))
        .group(&GENERAL_GROUP)
        .group(&OPERATOR_GROUP);

    let mut client = Client::builder(config.token)
        .event_handler(Handler)
        .framework(serenity_framework)
        .await?;

    {
        let mut client_data = client.data.write().await;

        client_data.insert::<ClientConfiguration>(ClientConfiguration::new(
            config.prefix,
            op_prefix.into(),
        ))
    }

    client.start().await?;

    Ok(())
}
