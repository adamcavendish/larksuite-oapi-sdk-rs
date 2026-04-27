//! Application v6 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationCreatedV6 {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub app: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationVisibilityChangedV6 {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub availability_status: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationBotMenuV6 {
    #[serde(default)]
    pub operator: serde_json::Value,
    #[serde(default)]
    pub event_key: String,
    #[serde(default)]
    pub timestamp: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationBotAddedV1 {
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationBotDeletedV1 {
    #[serde(default)]
    pub operator: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationAppVersionAuditV6 {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub version: serde_json::Value,
    #[serde(default)]
    pub audit_node: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationAppVersionPublishApplyV6 {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub version: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationAppVersionPublishRevokeV6 {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub version: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationFeedbackCreatedV6 {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub feedback_id: String,
    #[serde(default)]
    pub feedback: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationFeedbackUpdatedV6 {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub feedback_id: String,
    #[serde(default)]
    pub feedback: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationVisibilityAddedV6 {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub added_members: Vec<serde_json::Value>,
}

// ── Handler registration helpers ──

fn wrap_handler<T, F, Fut>(
    handler: F,
) -> impl Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>
+ Send
+ Sync
+ 'static
where
    T: for<'de> serde::Deserialize<'de> + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
{
    move |val: serde_json::Value| {
        let result: std::result::Result<T, _> = serde_json::from_value(val);
        match result {
            Ok(typed) => Box::pin(handler(typed))
                as Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>,
            Err(e) => Box::pin(async move {
                Err(LarkError::Event(format!(
                    "failed to deserialize event payload: {e}"
                )))
            }),
        }
    }
}

// ── EventDispatcher extension methods ──

impl EventDispatcher {
    pub fn on_p2_application_bot_menu_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApplicationBotMenuV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("application.bot.menu_v6", wrap_handler(handler))
    }

    pub fn on_p2_application_bot_added_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApplicationBotAddedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("im.chat.bot.disbanded_v1", wrap_handler(handler))
    }

    pub fn on_p2_application_bot_deleted_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApplicationBotDeletedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "im.chat.access_event.bot_p2p_chat_entered_v1",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_application_created_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApplicationCreatedV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("application.application.created_v6", wrap_handler(handler))
    }

    pub fn on_p2_application_visibility_changed_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApplicationVisibilityChangedV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "application.application.visibility_changed_v6",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_application_app_version_audit_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApplicationAppVersionAuditV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "application.application.app_version.audit_v6",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_application_app_version_publish_apply_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApplicationAppVersionPublishApplyV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "application.application.app_version.publish_apply_v6",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_application_app_version_publish_revoke_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApplicationAppVersionPublishRevokeV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "application.application.app_version.publish_revoke_v6",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_application_feedback_created_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApplicationFeedbackCreatedV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "application.application.feedback.created_v6",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_application_feedback_updated_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApplicationFeedbackUpdatedV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "application.application.feedback.updated_v6",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_application_visibility_added_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApplicationVisibilityAddedV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "application.application.visibility.added_v6",
            wrap_handler(handler),
        )
    }
}
