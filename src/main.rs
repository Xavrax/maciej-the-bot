use log::info;
use std::fmt::Debug;
use structopt::StructOpt;

/// Maciej-the-bot is simple discord bot written in Rust
/// that uses [serenity-rs](https://github.com/serenity-rs/serenity) as a backend.
#[derive(StructOpt, Debug)]
struct Opt {
    /// Prefix, which triggers bot
    #[structopt(short, long, env, default_value = "!")]
    prefix: String,

    /// Token, used by bot to login to discord API
    #[structopt(short, long, env)]
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), maciej_the_bot::Error> {
    let config = Opt::from_args();

    env_logger::init();
    info!("Maciej The Bot {}", env!("CARGO_PKG_VERSION"));
    info!("Launched with: {:?}", config);

    // todo: add dynamic prefix
    let op_prefix = "op";

    // Bot::configure(config.token)
    //     .with_prefix(config.prefix)
    //     .with_operator_prefix(op_prefix)
    //     .run()
    //     .await?;

    Ok(())
}
