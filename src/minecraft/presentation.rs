use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[group]
#[only_in(guilds)]
#[prefixes("mc", "minecraft")]
#[commands(register)]
struct Minecraft;

#[command]
async fn register(ctx: &Context, msg: &Message) -> CommandResult {
    todo!()
}
