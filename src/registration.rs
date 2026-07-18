//! OAuth app registration via device-code flow.
//!
//! Mirrors the Go SDK's `scene/registration` package. Drives the
//! `/oauth/v1/app/registration` endpoint to create a new app by
//! having the user scan a QR code.

use std::io::Write as _;

use base64::Engine as _;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Default)]
pub struct AppPreset {
    pub avatar: Vec<String>,
    pub name: String,
    pub desc: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AppAddons {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preset: Option<bool>,
    #[serde(default, skip_serializing_if = "AppAddonsScopes::is_empty")]
    pub scopes: AppAddonsScopes,
    #[serde(default, skip_serializing_if = "AppAddonsEvents::is_empty")]
    pub events: AppAddonsEvents,
    #[serde(default, skip_serializing_if = "AppAddonsCallbacks::is_empty")]
    pub callbacks: AppAddonsCallbacks,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AppAddonsScopes {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tenant: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<String>,
}

impl AppAddonsScopes {
    fn is_empty(&self) -> bool {
        self.tenant.is_empty() && self.user.is_empty()
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AppAddonsEvents {
    #[serde(default, skip_serializing_if = "AppAddonsEventItems::is_empty")]
    pub items: AppAddonsEventItems,
}

impl AppAddonsEvents {
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AppAddonsEventItems {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tenant: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user: Vec<String>,
}

impl AppAddonsEventItems {
    fn is_empty(&self) -> bool {
        self.tenant.is_empty() && self.user.is_empty()
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AppAddonsCallbacks {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}

impl AppAddonsCallbacks {
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
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
    pub app_preset: Option<AppPreset>,
    pub addons: Option<AppAddons>,
    pub create_only: bool,
    pub app_id: String,
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

fn validate_addons_list(values: &[String], path: &str) -> Result<usize, LarkError> {
    for (idx, value) in values.iter().enumerate() {
        if value.is_empty() {
            return Err(LarkError::Registration(format!(
                "registration: {path}[{idx}] must be a non-empty string"
            )));
        }
    }
    Ok(values.len())
}

fn encode_addons(addons: &AppAddons) -> Result<String, LarkError> {
    let mut count = 0usize;
    count += validate_addons_list(&addons.scopes.tenant, "Addons.Scopes.Tenant")?;
    count += validate_addons_list(&addons.scopes.user, "Addons.Scopes.User")?;
    count += validate_addons_list(&addons.events.items.tenant, "Addons.Events.Items.Tenant")?;
    count += validate_addons_list(&addons.events.items.user, "Addons.Events.Items.User")?;
    count += validate_addons_list(&addons.callbacks.items, "Addons.Callbacks.Items")?;
    if count == 0 && addons.preset != Some(false) {
        return Err(LarkError::Registration(
            "registration: Addons must contain at least one scope, event or callback".into(),
        ));
    }

    let body = serde_json::to_vec(addons).map_err(|e| {
        LarkError::Registration(format!("registration: marshal Addons failed: {e}"))
    })?;
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder
        .write_all(&body)
        .map_err(|e| LarkError::Registration(format!("registration: gzip Addons failed: {e}")))?;
    let compressed = encoder
        .finish()
        .map_err(|e| LarkError::Registration(format!("registration: gzip Addons failed: {e}")))?;
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(compressed))
}

fn app_preset_pairs(preset: &AppPreset) -> Result<Vec<(String, String)>, LarkError> {
    if preset.avatar.len() > 6 {
        return Err(LarkError::Registration(format!(
            "registration: AppPreset.Avatar supports at most 6 URLs, got {}",
            preset.avatar.len()
        )));
    }
    let mut pairs = Vec::new();
    for (idx, avatar) in preset.avatar.iter().enumerate() {
        if avatar.is_empty() {
            return Err(LarkError::Registration(format!(
                "registration: AppPreset.Avatar[{idx}] must be a non-empty string"
            )));
        }
        pairs.push(("avatar".to_string(), avatar.clone()));
    }
    if !preset.name.is_empty() {
        pairs.push(("name".to_string(), preset.name.clone()));
    }
    if !preset.desc.is_empty() {
        pairs.push(("desc".to_string(), preset.desc.clone()));
    }
    Ok(pairs)
}

fn build_qr_code_url(raw_url: &str, opts: &Options) -> Result<String, LarkError> {
    let mut parsed = url::Url::parse(raw_url)
        .map_err(|e| LarkError::Registration(format!("invalid QR URL: {e}")))?;
    {
        let preserved: Vec<(String, String)> = parsed.query_pairs().into_owned().collect();
        let mut q = parsed.query_pairs_mut();
        q.clear();
        for (key, value) in preserved {
            if key != "from" && key != "tp" && key != "source" {
                q.append_pair(&key, &value);
            }
        }
        q.append_pair("from", "sdk");
        q.append_pair("tp", "sdk");
        if opts.source.is_empty() {
            q.append_pair("source", SDK_NAME);
        } else {
            q.append_pair("source", &format!("{SDK_NAME}/{}", opts.source));
        }
        if let Some(ref preset) = opts.app_preset {
            for (key, value) in app_preset_pairs(preset)? {
                q.append_pair(&key, &value);
            }
        }
        if let Some(ref addons) = opts.addons {
            q.append_pair("addons", &encode_addons(addons)?);
        }
        if opts.create_only {
            q.append_pair("createOnly", "true");
        }
        if !opts.app_id.is_empty() {
            if opts.app_id.trim().is_empty() {
                return Err(LarkError::Registration(
                    "registration: Options.AppID must be a non-empty string".into(),
                ));
            }
            q.append_pair("clientID", &opts.app_id);
        }
    }
    Ok(parsed.to_string())
}

async fn do_registration_request<T: for<'de> Deserialize<'de>>(
    client: &aioduct::TokioClient,
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
        aioduct::TokioClient::builder()
            .tls(aioduct::tls::RustlsConnector::with_webpki_roots())
            .timeout(std::time::Duration::from_secs(30))
            .build()?
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

    let qr_url = build_qr_code_url(&begin.verification_uri_complete, &opts)?;
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
            "authorization_pending" => {
                if let Some(ref cb) = opts.on_status_change {
                    cb(&StatusChangeInfo {
                        status: STATUS_POLLING.into(),
                        interval,
                    });
                }
            }
            "" => {}
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    async fn mock_server_with_requests(
        responses: Vec<String>,
    ) -> (
        std::net::SocketAddr,
        tokio::task::JoinHandle<()>,
        Arc<Mutex<Vec<String>>>,
    ) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let responses = Arc::new(responses);
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let requests = Arc::new(Mutex::new(Vec::new()));

        let handle = tokio::spawn({
            let requests = Arc::clone(&requests);
            async move {
                loop {
                    let Ok((mut stream, _)) = listener.accept().await else {
                        break;
                    };
                    let responses = Arc::clone(&responses);
                    let counter = Arc::clone(&counter);
                    let requests = Arc::clone(&requests);
                    tokio::spawn(async move {
                        use tokio::io::{AsyncReadExt, AsyncWriteExt};

                        let mut buf = vec![0u8; 8192];
                        let Ok(n) = stream.read(&mut buf).await else {
                            return;
                        };
                        requests
                            .lock()
                            .unwrap()
                            .push(String::from_utf8_lossy(&buf[..n]).to_string());
                        let idx = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        let resp_idx = idx.min(responses.len() - 1);
                        let _ = stream.write_all(responses[resp_idx].as_bytes()).await;
                        let _ = stream.shutdown().await;
                    });
                }
            }
        });

        (addr, handle, requests)
    }

    #[test]
    fn qr_url_preserves_unrelated_query_params() {
        let opts = Options {
            source: "local-test".to_string(),
            domain: String::new(),
            lark_domain: String::new(),
            app_preset: None,
            addons: None,
            create_only: false,
            app_id: String::new(),
            on_qr_code: Box::new(|_| {}),
            on_status_change: None,
        };
        let url = build_qr_code_url(
            "https://qr.example.com/scan?foo=bar&from=old&tp=old&source=other",
            &opts,
        )
        .unwrap();
        let parsed = url::Url::parse(&url).unwrap();
        let mut query = parsed.query_pairs();
        assert!(query.clone().any(|(k, v)| k == "foo" && v == "bar"));
        assert!(query.clone().any(|(k, v)| k == "from" && v == "sdk"));
        assert!(query.clone().any(|(k, v)| k == "tp" && v == "sdk"));
        assert!(query.any(|(k, v)| k == "source" && v == "rust-sdk/local-test"));
    }

    #[test]
    fn qr_url_adds_preset_addons_create_only_and_app_id() {
        let opts = Options {
            source: String::new(),
            domain: String::new(),
            lark_domain: String::new(),
            app_preset: Some(AppPreset {
                avatar: vec!["https://example.com/a.png".into()],
                name: "Bot {user}".into(),
                desc: "Desc {user}".into(),
            }),
            addons: Some(AppAddons {
                preset: None,
                scopes: AppAddonsScopes {
                    tenant: vec!["im:message:send_as_bot".into()],
                    user: vec!["calendar:calendar:read".into()],
                },
                events: AppAddonsEvents {
                    items: AppAddonsEventItems {
                        tenant: vec!["im.message.receive_v1".into()],
                        user: Vec::new(),
                    },
                },
                callbacks: AppAddonsCallbacks {
                    items: vec!["card.action.trigger".into()],
                },
            }),
            create_only: true,
            app_id: "cli_existing".into(),
            on_qr_code: Box::new(|_| {}),
            on_status_change: None,
        };

        let url = build_qr_code_url("https://qr.example.com/scan", &opts).unwrap();
        let parsed = url::Url::parse(&url).unwrap();
        let pairs: std::collections::HashMap<_, _> = parsed.query_pairs().into_owned().collect();
        assert_eq!(pairs.get("source").map(String::as_str), Some("rust-sdk"));
        assert_eq!(
            pairs.get("avatar").map(String::as_str),
            Some("https://example.com/a.png")
        );
        assert_eq!(pairs.get("name").map(String::as_str), Some("Bot {user}"));
        assert_eq!(pairs.get("desc").map(String::as_str), Some("Desc {user}"));
        assert_eq!(pairs.get("createOnly").map(String::as_str), Some("true"));
        assert_eq!(
            pairs.get("clientID").map(String::as_str),
            Some("cli_existing")
        );

        let addons = pairs.get("addons").unwrap();
        let compressed = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(addons)
            .unwrap();
        let mut decoder = flate2::read::GzDecoder::new(compressed.as_slice());
        let mut decoded = String::new();
        std::io::Read::read_to_string(&mut decoder, &mut decoded).unwrap();
        let json: crate::JsonValue = serde_json::from_str(&decoded).unwrap();
        assert_eq!(json["scopes"]["tenant"][0], "im:message:send_as_bot");
        assert_eq!(
            json["events"]["items"]["tenant"][0],
            "im.message.receive_v1"
        );
        assert_eq!(json["callbacks"]["items"][0], "card.action.trigger");
        assert!(json.get("preset").is_none());
    }

    #[test]
    fn addons_preset_false_serializes_and_allows_empty_addons() {
        let encoded = encode_addons(&AppAddons {
            preset: Some(false),
            ..Default::default()
        })
        .unwrap();
        let compressed = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(encoded)
            .unwrap();
        let mut decoder = flate2::read::GzDecoder::new(compressed.as_slice());
        let mut decoded = String::new();
        std::io::Read::read_to_string(&mut decoder, &mut decoded).unwrap();

        let json: crate::JsonValue = serde_json::from_str(&decoded).unwrap();
        assert_eq!(json, serde_json::json!({ "preset": false }).into());
    }

    #[test]
    fn addons_preset_true_or_unset_requires_entries() {
        for preset in [None, Some(true)] {
            let err = encode_addons(&AppAddons {
                preset,
                ..Default::default()
            })
            .unwrap_err();
            assert!(
                err.to_string()
                    .contains("Addons must contain at least one scope, event or callback")
            );
        }
    }

    #[test]
    fn addons_preset_false_keeps_non_empty_string_validation() {
        let err = encode_addons(&AppAddons {
            preset: Some(false),
            callbacks: AppAddonsCallbacks {
                items: vec!["card.action.trigger".into(), String::new()],
            },
            ..Default::default()
        })
        .unwrap_err();
        assert!(
            err.to_string()
                .contains("Addons.Callbacks.Items[1] must be a non-empty string")
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn register_app_keeps_polling_on_empty_response() {
        let page1 = r#"{"device_code":"device-empty","verification_uri_complete":"https://qr.example.com/scan?foo=bar&from=old&tp=old&source=other","interval":1,"expire_in":60}"#;
        let page2 = r#"{}"#;
        let page3 = r#"{"client_id":"cli_after_empty","client_secret":"sec_after_empty"}"#;
        let (addr, _h, requests) = mock_server_with_requests(vec![
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                page1.len(),
                page1
            ),
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                page2.len(),
                page2
            ),
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                page3.len(),
                page3
            ),
        ])
        .await;

        let qr_info = Arc::new(Mutex::new(None));
        let qr_info_cb = Arc::clone(&qr_info);
        let statuses = Arc::new(Mutex::new(Vec::new()));
        let statuses_cb = Arc::clone(&statuses);
        let handle = tokio::spawn(register_app(Options {
            source: "local".to_string(),
            domain: format!("http://{}", addr),
            lark_domain: String::new(),
            app_preset: None,
            addons: None,
            create_only: false,
            app_id: String::new(),
            on_qr_code: Box::new(move |info| {
                *qr_info_cb.lock().unwrap() = Some(info.clone());
            }),
            on_status_change: Some(Box::new(move |info| {
                statuses_cb.lock().unwrap().push(info.status.clone());
            })),
        }));

        for _ in 0..50 {
            if requests.lock().unwrap().len() >= 3 {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
        assert_eq!(requests.lock().unwrap().len(), 3);

        let result = handle.await.unwrap().unwrap();
        assert_eq!(result.client_id, "cli_after_empty");
        assert_eq!(statuses.lock().unwrap().as_slice(), &[] as &[String]);

        let qr_info = qr_info.lock().unwrap().clone().unwrap();
        let parsed = url::Url::parse(&qr_info.url).unwrap();
        let mut query = parsed.query_pairs();
        assert!(query.clone().any(|(k, v)| k == "foo" && v == "bar"));
        assert!(query.clone().any(|(k, v)| k == "from" && v == "sdk"));
        assert!(query.clone().any(|(k, v)| k == "tp" && v == "sdk"));
        assert!(query.any(|(k, v)| k == "source" && v == "rust-sdk/local"));
    }
}
