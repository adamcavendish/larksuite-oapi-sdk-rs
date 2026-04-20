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
}

// ── Version struct ──

pub struct V1<'a> {
    pub ticket: TicketResource<'a>,
    pub ticket_message: TicketMessageResource<'a>,
    pub agent: AgentResource<'a>,
    pub category: CategoryResource<'a>,
    pub faq: FaqResource<'a>,
    pub notification: NotificationResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            ticket: TicketResource { config },
            ticket_message: TicketMessageResource { config },
            agent: AgentResource { config },
            category: CategoryResource { config },
            faq: FaqResource { config },
            notification: NotificationResource { config },
        }
    }
}
