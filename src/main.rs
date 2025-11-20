use std::env;

use log::error;
use serenity::{model::channel::Message, prelude::*};

use crate::{
    discord_mention_monitor::{DiscordMentionMonitor, MentionMonitorCallback},
    line_handler::LineHandler,
    message_provider::MessageProvider,
};

mod discord_mention_monitor;
mod line_handler;
mod message_provider;

impl MentionMonitorCallback for LineHandler {
    /// メンションを受け取ったときの処理
    ///
    /// メッセージを構築し、LINEに通知を送る
    async fn on_mention(&self, ctx: Context, message: &Message) {
        let out_message = match MessageProvider::build_message(&ctx, message).await {
            Ok(message) => message,
            Err(why) => {
                error!("Failed to build message: {:?}", why);
                return;
            }
        };

        if let Err(why) = self.send_notification(out_message.as_str()).await {
            error!("Failed to send notification: {:?}", why);
        }
    }
}

#[tokio::main]
async fn main() {
    // ロガーの初期化
    env_logger::init();
    // .envファイルから環境変数を読み込む
    dotenvy::dotenv().expect("Failed to load .env file!");

    // 環境変数から設定を読み込む
    let discord_token =
        env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN exists in the environment");
    let discord_target_user_id = env::var("DISCORD_TARGET_USER_ID")
        .expect("Expected DISCORD_TARGET_USER_ID exists in the environment");
    let line_messaging_api_url = env::var("LINE_MESSAGING_API_URL")
        .expect("Expected LINE_MESSAGING_API_URL exists in the environment");
    let line_token = env::var("LINE_MESSAGING_API_TOKEN")
        .expect("Expected LINE_MESSAGING_API_TOKEN exists in the environment");
    let line_user_id =
        env::var("LINE_USER_ID").expect("Expected LINE_USER_ID exists in the environment");

    // DiscordのGateway Intentsを設定
    // メッセージの内容とギルドのメッセージイベントを受け取る
    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    // LineHandlerとDiscordMentionMonitorの初期化
    let line_handler = LineHandler::new(line_messaging_api_url, line_token, line_user_id);
    let mention_monitor = DiscordMentionMonitor::builder(discord_target_user_id)
        .mention_callback(line_handler)
        .build()
        .expect("Error creating mention_monitor");
    let mut client = Client::builder(&discord_token, intents)
        .event_handler(mention_monitor)
        .await
        .expect("Error creating client");

    // クライアントを開始
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
