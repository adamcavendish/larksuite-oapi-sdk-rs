//! Lark event dispatch framework.
//!
//! [`EventDispatcher`] receives raw HTTP webhook requests, verifies signatures,
//! routes events by type, and calls registered async handlers.
//!
//! [`CardActionHandler`] does the same for interactive card callbacks.
//!
//! # Example
//!
//! ```rust,no_run
//! use larksuite_oapi_sdk_rs::EventDispatcher;
//!
//! let dispatcher = EventDispatcher::new("VERIFICATION_TOKEN", "ENCRYPT_KEY");
//! ```

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::cache::Cache;
use crate::crypto;
use crate::error::LarkError;

mod callback;
mod card_action;

pub use callback::{
    CallbackAction, CallbackCard, CallbackContext, CallbackOperator, CardActionTriggerRequest,
    CardActionTriggerResponse, InlinePreview, PreviewUrl, TemplateCard, Toast, ToastI18n,
    URLPreviewGetRequest, URLPreviewGetResponse,
};
pub use card_action::{
    CardAction, CardActionHandler, CardHandlerFn, CardHandlerResult, CustomResp,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventReq {
    #[serde(default)]
    pub headers: HashMap<String, Vec<String>>,
    pub body: Vec<u8>,
    #[serde(default)]
    pub request_uri: String,
}

impl EventReq {
    /// Extract the Lark request ID from the HTTP headers.
    /// Checks `X-Tt-Logid` first, then falls back to `X-Request-Id`.
    pub fn request_id(&self) -> &str {
        get_header_ref(&self.headers, "X-Tt-Logid")
            .or_else(|| get_header_ref(&self.headers, "X-Request-Id"))
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct EventResp {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub(crate) enum DispatchOutcome {
    Event(EventResp),
    Callback(EventResp),
}

impl DispatchOutcome {
    #[cfg(feature = "ws")]
    pub(crate) fn response(&self) -> &EventResp {
        match self {
            Self::Event(response) | Self::Callback(response) => response,
        }
    }

    #[cfg(feature = "ws")]
    pub(crate) fn callback_body(&self) -> Option<&[u8]> {
        match self {
            Self::Callback(response) => Some(&response.body),
            Self::Event(_) => None,
        }
    }

    fn into_response(self) -> EventResp {
        match self {
            Self::Event(response) | Self::Callback(response) => response,
        }
    }
}

impl EventResp {
    pub fn success(body: impl Serialize) -> Self {
        let body_bytes = serde_json::to_vec(&body).unwrap_or_default();
        Self {
            status_code: 200,
            headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
            body: body_bytes,
        }
    }

    pub fn error(status: u16, msg: &str) -> Self {
        let body = serde_json::json!({ "msg": msg });
        Self {
            status_code: status,
            headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
            body: serde_json::to_vec(&body).unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventHeader {
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    pub event_type: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub token: String,
}

/// P1 (v1.0 protocol) event header.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventV1Header {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub open_chat_id: String,
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(rename = "type", default)]
    pub event_type: String,
}

/// P2 (v2.0 protocol) event base with schema and header.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventV2Base {
    #[serde(default)]
    pub schema: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<EventHeader>,
}

impl EventV2Base {
    #[must_use]
    #[inline]
    pub fn tenant_key(&self) -> &str {
        self.header
            .as_ref()
            .map(|h| h.tenant_key.as_str())
            .unwrap_or_default()
    }
}

/// P1 (v1.0 protocol) event body base fields.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventBase {
    #[serde(default)]
    pub ts: String,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub token: String,
    #[serde(rename = "type", default)]
    pub event_type: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventV2Body {
    pub schema: Option<String>,
    pub header: Option<EventHeader>,
    pub challenge: Option<String>,
    #[serde(rename = "type")]
    pub req_type: Option<String>,
    pub token: Option<String>,
    #[serde(default)]
    pub event: crate::JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EncryptedBody {
    encrypt: String,
}

pub type EventHandlerFn = Arc<
    dyn Fn(crate::JsonValue) -> Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>
        + Send
        + Sync,
>;

pub type CustomizedEventHandlerFn = Arc<
    dyn Fn(EventReq, EventV2Body) -> Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>
        + Send
        + Sync,
>;

pub type CallbackHandlerFn = Arc<
    dyn Fn(
            crate::JsonValue,
        ) -> Pin<Box<dyn Future<Output = Result<crate::JsonValue, LarkError>> + Send>>
        + Send
        + Sync,
>;

/// One complete HTTP webhook authentication credential snapshot.
///
/// Replace both fields together when rotating credentials. Debug output redacts
/// both values. Empty fields retain the existing empty-credential behavior.
#[derive(Clone, Default)]
pub struct WebhookCredentials {
    pub verification_token: String,
    pub encrypt_key: String,
}

impl std::fmt::Debug for WebhookCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebhookCredentials")
            .field("verification_token", &"[REDACTED]")
            .field("encrypt_key", &"[REDACTED]")
            .finish()
    }
}

/// A credential lookup failure without secret-bearing error details.
#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error("webhook credentials provider failed")]
pub struct WebhookCredentialsError;

/// Supplies a local credential snapshot once per HTTP webhook request.
///
/// Implementations must be thread safe and promptly read local cached
/// configuration. Refresh external configuration outside the request path:
/// this synchronous API provides no I/O timeout or cancellation. Failures
/// reject the request without static-credential fallback. Trusted WebSocket
/// dispatch never invokes this provider.
///
/// Unwinding panics are converted to a fixed error, but the process panic hook
/// may still run. Never put secrets in panic values. `panic=abort` cannot be
/// recovered.
pub trait WebhookCredentialsProvider: Send + Sync {
    fn credentials(&self) -> Result<WebhookCredentials, WebhookCredentialsError>;
}

impl<F> WebhookCredentialsProvider for F
where
    F: Fn() -> Result<WebhookCredentials, WebhookCredentialsError> + Send + Sync,
{
    fn credentials(&self) -> Result<WebhookCredentials, WebhookCredentialsError> {
        self()
    }
}

// ── Shared dispatch pipeline ──

#[derive(Debug, Clone)]
enum SignAlgorithm {
    Sha256,
    Sha1,
}

/// Internal result produced by [`DispatchPipeline::process`].
enum PipelineResult {
    /// URL verification challenge — return this response immediately.
    Challenge(EventResp),
    /// Decrypted, verified event body with the original request.
    Event { body_str: String, req: EventReq },
}

/// Shared pre-processing pipeline used by both [`EventDispatcher`] and
/// [`CardActionHandler`]. Owns decode, decryption, challenge handling, and
/// signature verification.
#[derive(Clone)]
struct DispatchPipeline {
    verification_token: String,
    event_encrypt_key: String,
    skip_sign_verify: bool,
    sign_algorithm: SignAlgorithm,
    credentials_provider: Option<Arc<dyn WebhookCredentialsProvider>>,
}

impl DispatchPipeline {
    fn new(
        verification_token: String,
        event_encrypt_key: String,
        sign_algorithm: SignAlgorithm,
    ) -> Self {
        Self {
            verification_token,
            event_encrypt_key,
            skip_sign_verify: false,
            sign_algorithm,
            credentials_provider: None,
        }
    }

    fn process(&self, req: EventReq) -> Result<PipelineResult, LarkError> {
        let credentials = match &self.credentials_provider {
            Some(provider) => {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| provider.credentials()))
                    .map_err(|_| WebhookCredentialsError)
                    .and_then(|result| result)
                    .map_err(|_| LarkError::Event(WebhookCredentialsError.to_string()))?
            }
            None => WebhookCredentials {
                verification_token: self.verification_token.clone(),
                encrypt_key: self.event_encrypt_key.clone(),
            },
        };
        self.process_with_credentials(req, &credentials)
    }

    fn process_with_credentials(
        &self,
        req: EventReq,
        credentials: &WebhookCredentials,
    ) -> Result<PipelineResult, LarkError> {
        let body_str = std::str::from_utf8(&req.body)
            .map_err(|e| LarkError::Event(format!("invalid utf8 body: {e}")))?;

        let body_str = decrypt_if_needed(&credentials.encrypt_key, body_str)?;

        let parsed: crate::JsonValue = serde_json::from_str(&body_str)
            .map_err(|e| LarkError::Event(format!("failed to parse body: {e}")))?;

        // Check for URL verification in both V2 ("req_type") and card ("type") shapes.
        let is_challenge = parsed.get("type").and_then(|v| v.as_str()) == Some("url_verification")
            || parsed.get("req_type").and_then(|v| v.as_str()) == Some("url_verification");

        if is_challenge {
            verify_webhook_token(&credentials.verification_token, parsed.get("token"))?;
            let challenge = parsed
                .get("challenge")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            return Ok(PipelineResult::Challenge(EventResp::success(
                serde_json::json!({ "challenge": challenge }),
            )));
        }

        if matches!(self.sign_algorithm, SignAlgorithm::Sha256)
            && credentials.encrypt_key.is_empty()
            && !credentials.verification_token.is_empty()
        {
            // Any v2 marker forbids fallback to the v1 top-level token.
            let token = if parsed.get("schema").is_some() || parsed.get("header").is_some() {
                parsed.get("header").and_then(|header| header.get("token"))
            } else {
                parsed.get("token")
            };
            verify_webhook_token(&credentials.verification_token, token)?;
        }

        if !self.skip_sign_verify {
            match self.sign_algorithm {
                SignAlgorithm::Sha256 => {
                    if !credentials.encrypt_key.is_empty() {
                        let (timestamp, nonce, sig) = extract_signature_headers(&req.headers)?;
                        if !crypto::verify_signature_sha256(
                            &timestamp,
                            &nonce,
                            &credentials.encrypt_key,
                            &req.body,
                            &sig,
                        ) {
                            return Err(LarkError::Event(
                                "signature verification failed".to_string(),
                            ));
                        }
                    }
                }
                SignAlgorithm::Sha1 => {
                    if !credentials.verification_token.is_empty() {
                        let (timestamp, nonce, sig) = extract_signature_headers(&req.headers)?;
                        if !crypto::verify_signature_sha1(
                            &timestamp,
                            &nonce,
                            &credentials.verification_token,
                            &body_str,
                            &sig,
                        ) {
                            return Err(LarkError::Event(
                                "card signature verification failed".to_string(),
                            ));
                        }
                    }
                }
            }
        }

        Ok(PipelineResult::Event { body_str, req })
    }
}

fn verify_webhook_token(
    expected: &str,
    token: Option<&serde_json::Value>,
) -> Result<(), LarkError> {
    use subtle::ConstantTimeEq;

    if expected.is_empty() {
        return Ok(());
    }
    let valid = token
        .and_then(|value| value.as_str())
        .is_some_and(|actual| bool::from(expected.as_bytes().ct_eq(actual.as_bytes())));
    if valid {
        Ok(())
    } else {
        Err(LarkError::Event("verification token mismatch".into()))
    }
}

// ── EventDispatcher ──

#[must_use]
pub struct EventDispatcher {
    event_handlers: HashMap<String, EventHandlerFn>,
    customized_event_handlers: HashMap<String, CustomizedEventHandlerFn>,
    callback_handlers: HashMap<String, CallbackHandlerFn>,
    pipeline: DispatchPipeline,
    token_cache: Option<Arc<dyn Cache>>,
}

impl EventDispatcher {
    pub fn new(
        verification_token: impl Into<String>,
        event_encrypt_key: impl Into<String>,
    ) -> Self {
        Self {
            event_handlers: HashMap::new(),
            customized_event_handlers: HashMap::new(),
            callback_handlers: HashMap::new(),
            pipeline: DispatchPipeline::new(
                verification_token.into(),
                event_encrypt_key.into(),
                SignAlgorithm::Sha256,
            ),
            token_cache: None,
        }
    }

    /// Register a local credential snapshot provider without invoking it.
    ///
    /// Each HTTP request uses one snapshot; trusted WebSocket delivery does not
    /// invoke the provider. Without a provider, constructor credentials are used.
    pub fn with_webhook_credentials_provider(
        mut self,
        provider: Arc<dyn WebhookCredentialsProvider>,
    ) -> Self {
        self.pipeline.credentials_provider = Some(provider);
        self
    }

    /// Skip HTTP signatures while retaining configured verification-token checks.
    pub fn skip_sign_verify(mut self) -> Self {
        self.pipeline.skip_sign_verify = true;
        self
    }

    pub fn token_cache(mut self, cache: Arc<dyn Cache>) -> Self {
        self.token_cache = Some(cache);
        self
    }

    /// Register an automatic app_ticket handler for ISV (marketplace) apps.
    /// When an `app_ticket` event arrives, the ticket is stored in the token cache
    /// so that subsequent token requests can use it automatically.
    pub fn with_auto_app_ticket(self) -> Self {
        use std::time::Duration;

        use crate::token::AppTicketManager;

        let cache: Arc<dyn Cache> = self
            .token_cache
            .clone()
            .unwrap_or_else(|| Arc::new(crate::cache::LocalCache::new()));

        self.on_event("app_ticket", move |val: crate::JsonValue| {
            let cache = cache.clone();
            async move {
                let app_id = val
                    .get("app_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let app_ticket = val
                    .get("app_ticket")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                if app_id.is_empty() || app_ticket.is_empty() {
                    return Ok(());
                }
                let mgr = AppTicketManager::new(cache);
                mgr.set(&app_id, &app_ticket, Duration::from_secs(3600))
                    .await
            }
        })
    }

    pub fn on_event<F, Fut>(mut self, event_type: impl Into<String>, handler: F) -> Self
    where
        F: Fn(crate::JsonValue) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        let event_type = event_type.into();
        let handler = Arc::new(
            move |val: crate::JsonValue| -> Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>> {
                Box::pin(handler(val))
            },
        );
        self.event_handlers.insert(event_type, handler);
        self
    }

    pub fn on_callback<F, Fut, T>(mut self, callback_type: impl Into<String>, handler: F) -> Self
    where
        F: Fn(crate::JsonValue) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, LarkError>> + Send + 'static,
        T: Into<crate::JsonValue> + Send + 'static,
    {
        let callback_type = callback_type.into();
        let handler =
            Arc::new(
                move |val: crate::JsonValue| -> Pin<
                    Box<dyn Future<Output = Result<crate::JsonValue, LarkError>> + Send>,
                > {
                    let fut = handler(val);
                    Box::pin(async move { fut.await.map(Into::into) })
                },
            );
        self.callback_handlers.insert(callback_type, handler);
        self
    }

    fn on_typed_callback<Req, Resp, F, Fut>(
        mut self,
        callback_type: impl Into<String>,
        response_name: &'static str,
        handler: F,
    ) -> Self
    where
        Req: for<'de> Deserialize<'de> + Send + 'static,
        Resp: Serialize + Send + 'static,
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Resp, LarkError>> + Send + 'static,
    {
        let callback_type = callback_type.into();
        let handler =
            Arc::new(
                move |val: crate::JsonValue| -> Pin<
                    Box<dyn Future<Output = Result<crate::JsonValue, LarkError>> + Send>,
                > {
                    let req = match serde_json::from_value::<Req>(val.into()) {
                        Ok(req) => req,
                        Err(e) => {
                            return Box::pin(async move {
                                Err(LarkError::Event(format!(
                                    "failed to deserialize callback payload: {e}"
                                )))
                            });
                        }
                    };
                    let fut = handler(req);
                    Box::pin(async move {
                        let resp = fut.await?;
                        crate::JsonValue::from_serializable(resp).map_err(|e| {
                            LarkError::Event(format!("serialize {response_name} response: {e}"))
                        })
                    })
                },
            );
        self.callback_handlers.insert(callback_type, handler);
        self
    }

    #[cfg(feature = "channel")]
    pub(crate) fn has_callback_handler(&self, callback_type: &str) -> bool {
        self.callback_handlers.contains_key(callback_type)
    }

    /// Register a raw event handler that receives the full [`EventReq`] and
    /// parsed [`EventV2Body`], including headers, raw body, and request URI.
    pub fn on_customized_event<F, Fut>(mut self, event_type: impl Into<String>, handler: F) -> Self
    where
        F: Fn(EventReq, EventV2Body) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        let event_type = event_type.into();
        let handler = Arc::new(
            move |req: EventReq,
                  body: EventV2Body|
                  -> Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>> {
                Box::pin(handler(req, body))
            },
        );
        self.customized_event_handlers.insert(event_type, handler);
        self
    }

    /// Register a typed handler for `card.action.trigger` callbacks.
    pub fn on_card_action_trigger<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(CardActionTriggerRequest) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<CardActionTriggerResponse, LarkError>> + Send + 'static,
    {
        self.on_typed_callback("card.action.trigger", "trigger", handler)
    }

    /// Register a typed handler for `url.preview.get` callbacks.
    pub fn on_url_preview_get<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(URLPreviewGetRequest) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<URLPreviewGetResponse, LarkError>> + Send + 'static,
    {
        self.on_typed_callback("url.preview.get", "preview", handler)
    }

    /// Extract the raw event payload from an [`EventReq`].
    ///
    /// If an encrypt key is configured and the body contains an `"encrypt"` field,
    /// returns the encrypted ciphertext string. Otherwise returns the raw body as-is.
    /// Uses static constructor credentials; dynamic providers apply only to
    /// complete HTTP requests through [`Self::handle`].
    pub fn parse_req(&self, req: &EventReq) -> Result<String, LarkError> {
        let body_str = std::str::from_utf8(&req.body)
            .map_err(|e| LarkError::Event(format!("invalid utf8 body: {e}")))?;

        if self.pipeline.event_encrypt_key.is_empty() {
            return Ok(body_str.to_string());
        }

        let parsed: std::result::Result<EncryptedBody, _> = serde_json::from_str(body_str);
        match parsed {
            Ok(encrypted) if !encrypted.encrypt.is_empty() => Ok(encrypted.encrypt),
            _ => Ok(body_str.to_string()),
        }
    }

    /// Decrypt an event payload string.
    ///
    /// If an encrypt key is configured, decrypts the ciphertext. Otherwise
    /// returns the input unchanged. This is the second step after [`Self::parse_req`].
    /// Uses static constructor credentials rather than a dynamic provider.
    pub fn decrypt_event(&self, cipher_event_json: &str) -> Result<String, LarkError> {
        if self.pipeline.event_encrypt_key.is_empty() {
            return Ok(cipher_event_json.to_string());
        }
        crypto::event_decrypt(&self.pipeline.event_encrypt_key, cipher_event_json)
    }

    pub async fn handle(&self, req: EventReq) -> EventResp {
        self.dispatch(self.pipeline.process(req))
            .await
            .into_response()
    }

    #[cfg(feature = "ws")]
    pub(crate) async fn dispatch_for_ws(&self, req: EventReq) -> DispatchOutcome {
        // The WebSocket transport has already authenticated this plaintext
        // payload. Never resolve HTTP credentials or apply HTTP preprocessing.
        let prepared = std::str::from_utf8(&req.body)
            .map(|body| body.to_owned())
            .map_err(|e| LarkError::Event(format!("invalid utf8 body: {e}")))
            .map(|body_str| PipelineResult::Event { body_str, req });
        self.dispatch(prepared).await
    }

    async fn dispatch(&self, prepared: Result<PipelineResult, LarkError>) -> DispatchOutcome {
        match self.do_handle(prepared).await {
            Ok(outcome) => outcome,
            Err(e) => {
                tracing::error!("event handler error: {e}");
                DispatchOutcome::Event(EventResp::error(500, &e.to_string()))
            }
        }
    }

    async fn do_handle(
        &self,
        prepared: Result<PipelineResult, LarkError>,
    ) -> Result<DispatchOutcome, LarkError> {
        let (body_str, req) = match prepared? {
            PipelineResult::Challenge(resp) => return Ok(DispatchOutcome::Event(resp)),
            PipelineResult::Event { body_str, req } => (body_str, req),
        };

        let parsed: EventV2Body = serde_json::from_str(&body_str)
            .map_err(|e| LarkError::Event(format!("failed to parse event body: {e}")))?;

        let event_type = parsed
            .header
            .as_ref()
            .map(|h| h.event_type.as_str())
            .unwrap_or_else(|| {
                // P1 protocol: event type is in event.type
                parsed.event.as_value()["type"].as_str().unwrap_or_default()
            });

        if let Some(handler) = self.callback_handlers.get(event_type) {
            let event_data = parsed.event.clone();
            let resp_data = handler(event_data).await?;
            return Ok(DispatchOutcome::Callback(EventResp::success(resp_data)));
        }

        if let Some(handler) = self.customized_event_handlers.get(event_type) {
            handler(req, parsed).await?;
            return Ok(DispatchOutcome::Event(EventResp::success(
                serde_json::json!({ "msg": "success" }),
            )));
        }

        if let Some(handler) = self.event_handlers.get(event_type) {
            let event_data = parsed.event.clone();
            handler(event_data).await?;
            return Ok(DispatchOutcome::Event(EventResp::success(
                serde_json::json!({ "msg": "success" }),
            )));
        }

        tracing::warn!("no handler registered for event type: {event_type}");
        Ok(DispatchOutcome::Event(EventResp::success(
            serde_json::json!(
                { "msg": format!("no handler for event type: {event_type}") }
            ),
        )))
    }
}

