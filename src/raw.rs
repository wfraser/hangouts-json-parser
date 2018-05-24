use serde_json;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Hangouts {
    pub conversations: Vec<Conversation>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Conversation {
    #[serde(rename="conversation")] pub header: ConversationHeader,
    pub events: Vec<Event>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConversationHeader {
    pub conversation_id: ConversationId,
    #[serde(rename="conversation")] pub details: ConversationDetails,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConversationDetails {
    pub id: ConversationId,
    #[serde(rename="type")] pub typ: String,
    pub self_conversation_state: SelfConversationState,
    pub read_state: Vec<ReadState>,
    pub has_active_hangout: bool,
    pub otr_status: String,
    pub otr_toggle: String,
    pub current_participant: Vec<ParticipantId>,
    pub participant_data: Vec<ParticipantData>,
    pub fork_on_external_invite: bool,
    pub network_type: Vec<String>,
    pub force_history_state: String,
    pub group_link_sharing_status: String
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ConversationId {
    pub id: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SelfConversationState {
    pub self_read_state: ReadState,
    pub status: String,
    pub notification_level: String,
    pub view: Vec<String>,
    pub inviter_id: ParticipantId,
    pub invite_timestamp: String,
    pub invitation_display_type: Option<String>,
    pub invite_affinity: Option<String>,
    pub sort_timestamp: String,
    pub active_timestamp: Option<String>,
    pub delivery_medium_option: Vec<serde_json::Value>, // TODO
    pub is_guest: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ReadState {
    pub participant_id: ParticipantId,
    pub latest_read_timestamp: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(deny_unknown_fields)]
pub struct ParticipantId {
    pub gaia_id: String,
    pub chat_id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ParticipantData {
    pub id: ParticipantId,
    pub fallback_name: String,
    pub invitation_status: String,
    pub participant_type: String,
    pub new_invitation_status: String,
    pub in_different_customer_as_requester: bool,
    pub domain_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Event {
    #[serde(flatten)] pub header: EventHeader,
    #[serde(flatten)] pub data: EventData,
    pub event_type: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EventHeader {
    pub conversation_id: ConversationId,
    pub sender_id: ParticipantId,
    pub timestamp: String,
    pub self_event_state: SelfEventState,
    pub event_id: String,
    pub advances_sort_timestamp: bool,
    pub event_otr: String,
    pub delivery_medium: serde_json::Value, // TODO
    pub event_version: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SelfEventState {
    pub user_id: ParticipantId,
    pub client_generated_id: Option<String>,
    pub notification_level: String,
}

#[derive(Deserialize, Debug)]
//#[serde(deny_unknown_fields)]
pub enum EventData {
    #[serde(rename="chat_message")]
    ChatMessage {
        message_content: ChatSegments,
        annotation: Option<Vec<Annotation>>,
    },

    #[serde(rename="hangout_event")]
    HangoutEvent {
        #[serde(flatten)] data: HangoutEvent,
        media_type: String,
        participant_id: Vec<ParticipantId>,
    },
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ChatSegments {
    #[serde(default, rename="segment")] pub segments: Vec<ChatSegment>,
    #[serde(default, rename="attachment")] pub attachments: Vec<AttachmentSegment>,
}

#[derive(Deserialize, Debug)]
#[serde(tag="type")]
#[serde(deny_unknown_fields)]
pub enum ChatSegment {
    #[serde(rename="TEXT")]
    Text {
        text: String,
        #[serde(default)] formatting: Formatting,
    },

    #[serde(rename="LINK")]
    Link {
        text: String,
        link_data: LinkData,
        #[serde(default)] formatting: Formatting,
    },

    #[serde(rename="LINE_BREAK")]
    LineBreak {
        text: Option<String>,
    },
}

#[derive(Deserialize, Debug)]
pub struct Annotation {
    #[serde(rename="type")] pub typ: i32,
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LinkData {
    pub link_target: String,
    pub display_url: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Formatting {
    #[serde(default)] pub bold: bool,
    #[serde(default)] pub italics: bool,
    #[serde(default)] pub strikethrough: bool,
    #[serde(default)] pub underline: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AttachmentSegment {
    pub embed_item: serde_json::Value, // TODO
    pub id: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag="event_type")]
#[serde(deny_unknown_fields)]
pub enum HangoutEvent {
    #[serde(rename="START_HANGOUT")] StartHangout,
    #[serde(rename="END_HANGOUT")] EndHangout {
        hangout_duration_secs: String,
    },
}
