use serenity::{async_trait, model::prelude::*, prelude::*};

// pub mod cache_ready;
//pub mod category_create;
//pub mod category_delete;
//pub mod channel_create;
//pub mod channel_delete;
//pub mod guild_ban_addition;
//pub mod guild_member_addition;
//pub mod guild_member_removal;
//pub mod guild_member_update;
pub mod message;
//pub mod message_delete;
//pub mod message_update;
//pub mod presence_update;
pub mod ready;
pub mod voice_state_update;


pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, new_message: Message) {
        message::message(new_message).await
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await
    }

    async fn voice_state_update(&self, ctx: Context, gid: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) {
        voice_state_update::voice_state_update(ctx, gid, old, new).await
    }
}
