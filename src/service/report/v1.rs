use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<ReportField>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_list: Option<Vec<crate::JsonValue>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_contents: Option<Vec<crate::JsonValue>>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryRuleReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct QueryTaskReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleListData {
    #[serde(default)]
    pub rules: Vec<ReportRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskListData {
    #[serde(default)]
    pub items: Vec<ReportTask>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(QueryRuleResp, RuleListData);
impl_resp!(QueryTaskResp, TaskListData);

#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveRuleViewReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

// -- Query parameter types --

#[derive(Debug, Clone, Copy, Default)]
pub struct QueryRuleQuery<'a> {
    pub rule_name: Option<&'a str>,
    pub include_deleted: Option<i32>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> QueryRuleQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rule_name(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.rule_name = value.into();
        self
    }

    pub fn include_deleted(mut self, value: impl Into<Option<i32>>) -> Self {
        self.include_deleted = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct QueryTaskQuery<'a> {
    pub body: &'a QueryTaskReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> QueryTaskQuery<'a> {
    pub fn new(body: &'a QueryTaskReqBody) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RemoveRuleViewQuery<'a> {
    pub rule_id: &'a str,
    pub body: &'a RemoveRuleViewReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> RemoveRuleViewQuery<'a> {
    pub fn new(rule_id: &'a str, body: &'a RemoveRuleViewReqBody) -> Self {
        Self {
            rule_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

// ── Resources ──

pub struct RuleResource<'a> {
    config: &'a Config,
}

impl<'a> RuleResource<'a> {
    pub async fn query(
        &self,
        rule_name: Option<&str>,
        include_deleted: Option<i32>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<QueryRuleResp, LarkError> {
        let query = QueryRuleQuery::new()
            .rule_name(rule_name)
            .include_deleted(include_deleted)
            .user_id_type(user_id_type);
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &QueryRuleQuery<'_>,
        option: &RequestOption,
    ) -> Result<QueryRuleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/report/v1/rules/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("rule_name", query.rule_name)
        .query("include_deleted", query.include_deleted)
        .query("user_id_type", query.user_id_type)
        .send_response::<RuleListData, QueryRuleResp>()
        .await
    }
}

pub struct TaskResource<'a> {
    config: &'a Config,
}

impl<'a> TaskResource<'a> {
    pub async fn query(
        &self,
        body: &QueryTaskReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<QueryTaskResp, LarkError> {
        let query = QueryTaskQuery::new(body).user_id_type(user_id_type);
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &QueryTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<QueryTaskResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/report/v1/tasks/query",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_response::<TaskListData, QueryTaskResp>()
        .await
    }
}

// ── Version struct ──

pub struct RuleViewResource<'a> {
    config: &'a Config,
}

impl<'a> RuleViewResource<'a> {
    pub async fn remove(
        &self,
        rule_id: &str,
        body: &RemoveRuleViewReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = RemoveRuleViewQuery::new(rule_id, body).user_id_type(user_id_type);
        self.remove_by_query(&query, option).await
    }

    pub async fn remove_by_query(
        &self,
        query: &RemoveRuleViewQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/report/v1/rules/{}/views/remove", query.rule_id);
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_empty()
        .await
    }
}

pub struct V1<'a> {
    pub rule: RuleResource<'a>,
    pub rule_view: RuleViewResource<'a>,
    pub task: TaskResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            rule: RuleResource { config },
            rule_view: RuleViewResource { config },
            task: TaskResource { config },
        }
    }
}
