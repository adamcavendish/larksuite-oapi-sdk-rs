use serde::Serialize;

use crate::config::Config;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::resp::{ApiResp, CodeError, RawResponse};
use crate::service::common::{FromRawResponse, RestRequest};

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
#[non_exhaustive]
pub struct RawTokenResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
}

impl RawTokenResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

impl<T> FromRawResponse<T> for RawTokenResp {
    fn from_raw_response(api_resp: ApiResp, raw: RawResponse<T>) -> Self {
        Self {
            api_resp,
            code_error: raw.code_error,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateAppAccessTokenQuery<'a> {
    pub body: &'a CreateAppAccessTokenReqBody,
}

impl<'a> CreateAppAccessTokenQuery<'a> {
    pub fn new(body: &'a CreateAppAccessTokenReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InternalAppAccessTokenQuery<'a> {
    pub body: &'a InternalAppAccessTokenReqBody,
}

impl<'a> InternalAppAccessTokenQuery<'a> {
    pub fn new(body: &'a InternalAppAccessTokenReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ResendAppTicketQuery<'a> {
    pub body: &'a ResendAppTicketReqBody,
}

impl<'a> ResendAppTicketQuery<'a> {
    pub fn new(body: &'a ResendAppTicketReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateTenantAccessTokenQuery<'a> {
    pub body: &'a CreateTenantAccessTokenReqBody,
}

impl<'a> CreateTenantAccessTokenQuery<'a> {
    pub fn new(body: &'a CreateTenantAccessTokenReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InternalTenantAccessTokenQuery<'a> {
    pub body: &'a InternalTenantAccessTokenReqBody,
}

impl<'a> InternalTenantAccessTokenQuery<'a> {
    pub fn new(body: &'a InternalTenantAccessTokenReqBody) -> Self {
        Self { body }
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
        self.create_by_query(&CreateAppAccessTokenQuery::new(body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateAppAccessTokenQuery<'_>,
        option: &RequestOption,
    ) -> Result<RawTokenResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/auth/v3/app_access_token",
            vec![],
            option,
        )
        .json_body(query.body)?
        .send_response::<(), RawTokenResp>()
        .await
    }

    /// Get app_access_token for internal (self-built) apps.
    pub async fn internal(
        &self,
        body: &InternalAppAccessTokenReqBody,
        option: &RequestOption,
    ) -> Result<RawTokenResp, LarkError> {
        self.internal_by_query(&InternalAppAccessTokenQuery::new(body), option)
            .await
    }

    pub async fn internal_by_query(
        &self,
        query: &InternalAppAccessTokenQuery<'_>,
        option: &RequestOption,
    ) -> Result<RawTokenResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/auth/v3/app_access_token/internal",
            vec![],
            option,
        )
        .json_body(query.body)?
        .send_response::<(), RawTokenResp>()
        .await
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
        self.resend_by_query(&ResendAppTicketQuery::new(body), option)
            .await
    }

    pub async fn resend_by_query(
        &self,
        query: &ResendAppTicketQuery<'_>,
        option: &RequestOption,
    ) -> Result<RawTokenResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/auth/v3/app_ticket/resend",
            vec![],
            option,
        )
        .json_body(query.body)?
        .send_response::<(), RawTokenResp>()
        .await
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
        self.create_by_query(&CreateTenantAccessTokenQuery::new(body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateTenantAccessTokenQuery<'_>,
        option: &RequestOption,
    ) -> Result<RawTokenResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/auth/v3/tenant_access_token",
            vec![],
            option,
        )
        .json_body(query.body)?
        .send_response::<(), RawTokenResp>()
        .await
    }

    /// Get tenant_access_token for internal (self-built) apps.
    pub async fn internal(
        &self,
        body: &InternalTenantAccessTokenReqBody,
        option: &RequestOption,
    ) -> Result<RawTokenResp, LarkError> {
        self.internal_by_query(&InternalTenantAccessTokenQuery::new(body), option)
            .await
    }

    pub async fn internal_by_query(
        &self,
        query: &InternalTenantAccessTokenQuery<'_>,
        option: &RequestOption,
    ) -> Result<RawTokenResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/auth/v3/tenant_access_token/internal",
            vec![],
            option,
        )
        .json_body(query.body)?
        .send_response::<(), RawTokenResp>()
        .await
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
