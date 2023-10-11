use anyhow::Result;
use reqwest::Client;
use tracing::{debug, warn};

use crate::models::{
    discord::{
        Embed as DiscordEmbed, EmbedAuthor as DiscordEmbedAuthor,
        WebhookEvent as DiscordWebhookEvent,
    },
    trello_webhook::{Event, MemberCreator},
};

const COLOR_GREEN: u32 = 0x00ff00;
const TYPE_RICH: &str = "rich";
const ACTIONS_TO_IGNORE: [&str; 6] = [
    "action_moved_card_higher",
    "action_moved_card_lower",
    "action_moved_list_left",
    "action_moved_list_right",
    "action_renamed_checkitem",
    "action_renamed_checklist",
];
const TRELLO_ROOT_URL: &str = "https://trello.com";

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

    pub async fn report(&self, event: Event) -> Result<()> {
        if Self::should_skip_event(&event) {
            debug!("Skipping event {:?}", event);
            return Ok(());
        }
        let mut card_name: Option<String> = None;
        let mut card_url: Option<String> = None;
        if let Some(card) = &event.action.data.card {
            card_name = Some(card.name.clone().unwrap_or(String::from("**Unknown**")));
            card_url = Some(format!("{}/c/{}", TRELLO_ROOT_URL, card.short_link));
        }
        let discord_event = DiscordWebhookEvent {
            embeds: vec![DiscordEmbed {
                title: Some(String::from(Self::get_embed_title(&event))),
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
    fn should_skip_event(event: &Event) -> bool {
        match Self::get_action_name_translation_key(event) {
            Some(key) => Self::should_skip_action_translation_key(key),
            None => false,
        }
    }

    #[must_use]
    fn get_embed_title(event: &Event) -> &str {
        Self::get_action_name_translation_key(event)
            .map(|x| x.as_str())
            .and_then(Self::get_action_display_name)
            .unwrap_or_else(|| Self::get_action_type_display_name(&event.action._type))
    }

    #[must_use]
    fn get_action_display_name(translation_key: &str) -> Option<&str> {
        match translation_key {
            "action_added_list_to_board" => Some("List added"),
            "action_add_label_to_card" => Some("Label added to a card"),
            "action_archived_card" => Some("Card archived"),
            "action_archived_list" => Some("List archived"),
            "action_changed_description_of_card" => Some("Card description changed"),
            "action_comment_on_card" => Some("Comment added to a card"),
            "action_create_card" => Some("Card created"),
            "action_delete_card" => Some("Card deleted"),
            "action_move_card_from_list_to_list" => Some("Card moved"),
            "action_renamed_card" => Some("Card renamed"),
            "action_member_joined_card" => Some("Member added to a card"),
            "action_member_left_card" => Some("Member removed from a card"),
            "action_completed_checkitem" => Some("Checklist item completed"),
            "action_marked_checkitem_incomplete" => Some("Checklist item marked incomplete"),
            "action_invited_an_unconfirmed_member_to_board" => Some("Invited an unconfirmed member to board"),
            _ => {
                warn!("Unknown action name translation key: {}", translation_key);
                None
            }
        }
    }

    #[must_use]
    fn get_action_type_display_name(action_type: &str) -> &str {
        match action_type {
            "addAttachmentToCard" => "Attachment added to a card",
            "addChecklistToCard" => "Checklist added to a card",
            "addMemberToBoard" => "Added member to the board",
            "addMemberToCard" => "Added member to a card",
            "copyCard" => "Card copied",
            "createCard" => "Card created",
            "deleteBoardInvitation" => "Board invitation deleted",
            "deleteComment" => "Comment removed from a card",
            "removeChecklistFromCard" => "Checklist removed from a card",
            "updateBoard" => "Board updated",
            "updateCard" => "Card updated",
            "updateCheckItemStateOnCard" => "Checklist updated",
            "updateChecklist" => "Checklist updated",
            "updateList" => "List updated",
            _ => action_type,
        }
    }

    #[must_use]
    fn get_action_name_translation_key(event: &Event) -> Option<&String> {
        match &event.action.display {
            Some(display) => display.translation_key.as_ref(),
            None => None,
        }
    }

    #[must_use]
    fn should_skip_action_translation_key(translation_key: &str) -> bool {
        ACTIONS_TO_IGNORE.contains(&translation_key)
    }

    #[must_use]
    fn get_author(creator: Option<MemberCreator>) -> Option<DiscordEmbedAuthor> {
        creator.map(|creator| DiscordEmbedAuthor {
            name: creator.full_name.unwrap_or(creator.username),
        })
    }
}
