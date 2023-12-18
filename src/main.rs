use anyhow::Result;
use app::Bot;
use clap::Parser;
use dotenv::dotenv;

mod app;

/// Maciej-the-bot is simple discord bot written in Rust that uses serenity-rs as a backend.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Opts {
    #[clap(short, long, env)]
    /// Discord API token
    token: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let opts = Opts::parse();

    Bot::initialize(&opts.token, "!").await?.run().await?;

    Ok(())
}
