use anyhow::Result;
use maciej_the_bot::help_message;




#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", help_message());

    Ok(())
}
