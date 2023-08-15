use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api: TrelloAPIConfig,
    pub webhook: WebhookConfig,
    pub discord: DiscordConfig,
}

#[derive(Serialize, Deserialize)]
pub struct TrelloAPIConfig {
    pub key: String,
    pub secret: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebhookConfig {
    pub port: u16,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordConfig {
    pub url: String,
}
