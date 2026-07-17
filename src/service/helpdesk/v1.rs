use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{DownloadResp, EmptyResp, PageQuery, RestRequest};

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
    pub comments: Option<Vec<Comment>>,
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
    pub customized_fields: Option<Vec<CustomizedFieldDisplayItem>>,
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
    pub agent_skills: Option<Vec<AgentSkillLessInfo>>,
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
    pub answer_richtext: Option<Vec<Richtext>>,
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
    pub new_staff_scope_department_list: Option<Vec<NotificationDepartment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<TicketUser>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_list: Option<Vec<NotificationDepartment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_list: Option<Vec<NotificationChat>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_i18n_contents: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Comment {
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
pub struct I18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
pub struct Richtext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationDepartment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationChat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserQueryFaqInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HelpdeskEvent {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
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
    pub customized_fields: Option<Vec<CustomizedFieldDisplayItem>>,
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
    pub faqs: Option<Vec<UserQueryFaqInfo>>,
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
pub struct UserCustomizedField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_customized_field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    pub events: Option<Vec<HelpdeskEvent>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UnsubscribeEventReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<HelpdeskEvent>>,
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
pub struct AgentUser {
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
    pub company_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WeekdaySchedule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekday: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentSkillLessInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentSchedule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Vec<WeekdaySchedule>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_skills: Option<Vec<AgentSkillLessInfo>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentEmailData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agents: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAgentSchedulesData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_schedule: Option<AgentSchedule>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAgentScheduleData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_schedules: Option<Vec<AgentSchedule>>,
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
    pub user_customized_fields: Option<Vec<UserCustomizedField>>,
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
impl_resp!(AgentEmailResp, AgentEmailData);
impl_resp!(PatchAgentResp, ());
impl_resp!(DeleteAgentSchedulesResp, ());
impl_resp!(GetAgentSchedulesResp, GetAgentSchedulesData);
impl_resp!(PatchAgentSchedulesResp, ());
impl_resp!(CreateAgentScheduleResp, ());
impl_resp!(ListAgentScheduleResp, ListAgentScheduleData);

// ── Resources ──

pub struct TicketResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetTicketQuery<'a> {
    pub ticket_id: &'a str,
}

impl<'a> GetTicketQuery<'a> {
    pub fn new(ticket_id: &'a str) -> Self {
        Self { ticket_id }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListTicketQuery<'a> {
    pub page: PageQuery<'a>,
    pub ticket_id: Option<&'a str>,
    pub agent_id: Option<&'a str>,
    pub closed_by_id: Option<&'a str>,
    pub status: Option<i32>,
    pub guest_id: Option<&'a str>,
    pub keyword: Option<&'a str>,
    pub create_time_start: Option<i64>,
    pub create_time_end: Option<i64>,
}

impl<'a> ListTicketQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn ticket_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.ticket_id = value.into();
        self
    }

    pub fn agent_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.agent_id = value.into();
        self
    }

    pub fn closed_by_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.closed_by_id = value.into();
        self
    }

    pub fn status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.status = value.into();
        self
    }

    pub fn guest_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.guest_id = value.into();
        self
    }

    pub fn keyword(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.keyword = value.into();
        self
    }

    pub fn create_time_start(mut self, value: impl Into<Option<i64>>) -> Self {
        self.create_time_start = value.into();
        self
    }

    pub fn create_time_end(mut self, value: impl Into<Option<i64>>) -> Self {
        self.create_time_end = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct CustomizedFieldsTicketQuery {
    pub visible_only: Option<bool>,
}

impl CustomizedFieldsTicketQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn visible_only(mut self, value: impl Into<Option<bool>>) -> Self {
        self.visible_only = value.into();
        self
    }
}

impl<'a> TicketResource<'a> {
    pub async fn get(
        &self,
        ticket_id: &str,
        option: &RequestOption,
    ) -> Result<GetTicketResp, LarkError> {
        let query = GetTicketQuery::new(ticket_id);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetTicketQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetTicketResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/tickets/{}", query.ticket_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<TicketData, GetTicketResp>()
        .await
    }

