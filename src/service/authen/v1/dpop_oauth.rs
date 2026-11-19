use std::time::{Duration, UNIX_EPOCH};

use http::{StatusCode, header};
use serde::Deserialize;

use super::{AccessToken, AccessTokenRespData, OAuthTokenRequestBody};
use crate::constants::{OAUTH_TOKEN_URL_PATH, USER_AGENT};
use crate::dpop::{DPoPBinding, DPoPClock, DPoPKey, OffsetDPoPClock};
use crate::error::LarkError;
use crate::resp::{ApiResp, CodeError};

const HEARTBEAT_PATH: &str = "/accounts/status/heartbeat";
const CLOCK_SYNC_TIMEOUT: Duration = Duration::from_secs(5);
const ACCOUNTS_BODY_LIMIT: usize = 1024 * 1024;
const CLOCK_SKEW_CODE: i64 = 20153;
const INVALID_DPOP_PROOF: &str = "invalid_dpop_proof";

/// A refreshed user token and its binding to the original DPoP key.
#[non_exhaustive]
pub struct DPoPRefreshResp {
    /// Token data returned by Accounts. Keep the previous refresh token if
    /// `refresh_token` is absent.
    pub data: AccessTokenRespData,
    pub binding: DPoPBinding,
}

impl std::fmt::Debug for DPoPRefreshResp {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DPoPRefreshResp")
            .field("binding", &self.binding)
            .finish_non_exhaustive()
    }
}

impl AccessToken<'_> {
    /// Refresh an existing DPoP-bound user token through Accounts OAuth.
    ///
    /// Supply the original signing key and a clock scoped to the trusted
    /// Accounts destination. Serialize refreshes for each credential and
    /// persist the returned tokens together with the same key identity.
    /// The SDK signs a fresh proof for each send and retries once only after
    /// an explicit proof rejection with an acceptable server timestamp.
    pub async fn refresh_dpop(
        &self,
        refresh_token: &str,
        key: &DPoPKey,
        clock: &OffsetDPoPClock,
    ) -> Result<DPoPRefreshResp, LarkError> {
        if refresh_token.trim().is_empty() {
            return Err(LarkError::IllegalParam("refresh token is empty".into()));
        }
        let grant = OAuthTokenRequestBody {
            grant_type: "refresh_token".into(),
            client_id: Some(self.config.app_id.clone()),
            client_secret: Some(self.require_app_secret()?.into()),
            refresh_token: Some(refresh_token.into()),
            ..Default::default()
        };
        let endpoint = self.accounts_dpop_url(OAUTH_TOKEN_URL_PATH)?;

        let (response, token) = self
            .send_dpop_refresh(&endpoint, &grant, key, clock)
            .await?;
        let (response, token) =
            if token.is_clock_rejection() && adjust_from_date(clock, &response.header) {
                self.send_dpop_refresh(&endpoint, &grant, key, clock)
                    .await?
            } else {
                (response, token)
            };
        token.into_binding(response.status_code, key)
    }

    /// Calibrate a proof clock with the configured Accounts heartbeat.
    ///
    /// The POST carries no credentials and has a deadline of at most five
    /// seconds. The clock keeps its prior offset if calibration fails.
    pub async fn sync_dpop_clock(&self, clock: &OffsetDPoPClock) -> Result<(), LarkError> {
        let endpoint = self.accounts_dpop_url(HEARTBEAT_PATH)?;
        let timeout = self.config.req_timeout.min(CLOCK_SYNC_TIMEOUT);
        let response = self.accounts_post(&endpoint, None, timeout).await?;
        if response.status_code != StatusCode::OK.as_u16() {
            return Err(accounts_status_error("heartbeat", response.status_code));
        }

        let heartbeat: HeartbeatResponse = serde_json::from_slice(&response.raw_body)?;
        if heartbeat.code != 0 {
            return Err(CodeError {
                code: heartbeat.code,
                msg: "Accounts heartbeat failed".into(),
                ..Default::default()
            }
            .into());
        }
        let server_time = heartbeat
            .data
            .and_then(|data| data.now.parse::<i64>().ok())
            .filter(|&timestamp| timestamp > 0)
            .ok_or_else(|| LarkError::Token("Accounts heartbeat returned invalid time".into()))?;
        clock.adjust_to_server_time(server_time)?;
        Ok(())
    }

    fn accounts_dpop_url(&self, path: &str) -> Result<String, LarkError> {
        let base_url = crate::token::resolve_oauth_base_url(self.config);
        let base = url::Url::parse(&base_url)
            .ok()
            .filter(|url| {
                matches!(url.scheme(), "http" | "https")
                    && url.host_str().is_some()
                    && url.username().is_empty()
                    && url.password().is_none()
                    && url.query().is_none()
                    && url.fragment().is_none()
            })
            .ok_or_else(|| LarkError::IllegalParam("invalid Accounts OAuth base URL".into()))?;
        let logical_url = format!("{}{path}", base.as_str().trim_end_matches('/'));
        crate::url_resolver::resolve(self.config.platform_url_resolver.as_ref(), &logical_url)
    }

    async fn send_dpop_refresh(
        &self,
        endpoint: &str,
        grant: &OAuthTokenRequestBody,
        key: &DPoPKey,
        clock: &OffsetDPoPClock,
    ) -> Result<(ApiResp, AccountsTokenResponse), LarkError> {
        let proof = key.proof("POST", endpoint, clock.now_unix_seconds()?)?;
        let response = self
            .accounts_post(endpoint, Some((grant, &proof)), self.config.req_timeout)
            .await?;
        let status = response.status_code;
        if !matches!(status, 200 | 400 | 401 | 403) {
            return Err(accounts_status_error("DPoP refresh", status));
        }
        let token = serde_json::from_slice(&response.raw_body)?;
        Ok((response, token))
    }

    async fn accounts_post(
        &self,
        endpoint: &str,
        signed_grant: Option<(&OAuthTokenRequestBody, &str)>,
        timeout: Duration,
    ) -> Result<ApiResp, LarkError> {
        let mut request = self.config.accounts_http_client.post(endpoint)?.header(
            header::USER_AGENT,
            header::HeaderValue::from_static(USER_AGENT),
        );
        if let Some((grant, proof)) = signed_grant {
            request = request
                .header_str("DPoP", proof)?
                .header(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static("application/json; charset=utf-8"),
                )
                .json(grant)?;
        }

        let send = async {
            let response = request.send().await.map_err(|error| error.into_error())?;
            let status_code = response.status().as_u16();
            let header = response.headers().clone();
            let mut stream = response.into_bytes_stream();
            let mut raw_body = Vec::new();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                if chunk.len() > ACCOUNTS_BODY_LIMIT - raw_body.len() {
                    return Err(LarkError::Token("Accounts response exceeded 1 MiB".into()));
                }
                raw_body.extend_from_slice(&chunk);
            }
            Ok(ApiResp {
                status_code,
                header,
                raw_body,
            })
        };
        match tokio::time::timeout(timeout, send).await {
            Err(_) | Ok(Err(LarkError::Http(aioduct::Error::Timeout))) => Err(
                LarkError::ClientTimeout("Accounts request timed out".into()),
            ),
            Ok(result) => result,
        }
    }
}

