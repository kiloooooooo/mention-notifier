use log::{debug, info};

use regex::Regex;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

/// メンションを受け取ったときのコールバック
pub trait MentionMonitorCallback {
    /// メンションを受け取ったときに呼び出される
    fn on_mention(
        &self,
        ctx: Context,
        message: &Message,
    ) -> impl std::future::Future<Output = ()> + std::marker::Send;
}

/// DiscordMentionMonitorのビルダー
pub struct DiscordMentionMonitorBuilder<C>
where
    C: MentionMonitorCallback + Sync + Send + 'static,
{
    target_user_id: String,
    callback: Option<C>,
}

/// Discordのメンションを監視する構造体
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
    /// コールバックを設定する
    pub fn mention_callback(&self, callback: C) -> Self {
        DiscordMentionMonitorBuilder {
            target_user_id: self.target_user_id.clone(),
            callback: Some(callback),
        }
    }



    /// DiscordMentionMonitorをビルドする
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
    /// ビルダーを作成する
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
    /// メッセージを受け取ったときの処理
    async fn message(&self, ctx: Context, msg: Message) {
        debug!("Received message: {}", msg.content);

        // ターゲットユーザーへのメンションが含まれているか確認
        let regex = Regex::new(format!("<@{}>", self.target_user_id).as_str()).unwrap();
        if !regex.is_match(&msg.content) {
            return;
        }

        info!(
            "Received mention from user {}, content: {}",
            msg.author.name, msg.content
        );

        // コールバックを呼び出す
        self.callback.on_mention(ctx, &msg).await;
    }

    /// 準備完了時の処理
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected Discord server as {}", ready.user.name);
    }
}
