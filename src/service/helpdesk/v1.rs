use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest: Option<TicketUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<serde_json::Value>,
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
    pub dissatisfaction_reason: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<TicketUser>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solve: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<TicketUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collaborators: Option<Vec<TicketUser>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customized_fields: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_response_duration: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_human_reply_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_ai_filter: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_user: Option<TicketUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TicketUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TicketMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<TicketUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_skills: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_at: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Category {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Category>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Faq {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faq_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer_richtext: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<Category>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_user: Option<TicketUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_user: Option<TicketUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Notification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_user: Option<TicketUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sent_user_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_user_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_scope_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_staff_scope_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_staff_scope_department_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<TicketUser>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_i18n_contents: Option<Vec<serde_json::Value>>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateTicketReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_fields: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solved: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AnswerUserQueryReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faqs: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTicketMessageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateFaqReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faq: Option<Faq>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateFaqReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faq: Option<Faq>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCategoryReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateCategoryReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateNotificationReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
}

// ── Additional domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentSkillRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_operator: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_options: Option<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentSkill {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AgentSkillRule>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_skill_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TicketCustomizedField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket_customized_field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<TicketUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<TicketUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dropdown_allow_multiple: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receive_type: Option<String>,
}

// ── Additional request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAgentSkillReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AgentSkillRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchAgentSkillReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_skill: Option<AgentSkill>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SubscribeEventReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UnsubscribeEventReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelSendNotificationReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recall: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ExecuteSendNotificationReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SubmitApproveNotificationReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct StartServiceTicketReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_service: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointed_agents: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_info: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTicketCustomizedFieldReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

// ── Response wrappers ──

macro_rules! impl_resp {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: CodeError,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.success()
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct EmptyResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
}

