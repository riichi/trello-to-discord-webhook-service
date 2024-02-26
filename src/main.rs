use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::create_webhook::{main as create_webhook_main, CreateWebhookArgs};
use commands::get_boards::main as get_boards_main;
use commands::get_webhooks::main as get_webhooks_main;
use commands::start_webhook::main as start_webhook_main;
use tracing::debug;

use crate::config::Config;

mod commands;
mod config;
mod models;
mod reporting;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create Trello webhook.
    CreateWebhook(CreateWebhookArgs),
    /// List available Trello boards.
    GetBoards,
    /// List created Trello webhooks.
    GetWebhooks,
    /// Start the webhook service.
    StartWebhook,
}

fn get_config() -> Result<Config> {
    Ok(::config::Config::builder()
        .add_source(::config::File::with_name("config"))
        .add_source(::config::Environment::default().separator("_"))
        .build()?
        .try_deserialize()?)
}

fn configure_tracing(config: &Config) -> Result<()> {
    let level = config.tracing.try_parse_level()?;
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_max_level(level)
        .init();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_config()?;
    configure_tracing(&config)?;
    debug!("Config: {:#?}", config);
    let cli = Cli::parse();
    debug!("CLI: {:#?}", cli);
    match cli.command {
        Command::CreateWebhook(args) => create_webhook_main(args, &config).await?,
        Command::GetBoards => get_boards_main(&config).await?,
        Command::GetWebhooks => get_webhooks_main(&config).await?,
        Command::StartWebhook => start_webhook_main(&config).await?,
    }
    Ok(())
}
