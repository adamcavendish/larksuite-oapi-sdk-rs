use serde::Serialize;

use crate::config::Config;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAppAccessTokenReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ticket: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct InternalAppAccessTokenReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_secret: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ResendAppTicketReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_secret: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTenantAccessTokenReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct InternalTenantAccessTokenReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_secret: Option<String>,
}

// ── Response types (no data wrapper — token returned at top level) ──

#[derive(Debug, Clone)]
pub struct RawTokenResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
}

impl RawTokenResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

// ── Resources ──

pub struct AppAccessTokenResource<'a> {
    config: &'a Config,
}

impl<'a> AppAccessTokenResource<'a> {
    /// Exchange app_ticket for ISV app_access_token.
    pub async fn create(
        &self,
        body: &CreateAppAccessTokenReqBody,
        option: &RequestOption,
    ) -> Result<RawTokenResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/auth/v3/app_access_token");
        api_req.supported_access_token_types = vec![];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(RawTokenResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    /// Get app_access_token for internal (self-built) apps.
    pub async fn internal(
        &self,
        body: &InternalAppAccessTokenReqBody,
        option: &RequestOption,
    ) -> Result<RawTokenResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/auth/v3/app_access_token/internal",
        );
        api_req.supported_access_token_types = vec![];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(RawTokenResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct AppTicketResource<'a> {
    config: &'a Config,
}

impl<'a> AppTicketResource<'a> {
    /// Re-push app_ticket to the ISV application.
    pub async fn resend(
        &self,
        body: &ResendAppTicketReqBody,
        option: &RequestOption,
    ) -> Result<RawTokenResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/auth/v3/app_ticket/resend");
        api_req.supported_access_token_types = vec![];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(RawTokenResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct TenantAccessTokenResource<'a> {
    config: &'a Config,
}

impl<'a> TenantAccessTokenResource<'a> {
    /// Exchange app_access_token for ISV tenant_access_token.
    pub async fn create(
        &self,
        body: &CreateTenantAccessTokenReqBody,
        option: &RequestOption,
    ) -> Result<RawTokenResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/auth/v3/tenant_access_token");
        api_req.supported_access_token_types = vec![];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(RawTokenResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    /// Get tenant_access_token for internal (self-built) apps.
    pub async fn internal(
        &self,
        body: &InternalTenantAccessTokenReqBody,
        option: &RequestOption,
    ) -> Result<RawTokenResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/auth/v3/tenant_access_token/internal",
        );
        api_req.supported_access_token_types = vec![];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(RawTokenResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

// ── Version struct ──

pub struct V3<'a> {
    pub app_access_token: AppAccessTokenResource<'a>,
    pub app_ticket: AppTicketResource<'a>,
    pub tenant_access_token: TenantAccessTokenResource<'a>,
}

impl<'a> V3<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            app_access_token: AppAccessTokenResource { config },
            app_ticket: AppTicketResource { config },
            tenant_access_token: TenantAccessTokenResource { config },
        }
    }
}