#[derive(Deserialize)]
struct AccountsTokenResponse {
    #[serde(default)]
    code: i64,
    #[serde(default)]
    error: String,
    #[serde(flatten)]
    data: AccessTokenRespData,
}

impl AccountsTokenResponse {
    fn is_clock_rejection(&self) -> bool {
        self.code == CLOCK_SKEW_CODE
            && self.error == INVALID_DPOP_PROOF
            && self.data.access_token.as_deref().is_none_or(str::is_empty)
            && self.data.refresh_token.as_deref().is_none_or(str::is_empty)
    }

    fn rejected(&self, status: u16) -> LarkError {
        let error = match self.error.as_str() {
            INVALID_DPOP_PROOF
            | "invalid_grant"
            | "invalid_client"
            | "invalid_request"
            | "unauthorized_client"
            | "unsupported_grant_type"
            | "invalid_scope" => &self.error,
            _ => "OAuth rejection",
        };
        CodeError {
            code: self.code,
            msg: format!("Accounts DPoP refresh failed (HTTP {status}, {error})"),
            ..Default::default()
        }
        .into()
    }

    fn into_binding(mut self, status: u16, key: &DPoPKey) -> Result<DPoPRefreshResp, LarkError> {
        if status != 200 || self.code != 0 || !self.error.is_empty() {
            return Err(self.rejected(status));
        }
        if !self
            .data
            .token_type
            .as_deref()
            .is_some_and(|value| value.eq_ignore_ascii_case("DPoP"))
        {
            return Err(LarkError::DPoPBinding(
                "Accounts refresh did not return a DPoP token".into(),
            ));
        }
        let token = self.data.access_token.as_deref().unwrap_or_default();
        let binding = DPoPBinding::new(token, key.clone())?;
        self.data.refresh_token = self.data.refresh_token.filter(|token| !token.is_empty());
        Ok(DPoPRefreshResp {
            data: self.data,
            binding,
        })
    }
}

fn adjust_from_date(clock: &OffsetDPoPClock, headers: &http::HeaderMap) -> bool {
    let mut dates = headers.get_all(header::DATE).iter();
    let Some(date) = dates.next().filter(|_| dates.next().is_none()) else {
        return false;
    };
    let timestamp = date
        .to_str()
        .ok()
        .and_then(|date| httpdate::parse_http_date(date).ok())
        .and_then(|date| date.duration_since(UNIX_EPOCH).ok())
        .and_then(|time| i64::try_from(time.as_secs()).ok())
        .filter(|&time| time > 0);
    timestamp.is_some_and(|time| clock.adjust_to_server_time(time).is_ok())
}

fn accounts_status_error(operation: &str, status: u16) -> LarkError {
    let message = format!("Accounts {operation} failed (HTTP {status})");
    match status {
        429 => LarkError::RateLimited(message),
        504 => LarkError::ServerTimeout(message),
        _ => LarkError::Token(message),
    }
}

#[derive(Deserialize)]
struct HeartbeatResponse {
    code: i64,
    data: Option<HeartbeatData>,
}

#[derive(Deserialize)]
struct HeartbeatData {
    now: String,
}
