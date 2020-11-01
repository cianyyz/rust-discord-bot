use crate::data::cache::{BotId, BotOwners, DefaultPrefix, GuildPrefixes};
use log::error;
use serenity::{
    framework::standard::{macros::hook, CommandError, DispatchError},
    model::prelude::*,
    prelude::*,
};

//Sends non-command DMs from regular users to the bot owners.
#[hook]
pub async fn log_dm(ctx: &Context, message: &Message) {
    if message.guild_id.is_some() {
        return;
    }

    let data = ctx.data.read().await;
    let bot_id = match data.get::<BotId>() {
        Some(id) => id,
        None => return,
    };

    if &message.author.id == bot_id {
        return;
    };

    let owner_ids = match data.get::<BotOwners>() {
        Some(o) => o,
        None => return,
    };

    for owner_id in owner_ids.iter() {
        if &message.author.id == owner_id {
            continue;
        }
        if let Ok(user) = owner_id.to_user(ctx).await {
            if let Ok(chan) = user.create_dm_channel(ctx).await {
                let _ = chan.say(&ctx.http, format!("DM from {}:\n{}", &message.author, &message.content));
            };
        };
    }
}

//Generic handling of common user errors.
#[hook]
pub async fn dispatch_error(context: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::NotEnoughArguments { min, given } => {
            let _ = msg
                .channel_id
                .say(
                    &context.http,
                    format!("Need {} arguments, but only got {}.", min, given),
                )
                .await;
        }
        DispatchError::TooManyArguments { max, given } => {
            let _ = msg
                .channel_id
                .say(
                    &context.http,
                    format!("Max arguments allowed is {}, but got {}.", max, given),
                )
                .await;
        }
        _ => error!("Unhandled dispatch error. {:?}", error),
    }
}

//Logs every command that errored, should only be used for bot failures and not user failures.
#[hook]
pub async fn after(_ctx: &Context, _msg: &Message, _cmd_name: &str, error: Result<(), CommandError>) {
    if let Err(why) = error {
        error!("{:?}", why);
    }
}