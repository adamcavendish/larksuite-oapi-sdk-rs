/// Base URL for Feishu (China) API endpoints.
pub const FEISHU_BASE_URL: &str = "https://open.feishu.cn";
/// Base URL for Lark (international) API endpoints.
pub const LARK_BASE_URL: &str = "https://open.larksuite.com";

pub const USER_AGENT: &str = concat!("oapi-sdk-rust/v", env!("CARGO_PKG_VERSION"));

pub const HTTP_HEADER_KEY_REQUEST_ID: &str = "X-Request-Id";
pub const HTTP_HEADER_REQUEST_ID: &str = "Request-Id";
pub const HTTP_HEADER_KEY_LOG_ID: &str = "X-Tt-Logid";
pub const CUSTOM_REQUEST_ID: &str = "Oapi-Sdk-Request-Id";
pub const HEADER_X_TARGET_SERVICE: &str = "X-Target-Service";

pub const APP_ACCESS_TOKEN_INTERNAL_URL_PATH: &str = "/open-apis/auth/v3/app_access_token/internal";
pub const APP_ACCESS_TOKEN_URL_PATH: &str = "/open-apis/auth/v3/app_access_token";
pub const TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH: &str =
    "/open-apis/auth/v3/tenant_access_token/internal";
pub const TENANT_ACCESS_TOKEN_URL_PATH: &str = "/open-apis/auth/v3/tenant_access_token";
pub const OAUTH_TOKEN_URL_PATH: &str = "/oauth/v3/token";
pub const APPLY_APP_TICKET_PATH: &str = "/open-apis/auth/v3/app_ticket/resend";

pub const ERR_CODE_APP_TICKET_INVALID: i64 = 10012;
pub const ERR_CODE_ACCESS_TOKEN_INVALID: i64 = 99991671;
pub const ERR_CODE_APP_ACCESS_TOKEN_INVALID: i64 = 99991664;
pub const ERR_CODE_TENANT_ACCESS_TOKEN_INVALID: i64 = 99991663;
pub const ERR_CODE_CLIENT_ASSERTION_NOT_CONFIGURED: i64 = 7100;
pub const ERR_CODE_CLIENT_ASSERTION_TOKEN_EMPTY: i64 = 7101;
pub const ERR_CODE_CLIENT_ASSERTION_RETRIEVE_FAILED: i64 = 7102;
pub const ERR_CODE_CLIENT_ASSERTION_MODE_NOT_SUPPORTED: i64 = 7103;

pub const APP_TICKET_KEY_PREFIX: &str = "app_ticket";
pub const APP_ACCESS_TOKEN_KEY_PREFIX: &str = "app_access_token";
pub const TENANT_ACCESS_TOKEN_KEY_PREFIX: &str = "tenant_access_token";

pub const GRANT_TYPE_JWT_BEARER: &str = "urn:ietf:params:oauth:grant-type:jwt-bearer";
pub const CLIENT_ASSERTION_TYPE_JWT_BEARER: &str =
    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer";

/// Default OAuth base URL for Feishu (China).
pub const FEISHU_OAUTH_BASE_URL: &str = "https://accounts.feishu.cn";
/// Default OAuth base URL for Lark (international).
pub const LARK_OAUTH_BASE_URL: &str = "https://accounts.larksuite.com";

pub const EXPIRY_DELTA_SECONDS: u64 = 180;

/// Self-built (internal) or marketplace (ISV) application type.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum AppType {
    #[default]
    SelfBuilt,
    Marketplace,
}

impl std::fmt::Display for AppType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SelfBuilt => write!(f, "SelfBuilt"),
            Self::Marketplace => write!(f, "Marketplace"),
        }
    }
}

/// Access token type used for API authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AccessTokenType {
    None,
    App,
    Tenant,
    User,
}

impl std::fmt::Display for AccessTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none_access_token"),
            Self::App => write!(f, "app_access_token"),
            Self::Tenant => write!(f, "tenant_access_token"),
            Self::User => write!(f, "user_access_token"),
        }
    }
}
