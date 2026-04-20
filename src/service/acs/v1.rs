use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcsUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_clock_in: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_door_open: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcsDevice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_sn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub door_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub door_name: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateUserFaceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
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
pub struct AcsUserData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<AcsUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcsUserListData {
    #[serde(default)]
    pub items: Vec<AcsUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessRecordListData {
    #[serde(default)]
    pub items: Vec<AccessRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceListData {
    #[serde(default)]
    pub items: Vec<AcsDevice>,
}

impl_resp!(GetAcsUserResp, AcsUserData);
impl_resp!(ListAcsUserResp, AcsUserListData);
impl_resp!(ListAccessRecordResp, AccessRecordListData);
impl_resp!(ListDeviceResp, DeviceListData);

// ── Resources ──

pub struct AcsUserResource<'a> {
    config: &'a Config,
}

impl<'a> AcsUserResource<'a> {
    pub async fn get(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetAcsUserResp> {
        let path = format!("/open-apis/acs/v1/users/{user_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<AcsUserData>(self.config, &api_req, option).await?;
        Ok(GetAcsUserResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAcsUserResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/acs/v1/users");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<AcsUserListData>(self.config, &api_req, option).await?;
        Ok(ListAcsUserResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        user_id: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/acs/v1/users/{user_id}");
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
}

pub struct AccessRecordResource<'a> {
    config: &'a Config,
}

impl<'a> AccessRecordResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        from: Option<i64>,
        to: Option<i64>,
        device_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAccessRecordResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/acs/v1/access_records");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = from {
            api_req.query_params.set("from", v.to_string());
        }
        if let Some(v) = to {
            api_req.query_params.set("to", v.to_string());
        }
        if let Some(v) = device_id {
            api_req.query_params.set("device_id", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<AccessRecordListData>(self.config, &api_req, option).await?;
        Ok(ListAccessRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct DeviceResource<'a> {
    config: &'a Config,
}

impl<'a> DeviceResource<'a> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListDeviceResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/acs/v1/devices");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<DeviceListData>(self.config, &api_req, option).await?;
        Ok(ListDeviceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub user: AcsUserResource<'a>,
    pub access_record: AccessRecordResource<'a>,
    pub device: DeviceResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            user: AcsUserResource { config },
            access_record: AccessRecordResource { config },
            device: DeviceResource { config },
        }
    }
}
