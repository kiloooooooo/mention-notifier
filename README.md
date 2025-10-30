# 実行方法

## ローカル環境での実行

1. リポジトリをクローンします。

  `$ git clone https://github.com/kiloooooooo/mention-notifier.git`

2. `.env`ファイルを作成します

  ```
  DISCORD_TOKEN="<your discord api token>"
  DISCORD_TARGET_USER_ID="<your discord user-id>"

  LINE_MESSAGING_API_URL="https://api.line.me/v2/bot/message/push"
  LINE_MESSAGING_API_TOKEN="<your line messaging api token>"
  LINE_USER_ID="<your line user id>"

  NOTIFICATION_MESSAGE_FORMAT="Discordメッセージ通知:\n\"{guild_name}\" であなたがメンションされました"
  ```
3. 必要な依存関係をインストールします。

  `$ cargo install`

4. サーバーを起動します。

  `$ cargo run --release`
