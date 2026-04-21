use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError, RawResponse};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_variable: Option<serde_json::Value>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCardInstanceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_variable: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateCardInstanceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_variable: Option<serde_json::Value>,
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

fn parse_v2<T>(api_resp: ApiResp, raw: RawResponse<T>) -> (ApiResp, Option<CodeError>, Option<T>) {
    if raw.code_error.code != 0 {
        (api_resp, Some(raw.code_error), None)
    } else {
        (api_resp, None, raw.data)
    }
}

macro_rules! impl_resp_v2 {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: Option<CodeError>,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.as_ref().is_none_or(|e| e.code == 0)
            }
        }
    };
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardInstanceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

impl_resp!(CreateCardInstanceResp, CardInstanceData);

impl_resp_v2!(CreateCardResp, serde_json::Value);
impl_resp_v2!(UpdateCardResp, serde_json::Value);
impl_resp_v2!(BatchUpdateCardResp, serde_json::Value);
impl_resp_v2!(IdConvertCardResp, serde_json::Value);
impl_resp_v2!(SettingsCardResp, serde_json::Value);
impl_resp_v2!(CreateCardElementResp, serde_json::Value);
impl_resp_v2!(UpdateCardElementResp, serde_json::Value);
impl_resp_v2!(DeleteCardElementResp, ());
impl_resp_v2!(PatchCardElementResp, serde_json::Value);
impl_resp_v2!(ContentCardElementResp, serde_json::Value);

// ── Resources ──

pub struct CardInstanceResource<'a> {
    config: &'a Config,
}

impl<'a> CardInstanceResource<'a> {
    pub async fn create(
        &self,
        body: &CreateCardInstanceReqBody,
        option: &RequestOption,
    ) -> Result<CreateCardInstanceResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/cardkit/v1/card_instances");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CardInstanceData>(self.config, &api_req, option).await?;
        Ok(CreateCardInstanceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        instance_id: &str,
        body: &UpdateCardInstanceReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/cardkit/v1/card_instances/{instance_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
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

pub struct CardResource<'a> {
    config: &'a Config,
}

impl CardResource<'_> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateCardResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/cardkit/v1/cards");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateCardResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        card_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateCardResp> {
        let path = format!("/open-apis/cardkit/v1/cards/{card_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateCardResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_update(
        &self,
        card_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateCardResp> {
        let path = format!("/open-apis/cardkit/v1/cards/{card_id}/batch_update");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchUpdateCardResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn id_convert(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<IdConvertCardResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/cardkit/v1/cards/id_convert");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(IdConvertCardResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn settings(
        &self,
        card_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<SettingsCardResp> {
        let path = format!("/open-apis/cardkit/v1/cards/{card_id}/settings");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SettingsCardResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct CardElementResource<'a> {
    config: &'a Config,
}

impl CardElementResource<'_> {
    pub async fn create(
        &self,
        card_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateCardElementResp> {
        let path = format!("/open-apis/cardkit/v1/cards/{card_id}/elements");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateCardElementResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        card_id: &str,
        element_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateCardElementResp> {
        let path = format!("/open-apis/cardkit/v1/cards/{card_id}/elements/{element_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateCardElementResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        card_id: &str,
        element_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteCardElementResp> {
        let path = format!("/open-apis/cardkit/v1/cards/{card_id}/elements/{element_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteCardElementResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        card_id: &str,
        element_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchCardElementResp> {
        let path = format!("/open-apis/cardkit/v1/cards/{card_id}/elements/{element_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchCardElementResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn content(
        &self,
        card_id: &str,
        element_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<ContentCardElementResp> {
        let path = format!("/open-apis/cardkit/v1/cards/{card_id}/elements/{element_id}/content");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ContentCardElementResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub card_instance: CardInstanceResource<'a>,
    pub card: CardResource<'a>,
    pub card_element: CardElementResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            card_instance: CardInstanceResource { config },
            card: CardResource { config },
            card_element: CardElementResource { config },
        }
    }
}