    pub async fn update(
        &self,
        ticket_id: &str,
        body: &UpdateTicketReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/tickets/{ticket_id}");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
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
    ) -> Result<ListTicketResp, LarkError> {
        let query = ListTicketQuery {
            page: PageQuery::from_parts(page_size, page_token),
            ticket_id,
            agent_id,
            closed_by_id,
            status,
            guest_id,
            keyword,
            create_time_start,
            create_time_end,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListTicketQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListTicketResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/helpdesk/v1/tickets",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("ticket_id", query.ticket_id)
        .query("agent_id", query.agent_id)
        .query("closed_by_id", query.closed_by_id)
        .query("status", query.status)
        .query("guest_id", query.guest_id)
        .query("keyword", query.keyword)
        .query("create_time_start", query.create_time_start)
        .query("create_time_end", query.create_time_end)
        .page_query(query.page)
        .send_response::<TicketListData, ListTicketResp>()
        .await
    }

    pub async fn answer_user_query(
        &self,
        ticket_id: &str,
        body: &AnswerUserQueryReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/tickets/{ticket_id}/answer_user_query");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn start_service(
        &self,
        body: &StartServiceTicketReqBody,
        option: &RequestOption,
    ) -> Result<StartServiceTicketResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/helpdesk/v1/start_service",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_response::<StartServiceTicketRespData, StartServiceTicketResp>()
        .await
    }

    pub async fn customized_fields(
        &self,
        visible_only: Option<bool>,
        option: &RequestOption,
    ) -> Result<CustomizedFieldsTicketResp, LarkError> {
        let query = CustomizedFieldsTicketQuery { visible_only };
        self.customized_fields_by_query(&query, option).await
    }

    pub async fn customized_fields_by_query(
        &self,
        query: &CustomizedFieldsTicketQuery,
        option: &RequestOption,
    ) -> Result<CustomizedFieldsTicketResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/helpdesk/v1/customized_fields",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("visible_only", query.visible_only)
        .send_response::<CustomizedFieldsTicketRespData, CustomizedFieldsTicketResp>()
        .await
    }

    pub async fn ticket_image(
        &self,
        ticket_id: &str,
        msg_id: &str,
        index: Option<i32>,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/helpdesk/v1/ticket_images",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("ticket_id", ticket_id)
        .query("msg_id", msg_id)
        .query("index", index)
        .download()
        .await
    }
}

pub struct TicketMessageResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListTicketMessageQuery<'a> {
    pub ticket_id: &'a str,
    pub time_start: Option<i64>,
    pub time_end: Option<i64>,
    pub page_token: Option<i64>,
    pub page_size: Option<i32>,
}

impl<'a> ListTicketMessageQuery<'a> {
    pub fn new(ticket_id: &'a str) -> Self {
        Self {
            ticket_id,
            time_start: None,
            time_end: None,
            page_token: None,
            page_size: None,
        }
    }

    pub fn time_start(mut self, value: impl Into<Option<i64>>) -> Self {
        self.time_start = value.into();
        self
    }

    pub fn time_end(mut self, value: impl Into<Option<i64>>) -> Self {
        self.time_end = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<i64>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }
}

