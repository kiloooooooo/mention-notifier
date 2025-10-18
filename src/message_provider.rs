use std::env;

use anyhow::Result;
use serenity::{model::channel::Message, prelude::*};

pub struct MessageProvider;

impl MessageProvider {
    pub async fn build_message(ctx: &Context, discord_message: &Message) -> Result<String> {
        let guild_name = Self::get_guild_name_http(ctx, discord_message).await;
        let message_format = env::var("NOTIFICATION_MESSAGE_FORMAT")?;

        Ok(message_format.replace("{guild_name}", guild_name.unwrap().as_str()))
    }

    async fn get_guild_name_http(ctx: &Context, msg: &Message) -> Option<String> {
        match msg.guild_id {
            Some(guild_id) => Some(
                guild_id
                    .to_partial_guild(&ctx.http)
                    .await
                    .map_or("N/A".to_string(), |guild| guild.name),
            ),
            None => None,
        }
    }
}
