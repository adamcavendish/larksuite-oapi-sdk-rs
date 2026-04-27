//! OAuth app registration via device-code flow.
//!
//! Mirrors the Go SDK's `scene/registration` package. Drives the
//! `/oauth/v1/app/registration` endpoint to create a new app by
//! having the user scan a QR code.

use serde::Deserialize;

use crate::error::LarkError;

const SDK_NAME: &str = "rust-sdk";
const DEFAULT_FEISHU_DOMAIN: &str = "https://accounts.feishu.cn";
const DEFAULT_LARK_DOMAIN: &str = "https://accounts.larksuite.com";
const ENDPOINT: &str = "/oauth/v1/app/registration";
const DEFAULT_POLL_INTERVAL_SECS: u64 = 5;
const DEFAULT_EXPIRE_IN_SECS: u64 = 600;

pub const STATUS_POLLING: &str = "polling";
pub const STATUS_SLOW_DOWN: &str = "slow_down";
pub const STATUS_DOMAIN_SWITCHED: &str = "domain_switched";

#[derive(Debug, Clone)]
pub struct QRCodeInfo {
    pub url: String,
    pub expire_in: u64,
}

#[derive(Debug, Clone)]
pub struct StatusChangeInfo {
    pub status: String,
    pub interval: u64,
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub open_id: String,
    pub tenant_brand: String,
}

#[derive(Debug, Clone)]
pub struct RegisterAppResult {
    pub client_id: String,
    pub client_secret: String,
    pub user_info: Option<UserInfo>,
}

pub type OnQRCode = Box<dyn Fn(&QRCodeInfo) + Send + Sync>;
pub type OnStatusChange = Box<dyn Fn(&StatusChangeInfo) + Send + Sync>;

pub struct Options {
    pub source: String,
    pub domain: String,
    pub lark_domain: String,
    pub on_qr_code: OnQRCode,
    pub on_status_change: Option<OnStatusChange>,
}

#[derive(Debug, Clone)]
pub struct RegisterAppError {
    pub code: String,
    pub description: String,
}

impl std::fmt::Display for RegisterAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "register app error: {}: {}", self.code, self.description)
    }
}

impl std::error::Error for RegisterAppError {}

#[derive(Deserialize)]
struct BeginResponse {
    device_code: String,
    verification_uri_complete: String,
    #[serde(default)]
    interval: u64,
    #[serde(default)]
    expire_in: u64,
}

#[derive(Deserialize)]
struct PollResponse {
    #[serde(default)]
    client_id: String,
    #[serde(default)]
    client_secret: String,
    #[serde(default)]
    user_info: Option<PollUserInfo>,
    #[serde(default)]
    error: String,
    #[serde(default)]
    error_description: String,
}

#[derive(Deserialize)]
struct PollUserInfo {
    #[serde(default)]
    open_id: String,
    #[serde(default)]
    tenant_brand: String,
}

fn normalize_interval(interval: u64) -> u64 {
    if interval == 0 {
        DEFAULT_POLL_INTERVAL_SECS
    } else {
        interval
    }
}

fn normalize_expire_in(expire_in: u64) -> u64 {
    if expire_in == 0 {
        DEFAULT_EXPIRE_IN_SECS
    } else {
        expire_in
    }
}

fn build_endpoint_url(domain: &str) -> String {
    format!("{}{ENDPOINT}", domain.trim_end_matches('/'))
}

fn build_qr_code_url(raw_url: &str, source: &str) -> Result<String, LarkError> {
    let mut parsed = url::Url::parse(raw_url)
        .map_err(|e| LarkError::Registration(format!("invalid QR URL: {e}")))?;
    {
        let mut q = parsed.query_pairs_mut();
        q.append_pair("from", "sdk");
        q.append_pair("tp", "sdk");
        if source.is_empty() {
            q.append_pair("source", SDK_NAME);
        } else {
            q.append_pair("source", &format!("{SDK_NAME}/{source}"));
        }
    }
    Ok(parsed.to_string())
}

