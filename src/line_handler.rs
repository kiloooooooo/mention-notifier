use anyhow::Result;
use log::info;

/// LINE Messaging APIを使用して通知を送るハンドラ
pub struct LineHandler {
    messaging_api_url: String,
    token: String,
    target_user_id: String,
}

impl LineHandler {
    /// LineHandlerの新しいインスタンスを作成する
    pub fn new(messaging_api_url: String, token: String, target_user_id: String) -> Self {
        Self {
            messaging_api_url,
            token,
            target_user_id,
        }
    }

    /// LINEに通知を送る
    pub async fn send_notification(&self, message: &str) -> Result<()> {
        let body = serde_json::json!({
            "to": self.target_user_id,
            "messages": [{
                "type": "text",
                "text": message,
            }]
        });
        let client = reqwest::Client::new();
        let response = client
            .post(self.messaging_api_url.clone())
            .header("Content-Type", "application/json")
            .bearer_auth(self.token.clone())
            .body(body.to_string())
            .send()
            .await?
            .text()
            .await?;

        info!("Notification sent: {}", response);

        Ok(())
    }
}
