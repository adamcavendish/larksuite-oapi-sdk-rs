use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct QuerySessionReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LogoutSessionReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp_credential_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_type: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_reason: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionListData {
    #[serde(default)]
    pub session_list: Vec<SessionInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(QuerySessionResp, SessionListData);

#[derive(Debug, Clone, Copy)]
pub struct QuerySessionQuery<'a> {
    pub body: &'a QuerySessionReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> QuerySessionQuery<'a> {
    pub fn new(body: &'a QuerySessionReqBody) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LogoutSessionQuery<'a> {
    pub body: &'a LogoutSessionReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> LogoutSessionQuery<'a> {
    pub fn new(body: &'a LogoutSessionReqBody) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }
}

// ── Resources ──

pub struct SessionResource<'a> {
    config: &'a Config,
}

impl<'a> SessionResource<'a> {
    pub async fn query(
        &self,
        body: &QuerySessionReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<QuerySessionResp, LarkError> {
        self.query_by_query(
            &QuerySessionQuery::new(body).user_id_type(user_id_type),
            option,
        )
        .await
    }

    pub async fn query_by_query(
        &self,
        query: &QuerySessionQuery<'_>,
        option: &RequestOption,
    ) -> Result<QuerySessionResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/passport/v1/sessions/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<SessionListData>()
        .await?;
        Ok(QuerySessionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn logout(
        &self,
        body: &LogoutSessionReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        self.logout_by_query(
            &LogoutSessionQuery::new(body).user_id_type(user_id_type),
            option,
        )
        .await
    }

    pub async fn logout_by_query(
        &self,
        query: &LogoutSessionQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/passport/v1/sessions/logout",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<serde_json::Value>()
        .await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub session: SessionResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            session: SessionResource { config },
        }
    }
}