fn get_header_ref<'a>(headers: &'a HashMap<String, Vec<String>>, key: &str) -> Option<&'a str> {
    headers
        .get(key)
        .or_else(|| {
            headers
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(key))
                .map(|(_, values)| values)
        })
        .and_then(|v| v.first())
        .map(|s| s.as_str())
}

fn get_header(headers: &HashMap<String, Vec<String>>, key: &str) -> String {
    get_header_ref(headers, key).unwrap_or_default().to_string()
}

fn extract_signature_headers(
    headers: &HashMap<String, Vec<String>>,
) -> Result<(String, String, String), LarkError> {
    let timestamp = get_header(headers, "X-Lark-Request-Timestamp");
    let nonce = get_header(headers, "X-Lark-Request-Nonce");
    let sig = get_header(headers, "X-Lark-Signature");

    if sig.is_empty() {
        return Err(LarkError::Event(
            "missing X-Lark-Signature header".to_string(),
        ));
    }

    Ok((timestamp, nonce, sig))
}

fn decrypt_if_needed(encrypt_key: &str, body: &str) -> Result<String, LarkError> {
    if encrypt_key.is_empty() {
        return Ok(body.to_string());
    }

    let parsed: std::result::Result<EncryptedBody, _> = serde_json::from_str(body);
    match parsed {
        Ok(encrypted) if !encrypted.encrypt.is_empty() => {
            crypto::event_decrypt(encrypt_key, &encrypted.encrypt)
        }
        _ => Ok(body.to_string()),
    }
}
