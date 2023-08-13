use std::fs;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{
    authorize::{main as authorize_main, AuthorizeArgs},
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
    /// (Interactively) get Trello credentials.
    Authorize(AuthorizeArgs),
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
    let mut config: Config = toml::from_str(&fs::read_to_string("config.toml")?)?;
    let cli = Cli::parse();
    match cli.command {
        Command::Authorize(args) => authorize_main(args, &mut config).await?,
        Command::CreateWebhook(args) => create_webhook_main(args, &mut config).await?,
        Command::GetBoards => get_boards_main(&mut config).await?,
        Command::GetWebhooks => get_webhooks_main(&mut config).await?,
        Command::StartWebhook => start_webhook_main(&mut config).await?,
    }
    fs::write("config.toml", toml::to_string(&config)?)?;
    Ok(())
}
