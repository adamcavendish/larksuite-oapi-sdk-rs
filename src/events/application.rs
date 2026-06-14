//! Application v6 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
}

impl UserId {
    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }
    pub fn open_id(&self) -> Option<&str> {
        self.open_id.as_deref()
    }
    pub fn union_id(&self) -> Option<&str> {
        self.union_id.as_deref()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Operator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
}

impl Operator {
    pub fn open_id(&self) -> Option<&str> {
        self.operator_id.as_ref().and_then(UserId::open_id)
    }
    pub fn user_id(&self) -> Option<&str> {
        self.operator_id.as_ref().and_then(UserId::user_id)
    }
    pub fn union_id(&self) -> Option<&str> {
        self.operator_id.as_ref().and_then(UserId::union_id)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppScope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(default)]
    pub token_types: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppI18nInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_use: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationAppVersionEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub scopes: Vec<AppScope>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub back_home_url: Option<String>,
    #[serde(default)]
    pub i18n: Vec<AppI18nInfo>,
    #[serde(default)]
    pub common_categories: Vec<String>,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ability: Option<AppAbility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<AppVersionRemarkEvent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppAbility {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gadget: Option<Gadget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot: Option<Bot>,
    #[serde(default)]
    pub workplace_widgets: Vec<WorkplaceWidget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub navigate: Option<Navigate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud_doc: Option<CloudDoc>,
    #[serde(default)]
    pub docs_blocks: Vec<DocsBlock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_action: Option<MessageAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plus_menu: Option<PlusMenu>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Gadget {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_pc_mode: Option<i32>,
    #[serde(default)]
    pub schema_urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_use_mobile_pkg: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_min_lark_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_min_lark_version: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebApp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Bot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_request_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkplaceWidget {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_lark_version: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Navigate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc: Option<NavigateMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<NavigateMeta>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NavigateMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hover_image_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudDoc {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_url: Option<String>,
    #[serde(default)]
    pub i18n: Vec<CloudDocI18nInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudDocI18nInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocsBlock {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_type_id: Option<String>,
    #[serde(default)]
    pub i18n: Vec<BlockI18nInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_icon_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockI18nInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_app_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_app_link: Option<String>,
    #[serde(default)]
    pub i18n: Vec<MessageActionI18nInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageActionI18nInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlusMenu {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_app_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_app_link: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppVersionRemarkEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<AppVisibilityEvent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppVisibilityEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_all: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_list: Option<AppVisibleListEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invisible_list: Option<AppVisibleListEvent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppVisibleListEvent {
    #[serde(default)]
    pub open_ids: Vec<UserId>,
    #[serde(default)]
    pub department_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationCreatedV6 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_scene_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_source: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationBotMenuV6 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationAppVersionAuditV6 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audit_source: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationAppVersionPublishApplyV6 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_version: Option<ApplicationAppVersionEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub under_audit_version: Option<ApplicationAppVersionEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationAppVersionPublishRevokeV6 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationFeedbackCreatedV6 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_type: Option<i32>,
    #[serde(default)]
    pub fault_type: Vec<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub images: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationFeedbackUpdatedV6 {
    #[serde(default)]
    pub feedback_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ApplicationVisibilityAddedV6 {
    #[serde(default)]
    pub users: Vec<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,
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

    pub fn on_p2_application_created_v6<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ApplicationCreatedV6) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("application.application.created_v6", wrap_handler(handler))
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
