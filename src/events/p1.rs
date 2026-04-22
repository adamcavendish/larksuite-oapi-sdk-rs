//! P1 (v1.0 protocol) legacy event handlers.
//!
//! P1 events use `event.type` for routing rather than the P2 `header.event_type`.
//! The dispatcher handles both protocols transparently.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

// ── P1 payload types ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTicketEvent {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub app_ticket: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1OrderPaidV6 {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub price_plan_id: String,
    #[serde(default)]
    pub price_plan_type: String,
    #[serde(default)]
    pub seats: i32,
    #[serde(default)]
    pub buy_count: i32,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub pay_time: String,
    #[serde(default)]
    pub buy_type: String,
    #[serde(default)]
    pub src_order_id: String,
    #[serde(default)]
    pub operator_id: String,
    #[serde(default)]
    pub operator_name: String,
    #[serde(default)]
    pub tenant_key: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1AppUninstalledV6 {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1AppStatusChangedV6 {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1AppOpenV6 {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub applicants: Vec<serde_json::Value>,
    #[serde(default)]
    pub installer: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1MessageReceiveV1 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub msg_type: String,
    #[serde(default)]
    pub message_id: String,
    #[serde(default)]
    pub root_id: String,
    #[serde(default)]
    pub parent_id: String,
    #[serde(default)]
    pub open_chat_id: String,
    #[serde(default)]
    pub chat_type: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub union_id: String,
    #[serde(default)]
    pub open_message_id: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub text_without_at_bot: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1MessageReadV1 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub open_message_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1AddBotV1 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub chat_name: String,
    #[serde(default)]
    pub chat_owner_id: String,
    #[serde(default)]
    pub chat_owner_open_id: String,
    #[serde(default)]
    pub operator_id: String,
    #[serde(default)]
    pub operator_open_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1RemoveBotV1 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: String,
    #[serde(default)]
    pub operator_open_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1P2PChatCreatedV1 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator: serde_json::Value,
    #[serde(default)]
    pub user: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1ChatDisbandV1 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1GroupSettingUpdatedV1 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: String,
    #[serde(default)]
    pub after_change: serde_json::Value,
    #[serde(default)]
    pub before_change: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1UserInOutChatV1 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub chat_id: String,
    #[serde(default)]
    pub operator_id: String,
    #[serde(default)]
    pub users: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1UserStatusChangedV3 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub union_id: String,
    #[serde(default)]
    pub before_status: serde_json::Value,
    #[serde(default)]
    pub current_status: serde_json::Value,
    #[serde(default)]
    pub change_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1UserChangedV3 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub union_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1DepartmentChangedV3 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub open_department_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1ContactScopeChangedV3 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1ThirdPartyMeetingRoomChangedV1 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub roomid: String,
    #[serde(default)]
    pub uid: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1LeaveApprovalV4 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub instance_code: String,
    #[serde(default)]
    pub start_time: String,
    #[serde(default)]
    pub end_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1WorkApprovalV4 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub instance_code: String,
    #[serde(default)]
    pub start_time: String,
    #[serde(default)]
    pub end_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1ShiftApprovalV4 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub instance_code: String,
    #[serde(default)]
    pub shift_time: String,
    #[serde(default)]
    pub new_shift_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1RemedyApprovalV4 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub instance_code: String,
    #[serde(default)]
    pub remedy_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1TripApprovalV4 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub instance_code: String,
    #[serde(default)]
    pub trips: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P1OutApprovalV4 {
    #[serde(rename = "type", default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub instance_code: String,
    #[serde(default)]
    pub start_time: String,
    #[serde(default)]
    pub end_time: String,
}

// ── Handler registration helpers ──────────────────────────────────────────────

fn wrap_handler<T, F, Fut>(
    handler: F,
) -> impl Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync + 'static
where
    T: for<'de> serde::Deserialize<'de> + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    move |val: serde_json::Value| {
        let result: std::result::Result<T, _> = serde_json::from_value(val);
        match result {
            Ok(typed) => {
                Box::pin(handler(typed)) as Pin<Box<dyn Future<Output = Result<()>> + Send>>
            }
            Err(e) => Box::pin(async move {
                Err(Error::Event(format!(
                    "failed to deserialize event payload: {e}"
                )))
            }),
        }
    }
}

// ── EventDispatcher P1 extension methods ──────────────────────────────────────

impl EventDispatcher {
    pub fn on_p1_order_paid_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1OrderPaidV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("order_paid", wrap_handler(handler))
    }

    pub fn on_p1_app_uninstalled_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1AppUninstalledV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("app_uninstalled", wrap_handler(handler))
    }

    pub fn on_p1_app_status_changed_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1AppStatusChangedV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("app_status_change", wrap_handler(handler))
    }

    pub fn on_p1_app_open_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1AppOpenV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("app_open", wrap_handler(handler))
    }

    pub fn on_p1_message_receive_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1MessageReceiveV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("message", wrap_handler(handler))
    }

    pub fn on_p1_message_read_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1MessageReadV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("message_read", wrap_handler(handler))
    }

    pub fn on_p1_add_bot_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1AddBotV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("add_bot", wrap_handler(handler))
    }

    pub fn on_p1_remove_bot_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1RemoveBotV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("remove_bot", wrap_handler(handler))
    }

    pub fn on_p1_p2p_chat_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1P2PChatCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("p2p_chat_create", wrap_handler(handler))
    }

    pub fn on_p1_chat_disband_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1ChatDisbandV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("chat_disband", wrap_handler(handler))
    }

    pub fn on_p1_group_setting_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1GroupSettingUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("group_setting_update", wrap_handler(handler))
    }

    /// Registers a handler for user join/leave chat events (P1 protocol).
    /// Covers `add_user_to_chat`, `remove_user_from_chat`, and
    /// `revoke_add_user_from_chat` — all routed to the same handler.
    pub fn on_p1_user_in_out_chat_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1UserInOutChatV1) -> Fut + Send + Sync + 'static + Clone,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("add_user_to_chat", wrap_handler(handler.clone()))
            .on_event("remove_user_from_chat", wrap_handler(handler.clone()))
            .on_event("revoke_add_user_from_chat", wrap_handler(handler))
    }

    pub fn on_p1_user_status_changed_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1UserStatusChangedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("user_status_change", wrap_handler(handler))
    }

    /// Registers a handler for employee add/leave/update events (P1 protocol).
    /// Covers `user_add`, `user_leave`, and `user_update`.
    pub fn on_p1_user_changed_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1UserChangedV3) -> Fut + Send + Sync + 'static + Clone,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("user_add", wrap_handler(handler.clone()))
            .on_event("user_leave", wrap_handler(handler.clone()))
            .on_event("user_update", wrap_handler(handler))
    }

    /// Registers a handler for department add/delete/update events (P1 protocol).
    /// Covers `dept_add`, `dept_delete`, and `dept_update`.
    pub fn on_p1_department_changed_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1DepartmentChangedV3) -> Fut + Send + Sync + 'static + Clone,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("dept_add", wrap_handler(handler.clone()))
            .on_event("dept_delete", wrap_handler(handler.clone()))
            .on_event("dept_update", wrap_handler(handler))
    }

    pub fn on_p1_contact_scope_changed_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1ContactScopeChangedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("contact_scope_change", wrap_handler(handler))
    }

    /// Registers a handler for third-party meeting room calendar events (P1 protocol).
    /// Covers created, updated, and deleted variants.
    pub fn on_p1_third_party_meeting_room_changed_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1ThirdPartyMeetingRoomChangedV1) -> Fut + Send + Sync + 'static + Clone,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event(
            "third_party_meeting_room_event_created",
            wrap_handler(handler.clone()),
        )
        .on_event(
            "third_party_meeting_room_event_updated",
            wrap_handler(handler.clone()),
        )
        .on_event(
            "third_party_meeting_room_event_deleted",
            wrap_handler(handler),
        )
    }

    pub fn on_p1_leave_approval_v4<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1LeaveApprovalV4) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("leave_approvalV2", wrap_handler(handler))
    }

    pub fn on_p1_work_approval_v4<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1WorkApprovalV4) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("work_approval", wrap_handler(handler))
    }

    pub fn on_p1_shift_approval_v4<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1ShiftApprovalV4) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("shift_approval", wrap_handler(handler))
    }

    pub fn on_p1_remedy_approval_v4<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1RemedyApprovalV4) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("remedy_approval", wrap_handler(handler))
    }

    pub fn on_p1_trip_approval_v4<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1TripApprovalV4) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("trip_approval", wrap_handler(handler))
    }

    pub fn on_p1_out_approval_v4<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P1OutApprovalV4) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("out_approval", wrap_handler(handler))
    }

    pub fn on_app_ticket_event<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(AppTicketEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("app_ticket", wrap_handler(handler))
    }
}
