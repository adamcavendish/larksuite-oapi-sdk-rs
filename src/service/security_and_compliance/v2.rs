use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceRecordData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_record: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceRecordListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

macro_rules! impl_resp {
    ($name:ident, $data:ty) => {
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: Option<CodeError>,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.api_resp.status_code == 200
                    && self.code_error.as_ref().map_or(true, |e| e.code == 0)
            }
        }
    };
}

impl_resp!(UpdateDeviceApplyRecordV2Resp, serde_json::Value);
impl_resp!(CreateDeviceRecordV2Resp, DeviceRecordData);
impl_resp!(DeleteDeviceRecordV2Resp, ());
impl_resp!(GetDeviceRecordV2Resp, DeviceRecordData);
impl_resp!(ListDeviceRecordV2Resp, DeviceRecordListData);
impl_resp!(MineDeviceRecordV2Resp, DeviceRecordListData);
impl_resp!(UpdateDeviceRecordV2Resp, DeviceRecordData);

fn parse<T: for<'de> serde::Deserialize<'de>>(
    api_resp: ApiResp,
    raw: crate::resp::RawResponse<T>,
) -> impl FnOnce() -> (ApiResp, Option<CodeError>, Option<T>) {
    move || {
        let code_error = if raw.code_error.code != 0 {
            Some(raw.code_error)
        } else {
            None
        };
        (api_resp, code_error, raw.data)
    }
}

pub struct V2<'a> {
    pub device_apply_record: DeviceApplyRecordV2Resource<'a>,
    pub device_record: DeviceRecordV2Resource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            device_apply_record: DeviceApplyRecordV2Resource { config },
            device_record: DeviceRecordV2Resource { config },
        }
    }
}

pub struct DeviceApplyRecordV2Resource<'a> {
    config: &'a Config,
}

impl DeviceApplyRecordV2Resource<'_> {
    pub async fn update(
        &self,
        device_apply_record_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateDeviceApplyRecordV2Resp> {
        let path = format!(
            "/open-apis/security_and_compliance/v2/device_apply_records/{device_apply_record_id}"
        );
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw)();
        Ok(UpdateDeviceApplyRecordV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct DeviceRecordV2Resource<'a> {
    config: &'a Config,
}

impl DeviceRecordV2Resource<'_> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateDeviceRecordV2Resp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/security_and_compliance/v2/device_records",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<DeviceRecordData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw)();
        Ok(CreateDeviceRecordV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        device_record_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteDeviceRecordV2Resp> {
        let path =
            format!("/open-apis/security_and_compliance/v2/device_records/{device_record_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw)();
        Ok(DeleteDeviceRecordV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        device_record_id: &str,
        option: &RequestOption,
    ) -> Result<GetDeviceRecordV2Resp> {
        let path =
            format!("/open-apis/security_and_compliance/v2/device_records/{device_record_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<DeviceRecordData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw)();
        Ok(GetDeviceRecordV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDeviceRecordV2Resp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/security_and_compliance/v2/device_records",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<DeviceRecordListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw)();
        Ok(ListDeviceRecordV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn mine(&self, option: &RequestOption) -> Result<MineDeviceRecordV2Resp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/security_and_compliance/v2/device_records/mine",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<DeviceRecordListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw)();
        Ok(MineDeviceRecordV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        device_record_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateDeviceRecordV2Resp> {
        let path =
            format!("/open-apis/security_and_compliance/v2/device_records/{device_record_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<DeviceRecordData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw)();
        Ok(UpdateDeviceRecordV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}
