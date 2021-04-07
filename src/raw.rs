use serde_json;

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Hangouts {
    pub conversations: Vec<Conversation>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Conversation {
    #[serde(rename="conversation")] pub header: ConversationHeader,
    pub events: Vec<Event>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct ConversationHeader {
    pub conversation_id: ConversationId,
    #[serde(rename="conversation")] pub details: ConversationDetails,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct ConversationDetails {
    pub id: ConversationId,
    #[serde(rename="type")] pub typ: String,
    pub name: Option<String>, // set for type="GROUP" only
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
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct ConversationId {
    pub id: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct SelfConversationState {
    pub self_read_state: ReadState,
    pub status: String,
    pub notification_level: NotificationLevel,
    pub view: Vec<String>,
    pub inviter_id: ParticipantId,
    pub invite_timestamp: String,
    pub invitation_display_type: Option<String>,
    pub invite_affinity: Option<String>,
    pub sort_timestamp: String,
    pub active_timestamp: Option<String>,
    pub delivery_medium_option: Option<serde_json::Value>, // TODO
    pub is_guest: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct ReadState {
    pub participant_id: ParticipantId,
    pub latest_read_timestamp: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct ParticipantId {
    pub gaia_id: String,
    pub chat_id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct ParticipantData {
    pub id: ParticipantId,
    pub fallback_name: Option<String>,
    pub invitation_status: Option<String>,
    pub participant_type: Option<String>,
    pub new_invitation_status: Option<String>,
    pub in_different_customer_as_requester: Option<bool>,
    pub domain_id: Option<String>,
    pub phone_number: Option<serde_json::Value>, // TODO
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Event {
    #[serde(flatten)] pub header: EventHeader,
    #[serde(flatten)] pub data: EventData,
    pub event_type: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct SelfEventState {
    pub user_id: ParticipantId,
    pub client_generated_id: Option<String>,
    pub notification_level: Option<NotificationLevel>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub enum NotificationLevel {
    #[serde(rename="QUIET")] Quiet,
    #[serde(rename="RING")] Ring,
}

#[derive(Deserialize, Debug)]
//#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
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

    #[serde(rename="membership_change")]
    MembershipChange {
        #[serde(rename="type")] typ: String,
        participant_id: Vec<ParticipantId>,
    }
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct ChatSegments {
    #[serde(default, rename="segment")] pub segments: Vec<ChatSegment>,
    #[serde(default, rename="attachment")] pub attachments: Vec<AttachmentSegment>,
}

#[derive(Deserialize, Debug)]
#[serde(tag="type")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
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
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct LinkData {
    pub link_target: String,
    pub display_url: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Formatting {
    #[serde(default)] pub bold: bool,
    #[serde(default)] pub italics: bool,
    #[serde(default)] pub strikethrough: bool,
    #[serde(default)] pub underline: bool,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct AttachmentSegment {
    pub embed_item: EmbedItem,
    pub id: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct EmbedItem {
    pub id: Option<String>,
    pub plus_photo: Option<PlusPhoto>,
    pub place_v2: Option<PlaceV2>,
    pub thing_v2: Option<ThingV2>,
    #[serde(rename="type")] pub types: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct PlusPhoto {
    pub album_id: String,
    pub media_type: String,
    pub original_content_url: String,
    pub owner_obfuscated_id: String,
    pub photo_id: String,
    pub stream_id: Vec<String>,
    pub thumbnail: Thumbnail,
    pub url: String,
    pub download_url: Option<String>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Thumbnail {
    pub height_px: u64,
    pub width_px: u64,
    pub image_url: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct PlaceV2 {
    pub url: String,
    pub name: Option<String>,
    pub address: Address,
    pub geo: Geo,
    pub representative_image: RepresentativeImage,
    pub place_id: Option<String>,
    pub cluster_id: Option<String>,
    pub reference_id: Option<String>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Address {
    #[serde(rename="type", default)] pub types: Vec<String>,
    pub postal_address_v2: PostalAddressV2,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct PostalAddressV2 {
    pub name: Option<String>,
    pub street_address: Option<String>,
    pub address_locality: Option<String>,
    pub address_region: Option<String>,
    pub address_country: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Geo {
    #[serde(rename="type", default)] pub types: Vec<String>,
    pub geo_coordinates_v2: GeoCoordinatesV2,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct GeoCoordinatesV2 {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct RepresentativeImage {
    #[serde(rename="type")] pub types: Vec<String>,
    pub id: String,
    pub image_object_v2: ImageObjectV2,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct ImageObjectV2 {
    pub url: String,
    pub width: Option<String>,
    pub height: Option<String>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct ThingV2 {
    pub url: String,
    pub name: Option<String>,
    pub representative_image: RepresentativeImage,
}

#[derive(Deserialize, Debug)]
#[serde(tag="event_type")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub enum HangoutEvent {
    #[serde(rename="START_HANGOUT")] StartHangout,
    #[serde(rename="END_HANGOUT")] EndHangout {
        hangout_duration_secs: String,
    },
}
