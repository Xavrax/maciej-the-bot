use crate::command::Command;
use crate::discord_facade::DiscordFacade;
use async_trait::async_trait;

pub struct HelpCommand;

#[async_trait]
impl Command for HelpCommand {
    async fn execute<D>(self, discord: D) -> anyhow::Result<()>
    where
        D: DiscordFacade + 'static,
    {
        let prefix = "!";

        discord
            .reply(
                include_str!("../../help.txt")
                    .replace("{}", prefix)
                    .as_str(),
            )
            .await;

        Ok(())
    }
}
