use crate::prelude::*;
use serenity::prelude::*;

use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
#[min_args(0)]
#[max_args(0)]
#[description("Checks if bot is online.")]
#[usage("ping")]
#[example("ping")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!("PING");
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}