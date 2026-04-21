use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::{EmptyResp, parse_v2};
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

// ── New data types (v2 pattern) ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleExternalData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessRecordAccessPhotoData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserFaceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisitorData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visitor_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisitorListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp_v2!(CreateRuleExternalResp, RuleExternalData);
impl_resp_v2!(DeleteRuleExternalResp, ());
impl_resp_v2!(DeviceBindRuleExternalResp, serde_json::Value);
impl_resp_v2!(GetRuleExternalResp, RuleExternalData);
impl_resp_v2!(GetAccessRecordAccessPhotoResp, AccessRecordAccessPhotoData);
impl_resp_v2!(GetUserFaceResp, UserFaceData);
impl_resp_v2!(UpdateUserFaceResp, serde_json::Value);
impl_resp_v2!(CreateVisitorResp, VisitorData);
impl_resp_v2!(DeleteVisitorResp, ());
impl_resp_v2!(ListVisitorResp, VisitorListData);

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

// ── RuleExternal resource ──

pub struct RuleExternalResource<'a> {
    config: &'a Config,
}

impl RuleExternalResource<'_> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        rule_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateRuleExternalResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/acs/v1/rule_external");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = rule_id {
            api_req.query_params.set("rule_id", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<RuleExternalData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateRuleExternalResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        rule_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteRuleExternalResp> {
        let mut api_req = ApiReq::new(http::Method::DELETE, "/open-apis/acs/v1/rule_external");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.query_params.set("rule_id", rule_id);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteRuleExternalResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn device_bind(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<DeviceBindRuleExternalResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/acs/v1/rule_external/device_bind",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeviceBindRuleExternalResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        device_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetRuleExternalResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/acs/v1/rule_external");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = device_id {
            api_req.query_params.set("device_id", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<RuleExternalData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetRuleExternalResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── AccessRecordAccessPhoto resource ──

pub struct AccessRecordAccessPhotoResource<'a> {
    config: &'a Config,
}

impl AccessRecordAccessPhotoResource<'_> {
    pub async fn get(
        &self,
        access_record_id: &str,
        option: &RequestOption,
    ) -> Result<GetAccessRecordAccessPhotoResp> {
        let path = format!("/open-apis/acs/v1/access_records/{access_record_id}/access_photo");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<AccessRecordAccessPhotoData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetAccessRecordAccessPhotoResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── UserFace resource ──

pub struct UserFaceResource<'a> {
    config: &'a Config,
}

impl UserFaceResource<'_> {
    pub async fn get(
        &self,
        user_id: &str,
        is_cropped: Option<bool>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetUserFaceResp> {
        let path = format!("/open-apis/acs/v1/users/{user_id}/face");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = is_cropped {
            api_req.query_params.set("is_cropped", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<UserFaceData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetUserFaceResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        user_id: &str,
        body: &UpdateUserFaceReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateUserFaceResp> {
        let path = format!("/open-apis/acs/v1/users/{user_id}/face");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateUserFaceResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Visitor resource ──

pub struct VisitorResource<'a> {
    config: &'a Config,
}

impl VisitorResource<'_> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateVisitorResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/acs/v1/visitors");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<VisitorData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateVisitorResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        visitor_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<DeleteVisitorResp> {
        let path = format!("/open-apis/acs/v1/visitors/{visitor_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteVisitorResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub user: AcsUserResource<'a>,
    pub access_record: AccessRecordResource<'a>,
    pub access_record_access_photo: AccessRecordAccessPhotoResource<'a>,
    pub device: DeviceResource<'a>,
    pub rule_external: RuleExternalResource<'a>,
    pub user_face: UserFaceResource<'a>,
    pub visitor: VisitorResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            user: AcsUserResource { config },
            access_record: AccessRecordResource { config },
            access_record_access_photo: AccessRecordAccessPhotoResource { config },
            device: DeviceResource { config },
            rule_external: RuleExternalResource { config },
            user_face: UserFaceResource { config },
            visitor: VisitorResource { config },
        }
    }
}
