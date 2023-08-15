use std::str::FromStr;

use anyhow::Result;
use serde::Deserialize;
use tracing::Level;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub trello: TrelloAPIConfig,
    pub webhook: WebhookConfig,
    pub discord: DiscordConfig,
    pub tracing: TracingConfig,
}

#[derive(Debug, Deserialize)]
pub struct TrelloAPIConfig {
    pub key: String,
    pub secret: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhookConfig {
    pub port: u16,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscordConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TracingConfig {
    pub level: Option<String>,
}

impl TracingConfig {
    pub fn try_parse_level(&self) -> Result<Option<Level>> {
        Ok(self
            .level
            .as_ref()
            .map(|v| Level::from_str(v))
            .transpose()?)
    }
}