async fn do_registration_request<T: for<'de> Deserialize<'de>>(
    client: &aioduct::Client<aioduct::runtime::TokioRuntime>,
    domain: &str,
    form: &[(&str, &str)],
) -> Result<T, LarkError> {
    let url = build_endpoint_url(domain);
    let body: String = url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(form)
        .finish();
    let resp = client
        .request(http::Method::POST, &url)
        .map_err(|e| LarkError::Registration(format!("invalid registration url: {e}")))?
        .header(
            http::header::CONTENT_TYPE,
            http::HeaderValue::from_static("application/x-www-form-urlencoded"),
        )
        .body(body.into_bytes())
        .send()
        .await
        .map_err(|e| LarkError::Registration(format!("registration request failed: {e}")))?;
    let raw_body = resp
        .bytes()
        .await
        .map_err(|e| LarkError::Registration(format!("read response body failed: {e}")))?;
    if raw_body.iter().all(|b| b.is_ascii_whitespace()) {
        return Err(LarkError::Registration(
            "register app error: invalid_response: empty response body".into(),
        ));
    }
    serde_json::from_slice(&raw_body)
        .map_err(|e| LarkError::Registration(format!("registration: decode response failed: {e}")))
}

pub async fn register_app(opts: Options) -> Result<RegisterAppResult, LarkError> {
    let domain = if opts.domain.is_empty() {
        DEFAULT_FEISHU_DOMAIN.to_string()
    } else {
        opts.domain.clone()
    };
    let lark_domain = if opts.lark_domain.is_empty() {
        DEFAULT_LARK_DOMAIN.to_string()
    } else {
        opts.lark_domain.clone()
    };

    let client = {
        crate::config::install_default_crypto_provider();
        aioduct::Client::builder()
            .tls(aioduct::tls::RustlsConnector::with_webpki_roots())
            .timeout(std::time::Duration::from_secs(30))
            .build()
    };

    let begin: BeginResponse = do_registration_request(
        &client,
        &domain,
        &[
            ("action", "begin"),
            ("archetype", "PersonalAgent"),
            ("auth_method", "client_secret"),
            ("request_user_info", "open_id"),
        ],
    )
    .await?;

    if begin.device_code.is_empty() {
        return Err(LarkError::Registration(
            "register app error: invalid_response: device_code is empty".into(),
        ));
    }
    if begin.verification_uri_complete.is_empty() {
        return Err(LarkError::Registration(
            "register app error: invalid_response: verification_uri_complete is empty".into(),
        ));
    }

    let qr_url = build_qr_code_url(&begin.verification_uri_complete, &opts.source)?;
    let expire_in = normalize_expire_in(begin.expire_in);
    (opts.on_qr_code)(&QRCodeInfo {
        url: qr_url,
        expire_in,
    });

    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(expire_in);
    let mut current_domain = domain;
    let mut interval = normalize_interval(begin.interval);
    let mut switched_domain = false;
    let mut wait_before_poll = false;

    loop {
        if wait_before_poll {
            if tokio::time::Instant::now() >= deadline {
                return Err(LarkError::Registration(
                    "register app error: expired_token: registration expired".into(),
                ));
            }
            tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
        }
        wait_before_poll = true;

        let resp: PollResponse = do_registration_request(
            &client,
            &current_domain,
            &[("action", "poll"), ("device_code", &begin.device_code)],
        )
        .await?;

        if let Some(ref ui) = resp.user_info
            && ui.tenant_brand == "lark"
            && !switched_domain
        {
            current_domain = lark_domain.clone();
            switched_domain = true;
            if let Some(ref cb) = opts.on_status_change {
                cb(&StatusChangeInfo {
                    status: STATUS_DOMAIN_SWITCHED.into(),
                    interval,
                });
            }
            wait_before_poll = false;
            continue;
        }

        if !resp.client_id.is_empty() && !resp.client_secret.is_empty() {
            return Ok(RegisterAppResult {
                client_id: resp.client_id,
                client_secret: resp.client_secret,
                user_info: resp.user_info.map(|ui| UserInfo {
                    open_id: ui.open_id,
                    tenant_brand: ui.tenant_brand,
                }),
            });
        }

        match resp.error.as_str() {
            "authorization_pending" | "" => {
                if let Some(ref cb) = opts.on_status_change {
                    cb(&StatusChangeInfo {
                        status: STATUS_POLLING.into(),
                        interval,
                    });
                }
            }
            "slow_down" => {
                interval += 5;
                if let Some(ref cb) = opts.on_status_change {
                    cb(&StatusChangeInfo {
                        status: STATUS_SLOW_DOWN.into(),
                        interval,
                    });
                }
            }
            "access_denied" | "expired_token" => {
                return Err(LarkError::Registration(format!(
                    "register app error: {}: {}",
                    resp.error, resp.error_description
                )));
            }
            _ => {
                return Err(LarkError::Registration(format!(
                    "register app error: {}: {}",
                    resp.error, resp.error_description
                )));
            }
        }
    }
}
