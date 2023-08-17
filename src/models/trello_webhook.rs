use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Event {
    pub action: Action,
}

#[derive(Debug, Deserialize)]
pub struct Action {
    #[serde(alias = "type")]
    pub _type: String,
    pub date: DateTime<Local>,
    #[serde(alias = "memberCreator")]
    pub member_creator: Option<MemberCreator>,
    pub data: ActionData,
    pub display: Option<ActionDisplay>,
}

#[derive(Debug, Deserialize)]
pub struct MemberCreator {
    #[serde(alias = "fullName")]
    pub full_name: Option<String>,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct ActionData {
    pub card: Option<TrelloCard>,
}

#[derive(Debug, Deserialize)]
pub struct ActionDisplay {
    #[serde(alias = "translationKey")]
    pub translation_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrelloCard {
    pub name: Option<String>,
    #[serde(alias = "shortLink")]
    pub short_link: String,
}
