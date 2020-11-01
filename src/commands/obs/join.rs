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


#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Not implemented yet!").await?;

    Ok(())
}