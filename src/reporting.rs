use anyhow::Result;
use reqwest::Client;
use tracing::debug;

use crate::models::{
    DiscordEmbed, DiscordEmbedAuthor, DiscordWebhookEvent, MemberCreator, WebhookActionDisplay,
    WebhookEvent,
};

const COLOR_GREEN: u32 = 0x00ff00;
const TYPE_RICH: &str = "rich";
const ACTIONS_MOVE_CARD: [&str; 2] = ["action_moved_card_higher", "action_moved_card_lower"];

#[derive(Debug)]
pub struct DiscordReporter {
    endpoint: String,
    client: Client,
}

impl DiscordReporter {
    #[must_use]
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: Client::new(),
        }
    }

    pub async fn report(&self, event: WebhookEvent) -> Result<()> {
        if Self::should_skip(&event) {
            debug!("Skipping event {:?}", event);
            return Ok(());
        }
        let mut card_name: Option<String> = None;
        let mut card_url: Option<String> = None;
        if let Some(card) = event.action.data.card {
            card_name = Some(card.name);
            card_url = Some(format!("https://trello.com/c/{}", card.short_link));
        }
        let discord_event = DiscordWebhookEvent {
            embeds: vec![DiscordEmbed {
                title: Some(event.action._type),
                _type: Some(String::from(TYPE_RICH)),
                description: card_name,
                url: card_url,
                color: Some(COLOR_GREEN),
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

    #[must_use]
    fn should_skip(event: &WebhookEvent) -> bool {
        event
            .action
            .display
            .as_ref()
            .is_some_and(Self::should_skip_action_display)
    }

    #[must_use]
    fn should_skip_action_display(action_display: &WebhookActionDisplay) -> bool {
        action_display
            .translation_key
            .as_ref()
            .is_some_and(|s| Self::should_skip_action_translation_key(s))
    }

    #[must_use]
    fn should_skip_action_translation_key(translation_key: &str) -> bool {
        ACTIONS_MOVE_CARD.contains(&translation_key)
    }

    #[must_use]
    fn get_author(creator: Option<MemberCreator>) -> Option<DiscordEmbedAuthor> {
        creator.map(|creator| DiscordEmbedAuthor {
            name: creator.full_name.unwrap_or(creator.username),
        })
    }
}
