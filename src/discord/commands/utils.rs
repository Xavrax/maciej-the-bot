use crate::discord::model::errors::check_msg;
use serenity::{
    prelude::Context,
    model::prelude::Message,
    framework::standard::{
        CommandResult,
        macros::command
    }
};

#[command]
pub fn ping(context: &mut Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&context.http, "Pong!"));

    Ok(())
}