impl<'a> TicketMessageResource<'a> {
    pub async fn create(
        &self,
        ticket_id: &str,
        body: &CreateTicketMessageReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/tickets/{ticket_id}/messages");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn list(
        &self,
        ticket_id: &str,
        time_start: Option<i64>,
        time_end: Option<i64>,
        page_token: Option<i64>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListTicketMessageResp, LarkError> {
        let query = ListTicketMessageQuery {
            ticket_id,
            time_start,
            time_end,
            page_token,
            page_size,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListTicketMessageQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListTicketMessageResp, LarkError> {
        let path = format!(
            "/open-apis/helpdesk/v1/tickets/{}/messages",
            query.ticket_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("time_start", query.time_start)
        .query("time_end", query.time_end)
        .query("page_token", query.page_token)
        .query("page_size", query.page_size)
        .send_response::<TicketMessageListData, ListTicketMessageResp>()
        .await
    }

    pub async fn image(
        &self,
        ticket_id: &str,
        msg_id: &str,
        index: Option<i32>,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/helpdesk/v1/ticket_images",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("ticket_id", ticket_id)
        .query("msg_id", msg_id)
        .query("index", index)
        .download()
        .await
    }
}

pub struct AgentResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListAgentQuery {
    pub status: Option<i32>,
}

impl ListAgentQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.status = value.into();
        self
    }
}

impl<'a> AgentResource<'a> {
    pub async fn list(
        &self,
        status: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListAgentResp, LarkError> {
        let query = ListAgentQuery { status };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAgentQuery,
        option: &RequestOption,
    ) -> Result<ListAgentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/helpdesk/v1/agents",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("status", query.status)
        .send_response::<AgentListData, ListAgentResp>()
        .await
    }

    pub async fn agent_email(&self, option: &RequestOption) -> Result<AgentEmailResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/helpdesk/v1/agent_emails",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<AgentEmailData, AgentEmailResp>()
        .await
    }

    pub async fn patch(
        &self,
        agent_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchAgentResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/agents/{agent_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_response::<(), PatchAgentResp>()
        .await
    }
}

pub struct AgentSchedulesResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetAgentSchedulesQuery<'a> {
    pub agent_id: &'a str,
}

impl<'a> GetAgentSchedulesQuery<'a> {
    pub fn new(agent_id: &'a str) -> Self {
        Self { agent_id }
    }
}

impl<'a> AgentSchedulesResource<'a> {
    pub async fn delete(
        &self,
        agent_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteAgentSchedulesResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/agents/{agent_id}/schedules");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_response::<(), DeleteAgentSchedulesResp>()
        .await
    }

    pub async fn get(
        &self,
        agent_id: &str,
        option: &RequestOption,
    ) -> Result<GetAgentSchedulesResp, LarkError> {
        let query = GetAgentSchedulesQuery::new(agent_id);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetAgentSchedulesQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetAgentSchedulesResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/agents/{}/schedules", query.agent_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<GetAgentSchedulesData, GetAgentSchedulesResp>()
        .await
    }

    pub async fn patch(
        &self,
        agent_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchAgentSchedulesResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/agents/{agent_id}/schedules");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_response::<(), PatchAgentSchedulesResp>()
        .await
    }
}

pub struct AgentScheduleResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListAgentScheduleQuery<'a> {
    pub status: Option<&'a [i32]>,
}

impl<'a> ListAgentScheduleQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(mut self, value: impl Into<Option<&'a [i32]>>) -> Self {
        self.status = value.into();
        self
    }
}

impl<'a> AgentScheduleResource<'a> {
    pub async fn create(
        &self,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<CreateAgentScheduleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/helpdesk/v1/agent_schedules",
            vec![AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_response::<(), CreateAgentScheduleResp>()
        .await
    }

    pub async fn list(
        &self,
        status: Option<&[i32]>,
        option: &RequestOption,
    ) -> Result<ListAgentScheduleResp, LarkError> {
        let query = ListAgentScheduleQuery { status };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAgentScheduleQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAgentScheduleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/helpdesk/v1/agent_schedules",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query_values("status", query.status)
        .send_response::<ListAgentScheduleData, ListAgentScheduleResp>()
        .await
    }
}

pub struct CategoryResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetCategoryQuery<'a> {
    pub id: &'a str,
}

impl<'a> GetCategoryQuery<'a> {
    pub fn new(id: &'a str) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListCategoryQuery<'a> {
    pub language: Option<&'a str>,
}

impl<'a> ListCategoryQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn language(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.language = value.into();
        self
    }
}

