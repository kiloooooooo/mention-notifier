use log::{debug, info};

use regex::Regex;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub trait MentionMonitorCallback {
    fn on_mention(
        &self,
        ctx: Context,
        message: &Message,
    ) -> impl std::future::Future<Output = ()> + std::marker::Send;
}

pub struct DiscordMentionMonitorBuilder<C>
where
    C: MentionMonitorCallback + Sync + Send + 'static,
{
    target_user_id: String,
    callback: Option<C>,
}

pub struct DiscordMentionMonitor<C>
where
    C: MentionMonitorCallback + Sync + Send + 'static,
{
    target_user_id: String,
    callback: C,
}

impl<C> DiscordMentionMonitorBuilder<C>
where
    C: MentionMonitorCallback + Sync + Send + 'static,
{
    pub fn mention_callback(&self, callback: C) -> Self {
        DiscordMentionMonitorBuilder {
            target_user_id: self.target_user_id.clone(),
            callback: Some(callback),
        }
    }

    pub fn build(self) -> Result<DiscordMentionMonitor<C>, String> {
        match self.callback {
            Some(callback) => Ok(DiscordMentionMonitor {
                target_user_id: self.target_user_id,
                callback,
            }),
            None => Err("Callback not provided".to_string()),
        }
    }
}

impl<C> DiscordMentionMonitor<C>
where
    C: MentionMonitorCallback + Sync + Send + 'static,
{
    pub fn builder(target_user_id: String) -> DiscordMentionMonitorBuilder<C>
    where
        C: MentionMonitorCallback + Sync + Send + 'static,
    {
        DiscordMentionMonitorBuilder {
            target_user_id,
            callback: None,
        }
    }
}

#[async_trait]
impl<C> EventHandler for DiscordMentionMonitor<C>
where
    C: MentionMonitorCallback + Sync + Send + 'static,
{
    async fn message(&self, ctx: Context, msg: Message) {
        debug!("Received message: {}", msg.content);

        let regex = Regex::new(format!("<@{}>", self.target_user_id).as_str()).unwrap();
        if !regex.is_match(&msg.content) {
            return;
        }

        info!(
            "Received mention from user {}, content: {}",
            msg.author.name, msg.content
        );

        self.callback.on_mention(ctx, &msg).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected Discord server as {}", ready.user.name);
    }
}
