use anyhow::Result;
use clap::Args;
use reqwest::{Client, Url};

use crate::config::Config;

const BASE_URL: &str = "https://trello.com/1/webhooks";

#[derive(Args)]
pub struct CreateWebhookArgs {
    #[arg(long)]
    pub description: String,
    #[arg(long)]
    pub board_id: String,
    #[arg(long, default_value_t = false)]
    pub active: bool,
}

pub async fn main(args: CreateWebhookArgs, config: &Config) -> Result<()> {
    let token = config.api.api_token.as_ref().expect("API token missing");
    let url = Url::parse_with_params(
        BASE_URL,
        &[
            ("description", args.description.as_str()),
            ("callbackURL", config.trello.callback_url.as_str()),
            ("idModel", args.board_id.as_str()),
            ("active", if args.active { "true" } else { "false" }),
            ("key", config.api.api_key.as_str()),
            ("token", token.as_str()),
        ],
    )?;
    let client = Client::new();
    let response = client.post(url).send().await?.text().await?;
    println!("{}", response);
    Ok(())
}