impl EmptyResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TicketData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket: Option<Ticket>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TicketListData {
    #[serde(default)]
    pub tickets: Vec<Ticket>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TicketMessageListData {
    #[serde(default)]
    pub messages: Vec<TicketMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentListData {
    #[serde(default)]
    pub agents: Vec<AgentInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoryData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoryListData {
    #[serde(default)]
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FaqData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faq: Option<Faq>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FaqListData {
    #[serde(default)]
    pub items: Vec<Faq>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetNotificationData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_settings: Option<serde_json::Value>,
}

// ── Additional response data types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAgentSkillRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_skill_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAgentSkillRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_skill: Option<AgentSkill>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAgentSkillRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_skills: Option<Vec<AgentSkill>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAgentSkillRuleRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AgentSkillRule>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBotMessageRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchFaqRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Faq>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubmitApproveNotificationRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_access: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartServiceTicketRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomizedFieldsTicketRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_customized_fields: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket_customized_fields: Option<Vec<TicketCustomizedField>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTicketCustomizedFieldRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket_customized_field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helpdesk_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<TicketUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<TicketUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dropdown_allow_multiple: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTicketCustomizedFieldRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TicketCustomizedField>>,
}

// ── Download response ──

#[derive(Debug, Clone)]
pub struct DownloadResp {
    pub api_resp: ApiResp,
    pub file_name: Option<String>,
    pub data: Vec<u8>,
}

impl_resp!(GetTicketResp, TicketData);
impl_resp!(ListTicketResp, TicketListData);
impl_resp!(ListTicketMessageResp, TicketMessageListData);
impl_resp!(ListAgentResp, AgentListData);
impl_resp!(CreateCategoryResp, CategoryData);
impl_resp!(GetCategoryResp, CategoryData);
impl_resp!(ListCategoryResp, CategoryListData);
impl_resp!(CreateFaqResp, FaqData);
impl_resp!(GetFaqResp, FaqData);
impl_resp!(ListFaqResp, FaqListData);
impl_resp!(CreateNotificationResp, NotificationData);
impl_resp!(GetNotificationResp, GetNotificationData);
impl_resp!(CreateAgentSkillResp, CreateAgentSkillRespData);
impl_resp!(GetAgentSkillResp, GetAgentSkillRespData);
impl_resp!(ListAgentSkillResp, ListAgentSkillRespData);
impl_resp!(ListAgentSkillRuleResp, ListAgentSkillRuleRespData);
impl_resp!(CreateBotMessageResp, CreateBotMessageRespData);
impl_resp!(SearchFaqResp, SearchFaqRespData);
impl_resp!(
    SubmitApproveNotificationResp,
    SubmitApproveNotificationRespData
);
impl_resp!(StartServiceTicketResp, StartServiceTicketRespData);
impl_resp!(CustomizedFieldsTicketResp, CustomizedFieldsTicketRespData);
impl_resp!(
    GetTicketCustomizedFieldResp,
    GetTicketCustomizedFieldRespData
);
impl_resp!(
    ListTicketCustomizedFieldResp,
    ListTicketCustomizedFieldRespData
);

// ── Resources ──

pub struct TicketResource<'a> {
    config: &'a Config,
}

impl<'a> TicketResource<'a> {
    pub async fn get(&self, ticket_id: &str, option: &RequestOption) -> Result<GetTicketResp> {
        let path = format!("/open-apis/helpdesk/v1/tickets/{ticket_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<TicketData>(self.config, &api_req, option).await?;
        Ok(GetTicketResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        ticket_id: &str,
        body: &UpdateTicketReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/tickets/{ticket_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        ticket_id: Option<&str>,
        agent_id: Option<&str>,
        closed_by_id: Option<&str>,
        status: Option<i32>,
        guest_id: Option<&str>,
        keyword: Option<&str>,
        create_time_start: Option<i64>,
        create_time_end: Option<i64>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTicketResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/helpdesk/v1/tickets");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = ticket_id {
            api_req.query_params.set("ticket_id", v);
        }
        if let Some(v) = agent_id {
            api_req.query_params.set("agent_id", v);
        }
        if let Some(v) = closed_by_id {
            api_req.query_params.set("closed_by_id", v);
        }
        if let Some(v) = status {
            api_req.query_params.set("status", v.to_string());
        }
        if let Some(v) = guest_id {
            api_req.query_params.set("guest_id", v);
        }
        if let Some(v) = keyword {
            api_req.query_params.set("keyword", v);
        }
        if let Some(v) = create_time_start {
            api_req.query_params.set("create_time_start", v.to_string());
        }
        if let Some(v) = create_time_end {
            api_req.query_params.set("create_time_end", v.to_string());
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TicketListData>(self.config, &api_req, option).await?;
        Ok(ListTicketResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn answer_user_query(
        &self,
        ticket_id: &str,
        body: &AnswerUserQueryReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/tickets/{ticket_id}/answer_user_query");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn start_service(
        &self,
        body: &StartServiceTicketReqBody,
        option: &RequestOption,
    ) -> Result<StartServiceTicketResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/helpdesk/v1/start_service");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<StartServiceTicketRespData>(self.config, &api_req, option)
                .await?;
        Ok(StartServiceTicketResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn customized_fields(
        &self,
        visible_only: Option<bool>,
        option: &RequestOption,
    ) -> Result<CustomizedFieldsTicketResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/helpdesk/v1/customized_fields",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = visible_only {
            api_req.query_params.set("visible_only", v.to_string());
        }
        let (api_resp, raw) = transport::request_typed::<CustomizedFieldsTicketRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(CustomizedFieldsTicketResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn ticket_image(
        &self,
        ticket_id: &str,
        msg_id: &str,
        index: Option<i32>,
        option: &RequestOption,
    ) -> Result<DownloadResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/helpdesk/v1/ticket_images");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("ticket_id", ticket_id);
        api_req.query_params.set("msg_id", msg_id);
        if let Some(v) = index {
            api_req.query_params.set("index", v.to_string());
        }
        let mut opt = option.clone();
        opt.file_download = true;
        let api_resp = transport::request(self.config, &api_req, &opt).await?;
        let file_name = api_resp.file_name_by_header();
        let data = api_resp.raw_body.clone();
        Ok(DownloadResp {
            api_resp,
            file_name,
            data,
        })
    }
}

pub struct TicketMessageResource<'a> {
    config: &'a Config,
}

impl<'a> TicketMessageResource<'a> {
    pub async fn create(
        &self,
        ticket_id: &str,
        body: &CreateTicketMessageReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/tickets/{ticket_id}/messages");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        ticket_id: &str,
        time_start: Option<i64>,
        time_end: Option<i64>,
        page_token: Option<i64>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListTicketMessageResp> {
        let path = format!("/open-apis/helpdesk/v1/tickets/{ticket_id}/messages");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = time_start {
            api_req.query_params.set("time_start", v.to_string());
        }
        if let Some(v) = time_end {
            api_req.query_params.set("time_end", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v.to_string());
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<TicketMessageListData>(self.config, &api_req, option)
                .await?;
        Ok(ListTicketMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn image(
        &self,
        ticket_id: &str,
        msg_id: &str,
        index: Option<i32>,
        option: &RequestOption,
    ) -> Result<DownloadResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/helpdesk/v1/ticket_images");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("ticket_id", ticket_id);
        api_req.query_params.set("msg_id", msg_id);
        if let Some(v) = index {
            api_req.query_params.set("index", v.to_string());
        }
        let mut opt = option.clone();
        opt.file_download = true;
        let api_resp = transport::request(self.config, &api_req, &opt).await?;
        let file_name = api_resp.file_name_by_header();
        let data = api_resp.raw_body.clone();
        Ok(DownloadResp {
            api_resp,
            file_name,
            data,
        })
    }
}

pub struct AgentResource<'a> {
    config: &'a Config,
}

impl<'a> AgentResource<'a> {
    pub async fn list(&self, status: Option<i32>, option: &RequestOption) -> Result<ListAgentResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/helpdesk/v1/agents");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = status {
            api_req.query_params.set("status", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<AgentListData>(self.config, &api_req, option).await?;
        Ok(ListAgentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct CategoryResource<'a> {
    config: &'a Config,
}

impl<'a> CategoryResource<'a> {
    pub async fn create(
        &self,
        body: &CreateCategoryReqBody,
        option: &RequestOption,
    ) -> Result<CreateCategoryResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/helpdesk/v1/categories");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CategoryData>(self.config, &api_req, option).await?;
        Ok(CreateCategoryResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(&self, id: &str, option: &RequestOption) -> Result<GetCategoryResp> {
        let path = format!("/open-apis/helpdesk/v1/categories/{id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<CategoryData>(self.config, &api_req, option).await?;
        Ok(GetCategoryResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        id: &str,
        body: &UpdateCategoryReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/categories/{id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn delete(&self, id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/categories/{id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        language: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCategoryResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/helpdesk/v1/categories");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = language {
            api_req.query_params.set("language", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<CategoryListData>(self.config, &api_req, option).await?;
        Ok(ListCategoryResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct FaqResource<'a> {
    config: &'a Config,
}

impl<'a> FaqResource<'a> {
    pub async fn create(
        &self,
        body: &CreateFaqReqBody,
        option: &RequestOption,
    ) -> Result<CreateFaqResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/helpdesk/v1/faqs");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FaqData>(self.config, &api_req, option).await?;
        Ok(CreateFaqResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(&self, id: &str, option: &RequestOption) -> Result<GetFaqResp> {
        let path = format!("/open-apis/helpdesk/v1/faqs/{id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<FaqData>(self.config, &api_req, option).await?;
        Ok(GetFaqResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        id: &str,
        body: &UpdateFaqReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/faqs/{id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn delete(&self, id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/faqs/{id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        category_id: Option<&str>,
        keyword: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListFaqResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/helpdesk/v1/faqs");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = category_id {
            api_req.query_params.set("category_id", v);
        }
        if let Some(v) = keyword {
            api_req.query_params.set("keyword", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<FaqListData>(self.config, &api_req, option).await?;
        Ok(ListFaqResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn image(
        &self,
        id: &str,
        image_key: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp> {
        let path = format!("/open-apis/helpdesk/v1/faqs/{id}/image/{image_key}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let mut opt = option.clone();
        opt.file_download = true;
        let api_resp = transport::request(self.config, &api_req, &opt).await?;
        let file_name = api_resp.file_name_by_header();
        let data = api_resp.raw_body.clone();
        Ok(DownloadResp {
            api_resp,
            file_name,
            data,
        })
    }

    pub async fn search(
        &self,
        query: Option<&str>,
        base64: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<SearchFaqResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/helpdesk/v1/faqs/search");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = query {
            api_req.query_params.set("query", v);
        }
        if let Some(v) = base64 {
            api_req.query_params.set("base64", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<SearchFaqRespData>(self.config, &api_req, option).await?;
        Ok(SearchFaqResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct NotificationResource<'a> {
    config: &'a Config,
}

impl<'a> NotificationResource<'a> {
    pub async fn create(
        &self,
        body: &CreateNotificationReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateNotificationResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/helpdesk/v1/notifications");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<NotificationData>(self.config, &api_req, option).await?;
        Ok(CreateNotificationResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        notification_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetNotificationResp> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<GetNotificationData>(self.config, &api_req, option).await?;
        Ok(GetNotificationResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn submit_approve(
        &self,
        notification_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/submit_approve");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn execute(
        &self,
        notification_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/execute");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn cancel(&self, notification_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/cancel");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn cancel_approve(
        &self,
        notification_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/cancel_approve");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn cancel_send(
        &self,
        notification_id: &str,
        body: &CancelSendNotificationReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/cancel_send");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn execute_send(
        &self,
        notification_id: &str,
        body: &ExecuteSendNotificationReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/execute_send");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn preview(
        &self,
        notification_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/preview");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn patch(
        &self,
        notification_id: &str,
        body: &CreateNotificationReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn submit_approve_v2(
        &self,
        notification_id: &str,
        body: &SubmitApproveNotificationReqBody,
        option: &RequestOption,
    ) -> Result<SubmitApproveNotificationResp> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/submit_approve");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<SubmitApproveNotificationRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(SubmitApproveNotificationResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── AgentSkill resource ──

pub struct AgentSkillResource<'a> {
    config: &'a Config,
}

impl<'a> AgentSkillResource<'a> {
    pub async fn create(
        &self,
        body: &CreateAgentSkillReqBody,
        option: &RequestOption,
    ) -> Result<CreateAgentSkillResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/helpdesk/v1/agent_skills");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateAgentSkillRespData>(self.config, &api_req, option)
                .await?;
        Ok(CreateAgentSkillResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(&self, agent_skill_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/agent_skills/{agent_skill_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get(
        &self,
        agent_skill_id: &str,
        option: &RequestOption,
    ) -> Result<GetAgentSkillResp> {
        let path = format!("/open-apis/helpdesk/v1/agent_skills/{agent_skill_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<GetAgentSkillRespData>(self.config, &api_req, option)
                .await?;
        Ok(GetAgentSkillResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(&self, option: &RequestOption) -> Result<ListAgentSkillResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/helpdesk/v1/agent_skills");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<ListAgentSkillRespData>(self.config, &api_req, option)
                .await?;
        Ok(ListAgentSkillResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        agent_skill_id: &str,
        body: &PatchAgentSkillReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/helpdesk/v1/agent_skills/{agent_skill_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

// ── AgentSkillRule resource ──

pub struct AgentSkillRuleResource<'a> {
    config: &'a Config,
}

impl<'a> AgentSkillRuleResource<'a> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListAgentSkillRuleResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/helpdesk/v1/agent_skill_rules",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<ListAgentSkillRuleRespData>(self.config, &api_req, option)
                .await?;
        Ok(ListAgentSkillRuleResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── BotMessage resource ──

pub struct BotMessageResource<'a> {
    config: &'a Config,
}

impl<'a> BotMessageResource<'a> {
    pub async fn create(
        &self,
        body: &BotMessage,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateBotMessageResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/helpdesk/v1/message");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CreateBotMessageRespData>(self.config, &api_req, option)
                .await?;
        Ok(CreateBotMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Event resource ──

pub struct EventResource<'a> {
    config: &'a Config,
}

impl<'a> EventResource<'a> {
    pub async fn subscribe(
        &self,
        body: &SubscribeEventReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/helpdesk/v1/events/subscribe",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn unsubscribe(
        &self,
        body: &UnsubscribeEventReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/helpdesk/v1/events/unsubscribe",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

// ── TicketCustomizedField resource ──

pub struct TicketCustomizedFieldResource<'a> {
    config: &'a Config,
}

impl<'a> TicketCustomizedFieldResource<'a> {
    pub async fn create(
        &self,
        body: &TicketCustomizedField,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/helpdesk/v1/ticket_customized_fields",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn delete(
        &self,
        ticket_customized_field_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path =
            format!("/open-apis/helpdesk/v1/ticket_customized_fields/{ticket_customized_field_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get(
        &self,
        ticket_customized_field_id: &str,
        option: &RequestOption,
    ) -> Result<GetTicketCustomizedFieldResp> {
        let path =
            format!("/open-apis/helpdesk/v1/ticket_customized_fields/{ticket_customized_field_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<GetTicketCustomizedFieldRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(GetTicketCustomizedFieldResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_token: Option<&str>,
        page_size: Option<i32>,
        body: Option<&ListTicketCustomizedFieldReqBody>,
        option: &RequestOption,
    ) -> Result<ListTicketCustomizedFieldResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/helpdesk/v1/ticket_customized_fields",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(b) = body {
            api_req.body = Some(ReqBody::json(b)?);
        }
        let (api_resp, raw) = transport::request_typed::<ListTicketCustomizedFieldRespData>(
            self.config,
            &api_req,
            option,
        )
        .await?;
        Ok(ListTicketCustomizedFieldResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        ticket_customized_field_id: &str,
        body: &TicketCustomizedField,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path =
            format!("/open-apis/helpdesk/v1/ticket_customized_fields/{ticket_customized_field_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub ticket: TicketResource<'a>,
    pub ticket_message: TicketMessageResource<'a>,
    pub agent: AgentResource<'a>,
    pub agent_skill: AgentSkillResource<'a>,
    pub agent_skill_rule: AgentSkillRuleResource<'a>,
    pub bot_message: BotMessageResource<'a>,
    pub category: CategoryResource<'a>,
    pub event: EventResource<'a>,
    pub faq: FaqResource<'a>,
    pub notification: NotificationResource<'a>,
    pub ticket_customized_field: TicketCustomizedFieldResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            ticket: TicketResource { config },
            ticket_message: TicketMessageResource { config },
            agent: AgentResource { config },
            agent_skill: AgentSkillResource { config },
            agent_skill_rule: AgentSkillRuleResource { config },
            bot_message: BotMessageResource { config },
            category: CategoryResource { config },
            event: EventResource { config },
            faq: FaqResource { config },
            notification: NotificationResource { config },
            ticket_customized_field: TicketCustomizedFieldResource { config },
        }
    }
}
