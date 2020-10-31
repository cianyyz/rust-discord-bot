use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!("PING");
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}