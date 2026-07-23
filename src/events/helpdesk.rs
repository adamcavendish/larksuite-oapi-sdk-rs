//! Helpdesk v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomizedFieldDisplayItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct I18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Comments {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TicketUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TicketUserEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TicketEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest: Option<TicketUserEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<i32>,
    #[serde(default)]
    pub agents: Vec<TicketUserEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solve: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<TicketUserEvent>,
    #[serde(default)]
    pub collaborators: Vec<TicketUserEvent>,
    #[serde(default)]
    pub customized_fields: Vec<CustomizedFieldDisplayItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TicketEventUpdateInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Ticket {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest: Option<TicketUser>,
    #[serde(default)]
    pub comments: Vec<Comments>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dissatisfaction_reason: Option<I18n>,
    #[serde(default)]
    pub agents: Vec<TicketUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solve: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<TicketUser>,
    #[serde(default)]
    pub collaborators: Vec<TicketUser>,
    #[serde(default)]
    pub customized_fields: Vec<CustomizedFieldDisplayItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_service_duration: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_first_response_duration: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_service_duration: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_resolution_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_processing_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_entry_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_first_response_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_last_response_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_owner: Option<TicketUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TicketMessageContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    #[serde(default)]
    pub image_keys: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2HelpdeskNotificationApproveV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2HelpdeskTicketCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest: Option<TicketUserEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solve: Option<i32>,
    #[serde(default)]
    pub customized_fields: Vec<CustomizedFieldDisplayItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2HelpdeskTicketUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<TicketEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_object: Option<TicketEventUpdateInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2HelpdeskTicketMessageCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket_message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket: Option<Ticket>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<TicketMessageContent>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_helpdesk_ticket_created_v1 => P2HelpdeskTicketCreatedV1
        : "helpdesk.ticket.created_v1",
    on_p2_helpdesk_ticket_updated_v1 => P2HelpdeskTicketUpdatedV1
        : "helpdesk.ticket.updated_v1",
    on_p2_helpdesk_ticket_message_created_v1 => P2HelpdeskTicketMessageCreatedV1
        : "helpdesk.ticket_message.created_v1",
    on_p2_helpdesk_notification_approve_v1 => P2HelpdeskNotificationApproveV1
        : "helpdesk.notification.approve_v1",
}
