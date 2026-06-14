//! Helpdesk v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HelpdeskTicketCreatedV1 {
    #[serde(default)]
    pub ticket: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HelpdeskTicketUpdatedV1 {
    #[serde(default)]
    pub ticket: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HelpdeskTicketMessageCreatedV1 {
    #[serde(default)]
    pub ticket_message: serde_json::Value,
    #[serde(default)]
    pub ticket: serde_json::Value,
    #[serde(default)]
    pub message: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2HelpdeskNotificationApproveV1 {
    #[serde(default)]
    pub object: serde_json::Value,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_helpdesk_ticket_created_v1 => P2HelpdeskTicketCreatedV1
        : "helpdesk.ticket_message.created_v1",
    on_p2_helpdesk_ticket_updated_v1 => P2HelpdeskTicketUpdatedV1
        : "helpdesk.ticket.updated_v1",
    on_p2_helpdesk_ticket_message_created_v1 => P2HelpdeskTicketMessageCreatedV1
        : "helpdesk.ticket.ticket_message.created_v1",
    on_p2_helpdesk_notification_approve_v1 => P2HelpdeskNotificationApproveV1
        : "helpdesk.notification.approve_v1",
}
