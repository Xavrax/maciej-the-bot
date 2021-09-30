use anyhow::Result;
use maciej_the_bot::help_message;
use std::str::from_utf8;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", help_message());

    Ok(())
}
