use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api: TrelloAPIConfig,
    pub trello: WebhookConfig,
    pub discord: DiscordConfig,
}

#[derive(Serialize, Deserialize)]
pub struct TrelloAPIConfig {
    pub api_key: String,
    pub api_secret: String,
    pub api_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct WebhookConfig {
    pub port: u16,
    pub callback_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordConfig {
    pub url: String,
}
