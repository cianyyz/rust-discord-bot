  
use crate::prelude::*;
use serenity::{model::prelude::*, prelude::*};


pub async fn voice_state_update(ctx: Context, gid: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) {
    let gid = match gid {
        Some(id) => id,
        None => return,
    };


}