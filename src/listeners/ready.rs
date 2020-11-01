use crate::prelude::*;
use log::info;
use serenity::{model::prelude::*, prelude::*};

pub async fn ready(ctx: Context, ready: Ready) {
    info!("Connected as {}", ready.user.name);
}