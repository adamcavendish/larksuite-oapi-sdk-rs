use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::EmptyResp;
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MdmDevice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceListData {
    #[serde(default)]
    pub items: Vec<MdmDevice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(ListDeviceResp, DeviceListData);

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateDeviceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

// ── Resources ──

pub struct UserDeviceResource<'a> {
    config: &'a Config,
}

impl<'a> UserDeviceResource<'a> {
    pub async fn list(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDeviceResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/mdm/v1/user_devices");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("user_id", user_id);
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<DeviceListData>(self.config, &api_req, option).await?;
        Ok(ListDeviceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        device_id: &str,
        body: &UpdateDeviceReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/mdm/v1/user_devices/{device_id}");
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

pub struct UserAuthDataRelationResource<'a> {
    config: &'a Config,
}

impl<'a> UserAuthDataRelationResource<'a> {
    pub async fn bind(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/mdm/v1/user_auth_data_relations/bind",
        );
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

    pub async fn unbind(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/mdm/v1/user_auth_data_relations/unbind",
        );
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

// ── Version struct ──

pub struct V1<'a> {
    pub user_device: UserDeviceResource<'a>,
    pub user_auth_data_relation: UserAuthDataRelationResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            user_device: UserDeviceResource { config },
            user_auth_data_relation: UserAuthDataRelationResource { config },
        }
    }
}
