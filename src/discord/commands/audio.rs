use crate::discord::model::{
    managers::VoiceManager,
    errors::check_msg
};
use serenity::{
    prelude::Context,
    model::prelude::Message,
    framework::standard::{
        CommandResult,
        macros::command
    }
};
use crate::discord::model::messages::say;

#[command]
pub fn deafen(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, say("Groups and DMs not supported")));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().unwrap();
    let mut manager = manager_lock.lock();

    let handler = match manager.get_mut(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(&ctx, say("Not in a voice channel")));

            return Ok(());
        },
    };

    if handler.self_deaf {
        check_msg(msg.channel_id.say(&ctx.http, say("Already deafened")));
    } else {
        handler.deafen(true);

        check_msg(msg.channel_id.say(&ctx.http, say("Deafened")));
    }

    Ok(())
}



#[command]
pub fn mute(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, say("Groups and DMs not supported")));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    let handler = match manager.get_mut(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(&ctx, say("Not in a voice channel")));

            return Ok(());
        },
    };

    if handler.self_mute {
        check_msg(msg.channel_id.say(&ctx.http, say("Already muted")));
    } else {
        handler.mute(true);

        check_msg(msg.channel_id.say(&ctx.http, say("Now muted")));
    }

    Ok(())
}

#[command]
pub fn undeafen(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, say("Error finding channel info")));

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) {
        handler.deafen(false);

        check_msg(msg.channel_id.say(&ctx.http, say("Undeafened")));
    } else {
        check_msg(msg.channel_id.say(&ctx.http, say("Not in a voice channel to undeafen in")));
    }

    Ok(())
}

#[command]
pub fn unmute(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, say("Error finding channel info")));

            return Ok(());
        },
    };
    let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) {
        handler.mute(false);

        check_msg(msg.channel_id.say(&ctx.http, say("Unmuted")));
    } else {
        check_msg(msg.channel_id.say(&ctx.http, say("Not in a voice channel to unmute in")));
    }

    Ok(())
}