impl<'a> CategoryResource<'a> {
    pub async fn create(
        &self,
        body: &CreateCategoryReqBody,
        option: &RequestOption,
    ) -> Result<CreateCategoryResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/helpdesk/v1/categories",
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<CategoryData, CreateCategoryResp>()
        .await
    }

    pub async fn get(
        &self,
        id: &str,
        option: &RequestOption,
    ) -> Result<GetCategoryResp, LarkError> {
        let query = GetCategoryQuery::new(id);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetCategoryQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetCategoryResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/categories/{}", query.id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<CategoryData, GetCategoryResp>()
        .await
    }

    pub async fn update(
        &self,
        id: &str,
        body: &UpdateCategoryReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/categories/{id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn delete(&self, id: &str, option: &RequestOption) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/categories/{id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn list(
        &self,
        language: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCategoryResp, LarkError> {
        let query = ListCategoryQuery { language };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCategoryQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCategoryResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/helpdesk/v1/categories",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("language", query.language)
        .send_response::<CategoryListData, ListCategoryResp>()
        .await
    }
}

pub struct FaqResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetFaqQuery<'a> {
    pub id: &'a str,
}

impl<'a> GetFaqQuery<'a> {
    pub fn new(id: &'a str) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListFaqQuery<'a> {
    pub page: PageQuery<'a>,
    pub category_id: Option<&'a str>,
    pub keyword: Option<&'a str>,
}

impl<'a> ListFaqQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn category_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.category_id = value.into();
        self
    }

    pub fn keyword(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.keyword = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct SearchFaqQuery<'a> {
    pub page: PageQuery<'a>,
    pub query: Option<&'a str>,
    pub base64: Option<&'a str>,
}

impl<'a> SearchFaqQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn query(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.query = value.into();
        self
    }

    pub fn base64(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.base64 = value.into();
        self
    }
}

impl<'a> FaqResource<'a> {
    pub async fn create(
        &self,
        body: &CreateFaqReqBody,
        option: &RequestOption,
    ) -> Result<CreateFaqResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/helpdesk/v1/faqs",
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<FaqData, CreateFaqResp>()
        .await
    }

    pub async fn get(&self, id: &str, option: &RequestOption) -> Result<GetFaqResp, LarkError> {
        let query = GetFaqQuery::new(id);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetFaqQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetFaqResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/faqs/{}", query.id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<FaqData, GetFaqResp>()
        .await
    }

    pub async fn update(
        &self,
        id: &str,
        body: &UpdateFaqReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/faqs/{id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn delete(&self, id: &str, option: &RequestOption) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/faqs/{id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        category_id: Option<&str>,
        keyword: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListFaqResp, LarkError> {
        let query = ListFaqQuery {
            page: PageQuery::from_parts(page_size, page_token),
            category_id,
            keyword,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListFaqQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListFaqResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/helpdesk/v1/faqs",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("category_id", query.category_id)
        .query("keyword", query.keyword)
        .send_response::<FaqListData, ListFaqResp>()
        .await
    }

    pub async fn image(
        &self,
        id: &str,
        image_key: &str,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/faqs/{id}/image/{image_key}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .download()
        .await
    }

    pub async fn search(
        &self,
        query: Option<&str>,
        base64: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<SearchFaqResp, LarkError> {
        let query = SearchFaqQuery {
            page: PageQuery::from_parts(page_size, page_token),
            query,
            base64,
        };
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchFaqQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchFaqResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/helpdesk/v1/faqs/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("query", query.query)
        .query("base64", query.base64)
        .page_query(query.page)
        .send_response::<SearchFaqRespData, SearchFaqResp>()
        .await
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
    ) -> Result<CreateNotificationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/helpdesk/v1/notifications",
            vec![AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<NotificationData, CreateNotificationResp>()
        .await
    }

    pub async fn get(
        &self,
        notification_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetNotificationResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_response::<GetNotificationData, GetNotificationResp>()
        .await
    }

    pub async fn submit_approve(
        &self,
        notification_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/submit_approve");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(&body)?
        .send_empty()
        .await
    }

    pub async fn execute(
        &self,
        notification_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/execute");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(&body)?
        .send_empty()
        .await
    }

    pub async fn cancel(
        &self,
        notification_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/cancel");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn cancel_approve(
        &self,
        notification_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/cancel_approve");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn cancel_send(
        &self,
        notification_id: &str,
        body: &CancelSendNotificationReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/cancel_send");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn execute_send(
        &self,
        notification_id: &str,
        body: &ExecuteSendNotificationReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/execute_send");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn preview(
        &self,
        notification_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/preview");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn patch(
        &self,
        notification_id: &str,
        body: &CreateNotificationReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn submit_approve_v2(
        &self,
        notification_id: &str,
        body: &SubmitApproveNotificationReqBody,
        option: &RequestOption,
    ) -> Result<SubmitApproveNotificationResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/notifications/{notification_id}/submit_approve");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<SubmitApproveNotificationRespData, SubmitApproveNotificationResp>()
        .await
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
    ) -> Result<CreateAgentSkillResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/helpdesk/v1/agent_skills",
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<CreateAgentSkillRespData, CreateAgentSkillResp>()
        .await
    }

    pub async fn delete(
        &self,
        agent_skill_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/agent_skills/{agent_skill_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn get(
        &self,
        agent_skill_id: &str,
        option: &RequestOption,
    ) -> Result<GetAgentSkillResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/agent_skills/{agent_skill_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<GetAgentSkillRespData, GetAgentSkillResp>()
        .await
    }

    pub async fn list(&self, option: &RequestOption) -> Result<ListAgentSkillResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/helpdesk/v1/agent_skills",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<ListAgentSkillRespData, ListAgentSkillResp>()
        .await
    }

    pub async fn patch(
        &self,
        agent_skill_id: &str,
        body: &PatchAgentSkillReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/helpdesk/v1/agent_skills/{agent_skill_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }
}

// ── AgentSkillRule resource ──

pub struct AgentSkillRuleResource<'a> {
    config: &'a Config,
}

impl<'a> AgentSkillRuleResource<'a> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListAgentSkillRuleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/helpdesk/v1/agent_skill_rules",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<ListAgentSkillRuleRespData, ListAgentSkillRuleResp>()
        .await
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
    ) -> Result<CreateBotMessageResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/helpdesk/v1/message",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_response::<CreateBotMessageRespData, CreateBotMessageResp>()
        .await
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
    ) -> Result<EmptyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/helpdesk/v1/events/subscribe",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn unsubscribe(
        &self,
        body: &UnsubscribeEventReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/helpdesk/v1/events/unsubscribe",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
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
    ) -> Result<EmptyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/helpdesk/v1/ticket_customized_fields",
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn delete(
        &self,
        ticket_customized_field_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path =
            format!("/open-apis/helpdesk/v1/ticket_customized_fields/{ticket_customized_field_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn get(
        &self,
        ticket_customized_field_id: &str,
        option: &RequestOption,
    ) -> Result<GetTicketCustomizedFieldResp, LarkError> {
        let path =
            format!("/open-apis/helpdesk/v1/ticket_customized_fields/{ticket_customized_field_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<GetTicketCustomizedFieldRespData, GetTicketCustomizedFieldResp>()
        .await
    }

    pub async fn list(
        &self,
        page_token: Option<&str>,
        page_size: Option<i32>,
        body: Option<&ListTicketCustomizedFieldReqBody>,
        option: &RequestOption,
    ) -> Result<ListTicketCustomizedFieldResp, LarkError> {
        let mut request = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/helpdesk/v1/ticket_customized_fields",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("page_token", page_token)
        .query("page_size", page_size);
        if let Some(b) = body {
            request = request.json_body(b)?;
        }
        request
            .send_response::<ListTicketCustomizedFieldRespData, ListTicketCustomizedFieldResp>()
            .await
    }

    pub async fn patch(
        &self,
        ticket_customized_field_id: &str,
        body: &TicketCustomizedField,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path =
            format!("/open-apis/helpdesk/v1/ticket_customized_fields/{ticket_customized_field_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub ticket: TicketResource<'a>,
    pub ticket_message: TicketMessageResource<'a>,
    pub agent: AgentResource<'a>,
    pub agent_schedules: AgentSchedulesResource<'a>,
    pub agent_schedule: AgentScheduleResource<'a>,
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
            agent_schedules: AgentSchedulesResource { config },
            agent_schedule: AgentScheduleResource { config },
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
