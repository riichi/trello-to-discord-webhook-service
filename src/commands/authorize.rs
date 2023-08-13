use std::io::{self, Write};

use anyhow::Result;
use clap::{Args, ValueEnum};
use open;
use reqwest::Url;

use crate::config::Config;

const BASE_URL: &str = "https://trello.com/1/authorize";

#[derive(Clone, ValueEnum)]
pub enum Expiration {
    #[clap(name = "1hour")]
    OneHour,
    #[clap(name = "1day")]
    OneDay,
    #[clap(name = "30days")]
    ThirtyDays,
    #[clap(name = "never")]
    Never,
}

impl ToString for Expiration {
    fn to_string(&self) -> String {
        String::from(
            self.to_possible_value()
                .expect("Value should not be skipped")
                .get_name(),
        )
    }
}

#[derive(Args)]
pub struct AuthorizeArgs {
    #[arg(long)]
    pub name: String,
    #[arg(long, value_enum, default_value_t = Expiration::OneHour)]
    pub expiration: Expiration,
}

pub async fn main(args: AuthorizeArgs, config: &mut Config) -> Result<()> {
    let url = Url::parse_with_params(
        BASE_URL,
        &[
            ("scope", "read"),
            ("expiration", args.expiration.to_string().as_str()),
            ("name", args.name.as_str()),
            ("key", config.api.api_key.as_str()),
            ("response_type", "token"),
        ],
    )?;
    open::that(url.to_string())?;
    {
        let mut handle = io::stdout().lock();
        handle.write_all("Generated token: ".as_bytes())?;
        handle.flush()?;
    }
    let mut token = String::new();
    io::stdin().read_line(&mut token)?;
    config.api.api_token = Some(String::from(token.trim()));
    Ok(())
}
