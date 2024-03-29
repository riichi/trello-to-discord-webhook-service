use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{head, post};
use axum::{self, Router};
use base64::engine::general_purpose::STANDARD as Base64;
use base64::Engine;
use hmac::{Hmac, Mac};
use hyper::body::Bytes;
use hyper::HeaderMap;
use sha1::Sha1;
use tower_http::trace::TraceLayer;
use tracing::{debug, warn};

use crate::config::Config;
use crate::models::trello_webhook::Event;
use crate::reporting::DiscordReporter;

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
            trello_api_secret: config.trello.secret.clone(),
            webhook_url: config.webhook.url.clone(),
        }))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.webhook.port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    Ok(axum::serve(listener, app.into_make_service()).await?)
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

    let event: Event = serde_json::from_slice(&raw_body).map_err(|e| {
        warn!("Could not parse payload: {}", e);
        (StatusCode::BAD_REQUEST, "Could not parse payload").into_response()
    })?;
    debug!("New event: {:?}", raw_body);
    state.reporter.report(event).await.map_err(|e| {
        warn!("Internal server error: {}", e);
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
