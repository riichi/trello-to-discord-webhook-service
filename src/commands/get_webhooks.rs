use std::io;

use anyhow::Result;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::config::Config;

const BASE_URL: &str = "https://trello.com/1/tokens";

#[derive(Serialize, Deserialize)]
struct Webhook {
    pub id: String,
    pub description: String,
    #[serde(alias = "idModel")]
    pub id_model: String,
    #[serde(alias = "callbackURL")]
    pub callback_url: String,
    pub active: bool,
    #[serde(alias = "consecutiveFailures")]
    pub consecutive_failures: u32,
    #[serde(alias = "firstConsecutiveFailDate")]
    pub first_consecutive_fail_date: Option<String>,
}

pub async fn main(config: &Config) -> Result<()> {
    let base_url = format!("{}/{}/webhooks", BASE_URL, config.trello.token);
    let url = Url::parse_with_params(
        &base_url,
        &[("key", &config.trello.key), ("token", &config.trello.token)],
    )?;
    let response = reqwest::get(url).await?;
    let response: Vec<Webhook> = response.json().await?;
    serde_json::to_writer_pretty(io::stdout().lock(), &response)?;
    Ok(())
}
