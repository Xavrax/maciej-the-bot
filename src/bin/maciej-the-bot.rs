use anyhow::Result;
use structopt::StructOpt;

/// Maciej-the-bot is simple discord bot written in Rust
/// that uses [serenity-rs](https://github.com/serenity-rs/serenity) as a backend.
#[derive(StructOpt)]
struct Opt {
    /// Prefix for bot commands
    #[structopt(short, long, env, default_value = "!")]
    prefix: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Opt::from_args();

    Ok(())
}
