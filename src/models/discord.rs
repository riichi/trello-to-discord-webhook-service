use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Serialize)]
pub struct WebhookEvent {
    pub embeds: Vec<Embed>,
}

#[derive(Serialize)]
pub struct Embed {
    pub title: Option<String>,
    #[serde(alias = "type")]
    pub _type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: Option<u32>,
    pub fields: Vec<EmbedField>,
    pub timestamp: Option<DateTime<Local>>,
    pub author: Option<EmbedAuthor>,
}

#[derive(Serialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize)]
pub struct EmbedAuthor {
    pub name: String,
}
