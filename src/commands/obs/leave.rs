use serenity::model::prelude::*;
use serenity::{
    async_trait,
    framework::{
        StandardFramework,
        standard::{macros::command, Args, CommandResult},
    },
    prelude::*,
    model::{
        channel::Message,
        gateway::Ready,
        id::ChannelId,
        misc::Mentionable
    },
    Result as SerenityResult,
};

use std::env;

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Not implemented yet!").await?;

    Ok(())
}