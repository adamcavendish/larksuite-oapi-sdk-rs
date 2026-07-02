use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::resp::ApiResp;
use crate::service::common::{PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EhrEmployee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_fields: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<serde_json::Value>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeListData {
    #[serde(default)]
    pub items: Vec<EhrEmployee>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(ListEhrEmployeeResp, EmployeeListData);

#[derive(Debug, Clone)]
pub struct GetAttachmentResp {
    pub api_resp: ApiResp,
    pub file_name: Option<String>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListEmployeeQuery<'a> {
    pub user_id_type: Option<&'a str>,
    pub view: Option<&'a str>,
    pub status: Option<&'a [i32]>,
    pub page: PageQuery<'a>,
}

impl<'a> ListEmployeeQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn view(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.view = value.into();
        self
    }

    pub fn status(mut self, value: impl Into<Option<&'a [i32]>>) -> Self {
        self.status = value.into();
        self
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

// ── Resources ──

pub struct AttachmentResource<'a> {
    config: &'a Config,
}

impl<'a> AttachmentResource<'a> {
    pub async fn get(
        &self,
        token: &str,
        option: &RequestOption,
    ) -> Result<GetAttachmentResp, LarkError> {
        let path = format!("/open-apis/ehr/v1/attachments/{token}");
        let download = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .download()
        .await?;
        Ok(GetAttachmentResp {
            api_resp: download.api_resp,
            file_name: download.file_name,
            data: download.data,
        })
    }
}

pub struct EmployeeResource<'a> {
    config: &'a Config,
}

impl<'a> EmployeeResource<'a> {
    pub async fn list(
        &self,
        user_id_type: Option<&str>,
        view: Option<&str>,
        status: Option<Vec<i32>>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListEhrEmployeeResp, LarkError> {
        let query = ListEmployeeQuery::new()
            .user_id_type(user_id_type)
            .view(view)
            .status(status.as_deref())
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListEmployeeQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListEhrEmployeeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/ehr/v1/employees",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("view", query.view)
        .query_values("status", query.status)
        .page_query(query.page)
        .send_response::<EmployeeListData, ListEhrEmployeeResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub attachment: AttachmentResource<'a>,
    pub employee: EmployeeResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            attachment: AttachmentResource { config },
            employee: EmployeeResource { config },
        }
    }
}
