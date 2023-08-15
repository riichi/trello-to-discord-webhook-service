use std::io;

use anyhow::Result;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::config::Config;

const BASE_URL: &str = "https://trello.com/1/members/me/boards";

#[derive(Serialize, Deserialize)]
struct Board {
    id: String,
    name: String,
    desc: String,
    #[serde(alias = "descData")]
    desc_data: Option<String>,
    closed: bool,
    #[serde(alias = "idMemberCreator")]
    id_member_creator: String,
    #[serde(alias = "idOrganization")]
    id_organization: String,
    pinned: bool,
    url: String,
    #[serde(alias = "shortUrl")]
    short_url: String,
    starred: bool,
}

pub async fn main(config: &Config) -> Result<()> {
    let token = config.api.api_token.as_ref().expect("API token missing");
    let url = Url::parse_with_params(BASE_URL, &[("key", &config.api.api_key), ("token", token)])?;
    let response = reqwest::get(url).await?;
    let response: Vec<Board> = response.json().await?;
    serde_json::to_writer_pretty(io::stdout().lock(), &response)?;
    Ok(())
}
