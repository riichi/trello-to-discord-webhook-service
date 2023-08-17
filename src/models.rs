use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WebhookEvent {
    pub action: WebhookAction,
}

#[derive(Debug, Deserialize)]
pub struct WebhookAction {
    #[serde(alias = "type")]
    pub _type: String,
    pub date: DateTime<Local>,
    #[serde(alias = "memberCreator")]
    pub member_creator: Option<MemberCreator>,
    pub data: WebhookActionData,
    pub display: Option<WebhookActionDisplay>,
}

#[derive(Debug, Deserialize)]
pub struct MemberCreator {
    #[serde(alias = "fullName")]
    pub full_name: Option<String>,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhookActionData {
    pub card: Option<TrelloCard>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookActionDisplay {
    #[serde(alias = "translationKey")]
    pub translation_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrelloCard {
    pub name: String,
    #[serde(alias = "shortLink")]
    pub short_link: String,
}

#[derive(Serialize)]
pub struct DiscordWebhookEvent {
    pub embeds: Vec<DiscordEmbed>,
}

#[derive(Serialize)]
pub struct DiscordEmbed {
    pub title: Option<String>,
    #[serde(alias = "type")]
    pub _type: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: Option<u32>,
    pub fields: Vec<DiscordEmbedField>,
    pub timestamp: Option<DateTime<Local>>,
    pub author: Option<DiscordEmbedAuthor>,
}

#[derive(Serialize)]
pub struct DiscordEmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize)]
pub struct DiscordEmbedAuthor {
    pub name: String,
}
