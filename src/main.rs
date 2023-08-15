use std::fs;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{
    create_webhook::{main as create_webhook_main, CreateWebhookArgs},
    get_boards::main as get_boards_main,
    get_webhooks::main as get_webhooks_main,
    start_webhook::main as start_webhook_main,
};
use config::Config;

mod commands;
mod config;
mod models;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
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

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = toml::from_str(&fs::read_to_string("config.toml")?)?;
    let cli = Cli::parse();
    match cli.command {
        Command::CreateWebhook(args) => create_webhook_main(args, &config).await?,
        Command::GetBoards => get_boards_main(&config).await?,
        Command::GetWebhooks => get_webhooks_main(&config).await?,
        Command::StartWebhook => start_webhook_main(&config).await?,
    }
    Ok(())
}
