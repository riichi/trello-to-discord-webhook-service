use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{
    self,
    extract::State,
    response::{IntoResponse, Response},
    routing::{head, post},
    Router, Server,
};
use base64::{engine::general_purpose::STANDARD as Base64, Engine};
use hmac::{Hmac, Mac};
use hyper::{body::Bytes, HeaderMap};
use reqwest::{Client, StatusCode};
use sha1::Sha1;

use crate::{
    config::Config,
    models::{DiscordEmbed, DiscordEmbedAuthor, DiscordWebhookEvent, MemberCreator, WebhookEvent},
};

struct DiscordReporter {
    endpoint: String,
    client: Client,
}

impl DiscordReporter {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: Client::new(),
        }
    }

    pub async fn report(&self, event: WebhookEvent) -> Result<()> {
        let mut card_name: Option<String> = None;
        let mut card_url: Option<String> = None;
        if let Some(card) = event.action.data.card {
            card_name = Some(card.name);
            card_url = Some(format!("https://trello.com/c/{}", card.short_link));
        }
        let discord_event = DiscordWebhookEvent {
            embeds: vec![DiscordEmbed {
                title: Some(event.action._type),
                _type: Some(String::from("rich")),
                description: card_name,
                url: card_url,
                color: Some(0x00ff00),
                fields: vec![],
                timestamp: Some(event.action.date),
                author: Self::get_author(event.action.member_creator),
            }],
        };
        self.client
            .post(&self.endpoint)
            .json(&discord_event)
            .send()
            .await?;
        Ok(())
    }

    fn get_author(creator: Option<MemberCreator>) -> Option<DiscordEmbedAuthor> {
        creator.map(|creator| DiscordEmbedAuthor {
            name: creator.full_name.unwrap_or(creator.username),
        })
    }
}

struct WebhookState {
    pub reporter: DiscordReporter,
    pub trello_api_secret: String,
    pub webhook_url: String,
}

pub async fn main(config: &Config) -> Result<()> {
    let app = Router::new()
        .route("/", post(post_endpoint))
        .route("/", head(head_endpoint))
        .with_state(Arc::new(WebhookState {
            reporter: DiscordReporter::new(config.discord.url.clone()),
            trello_api_secret: config.api.secret.clone(),
            webhook_url: config.webhook.url.clone(),
        }));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.webhook.port));
    Ok(Server::bind(&addr).serve(app.into_make_service()).await?)
}

async fn post_endpoint(
    State(state): State<Arc<WebhookState>>,
    headers: HeaderMap,
    raw_body: Bytes,
) -> Result<(), Response> {
    check_signature(
        &state.trello_api_secret,
        &state.webhook_url,
        &raw_body,
        &headers,
    )?;

    let event: WebhookEvent = serde_json::from_slice(&raw_body).map_err(|e| {
        println!("Could not parse payload: {}", e);
        (StatusCode::BAD_REQUEST, "Could not parse payload").into_response()
    })?;
    println!(
        "New event: {} (date: {})",
        event.action._type, event.action.date
    );
    state.reporter.report(event).await.map_err(|e| {
        println!("Internal server error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
    })
}

fn check_signature(
    trello_api_secret: &str,
    webhook_url: &str,
    request_body: &Bytes,
    headers: &HeaderMap,
) -> Result<(), Response> {
    let token = headers
        .get("x-trello-webhook")
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing signature").into_response())?
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid HMAC signature format").into_response())?;

    type HmacSha1 = Hmac<Sha1>;

    let mut hmac = HmacSha1::new_from_slice(trello_api_secret.as_bytes()).unwrap();
    hmac.update(request_body);
    hmac.update(webhook_url.as_bytes());
    let result = hmac.finalize().into_bytes();
    let digest = Base64.encode(result);

    if digest != token {
        return Err((StatusCode::UNAUTHORIZED, "Invalid HMAC signature").into_response());
    }
    Ok(())
}

async fn head_endpoint() {}
