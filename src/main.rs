use anyhow::Result;
use std::str::from_utf8;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

async fn help() -> Result<()> {
    let mut help_file = File::open(format!("{}/help.txt", std::env!("CARGO_MANIFEST_DIR")))
        .await
        .unwrap();

    let mut content = vec![];
    help_file.read_to_end(&mut content).await;
    println!("{}", from_utf8(&content)?);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    help().await;

    Ok(())
}
