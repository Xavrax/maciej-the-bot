use crate::discord::model::errors::check_msg;
use std::env;
use serenity::{
    prelude::Context,
    model::prelude::Message,
    framework::standard::{
        CommandResult,
        macros::command,
        Args
    }
};
use crate::discord::model::messages::say;

#[command]
pub fn ping(context: &mut Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&context.http, say("Pong!")));

    Ok(())
}

#[command]
pub fn prefix(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let prefix = match args.single::<String>() {
        Ok(prefix) => prefix,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, say("Must provide a prefix")));

            return Ok(());
        },
    };

    env::set_var("MACIEJ_PREFIX", prefix);

    Ok(())
}

#[command]
pub fn help(context: &mut Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&context.http, say("here will be help message!")));

    Ok(())
}