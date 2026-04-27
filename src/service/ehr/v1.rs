use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, RequestOption};
use crate::resp::ApiResp;
use crate::transport;

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
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let mut opt = option.clone();
        opt.file_download = true;
        let api_resp = transport::request(self.config, &api_req, &opt).await?;
        let file_name = api_resp.file_name_by_header();
        let data = api_resp.raw_body.clone();
        Ok(GetAttachmentResp {
            api_resp,
            file_name,
            data,
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
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/ehr/v1/employees");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = view {
            api_req.query_params.set("view", v);
        }
        if let Some(statuses) = status {
            for s in statuses {
                api_req.query_params.add("status", s.to_string());
            }
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<EmployeeListData>(self.config, &api_req, option).await?;
        Ok(ListEhrEmployeeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
