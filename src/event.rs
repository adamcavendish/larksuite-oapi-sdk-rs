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
pub struct EventV2Body {
    pub schema: Option<String>,
    pub header: Option<EventHeader>,
    pub challenge: Option<String>,
    #[serde(rename = "type")]
    pub req_type: Option<String>,
    pub token: Option<String>,
    #[serde(default)]
    pub event: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EncryptedBody {
    encrypt: String,
}

pub type EventHandlerFn = Arc<
    dyn Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>
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
            serde_json::Value,
        ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, LarkError>> + Send>>
        + Send
        + Sync,
>;

#[must_use]
pub struct EventDispatcher {
    event_handlers: HashMap<String, EventHandlerFn>,
    customized_event_handlers: HashMap<String, CustomizedEventHandlerFn>,
    callback_handlers: HashMap<String, CallbackHandlerFn>,
    verification_token: String,
    event_encrypt_key: String,
    skip_sign_verify: bool,
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
            verification_token: verification_token.into(),
            event_encrypt_key: event_encrypt_key.into(),
            skip_sign_verify: false,
            token_cache: None,
        }
    }

    pub fn skip_sign_verify(mut self) -> Self {
        self.skip_sign_verify = true;
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

        self.on_event("app_ticket", move |val: serde_json::Value| {
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
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        let event_type = event_type.into();
        let handler = Arc::new(
            move |val: serde_json::Value| -> Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>> {
                Box::pin(handler(val))
            },
        );
        self.event_handlers.insert(event_type, handler);
        self
    }

    pub fn on_callback<F, Fut>(mut self, callback_type: impl Into<String>, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<serde_json::Value, LarkError>> + Send + 'static,
    {
        let callback_type = callback_type.into();
        let handler =
            Arc::new(
                move |val: serde_json::Value| -> Pin<
                    Box<dyn Future<Output = Result<serde_json::Value, LarkError>> + Send>,
                > { Box::pin(handler(val)) },
            );
        self.callback_handlers.insert(callback_type, handler);
        self
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
        self.on_callback("card.action.trigger", move |val: serde_json::Value| {
            let req: CardActionTriggerRequest = serde_json::from_value(val).unwrap_or_default();
            let fut = handler(req);
            async move {
                let resp = fut.await?;
                serde_json::to_value(resp)
                    .map_err(|e| LarkError::Event(format!("serialize trigger response: {e}")))
            }
        })
    }

    /// Register a typed handler for `url.preview.get` callbacks.
    pub fn on_url_preview_get<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(URLPreviewGetRequest) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<URLPreviewGetResponse, LarkError>> + Send + 'static,
    {
        self.on_callback("url.preview.get", move |val: serde_json::Value| {
            let req: URLPreviewGetRequest = serde_json::from_value(val).unwrap_or_default();
            let fut = handler(req);
            async move {
                let resp = fut.await?;
                serde_json::to_value(resp)
                    .map_err(|e| LarkError::Event(format!("serialize preview response: {e}")))
            }
        })
    }

    /// Extract the raw event payload from an [`EventReq`].
    ///
    /// If an encrypt key is configured and the body contains an `"encrypt"` field,
    /// returns the encrypted ciphertext string. Otherwise returns the raw body as-is.
    pub fn parse_req(&self, req: &EventReq) -> Result<String, LarkError> {
        let body_str = std::str::from_utf8(&req.body)
            .map_err(|e| LarkError::Event(format!("invalid utf8 body: {e}")))?;

        if self.event_encrypt_key.is_empty() {
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
    /// returns the input unchanged. This is the second step after [`parse_req`].
    pub fn decrypt_event(&self, cipher_event_json: &str) -> Result<String, LarkError> {
        if self.event_encrypt_key.is_empty() {
            return Ok(cipher_event_json.to_string());
        }
        crypto::event_decrypt(&self.event_encrypt_key, cipher_event_json)
    }

    pub async fn handle(&self, req: EventReq) -> EventResp {
        match self.do_handle(req).await {
            Ok(resp) => resp,
            Err(e) => {
                tracing::error!("event handler error: {e}");
                EventResp::error(500, &e.to_string())
            }
        }
    }

    async fn do_handle(&self, req: EventReq) -> Result<EventResp, LarkError> {
        let body_str = std::str::from_utf8(&req.body)
            .map_err(|e| LarkError::Event(format!("invalid utf8 body: {e}")))?;

        let body_str = decrypt_if_needed(&self.event_encrypt_key, body_str)?;

        let parsed: EventV2Body = serde_json::from_str(&body_str)
            .map_err(|e| LarkError::Event(format!("failed to parse event body: {e}")))?;

        if parsed.req_type.as_deref() == Some("url_verification") {
            return self.handle_url_verification(&parsed);
        }

        if !self.skip_sign_verify {
            self.verify_signature(&req)?;
        }

        let event_type = parsed
            .header
            .as_ref()
            .map(|h| h.event_type.as_str())
            .unwrap_or_else(|| {
                // P1 protocol: event type is in event.type
                parsed.event["type"].as_str().unwrap_or_default()
            });

        if let Some(handler) = self.callback_handlers.get(event_type) {
            let event_data = parsed.event.clone();
            let resp_data = handler(event_data).await?;
            return Ok(EventResp::success(resp_data));
        }

        if let Some(handler) = self.customized_event_handlers.get(event_type) {
            handler(req, parsed).await?;
            return Ok(EventResp::success(serde_json::json!({ "msg": "success" })));
        }

        if let Some(handler) = self.event_handlers.get(event_type) {
            let event_data = parsed.event.clone();
            handler(event_data).await?;
            return Ok(EventResp::success(serde_json::json!({ "msg": "success" })));
        }

        tracing::warn!("no handler registered for event type: {event_type}");
        Ok(EventResp::success(serde_json::json!({
            "msg": format!("no handler for event type: {event_type}")
        })))
    }

    fn handle_url_verification(&self, parsed: &EventV2Body) -> Result<EventResp, LarkError> {
        if let Some(ref token) = parsed.token
            && token != &self.verification_token
            && !self.verification_token.is_empty()
        {
            return Err(LarkError::Event("verification token mismatch".to_string()));
        }

        let challenge = parsed.challenge.as_deref().unwrap_or_default();
        Ok(EventResp::success(
            serde_json::json!({ "challenge": challenge }),
        ))
    }

    fn verify_signature(&self, req: &EventReq) -> Result<(), LarkError> {
        if self.event_encrypt_key.is_empty() {
            return Ok(());
        }

        let (timestamp, nonce, sig) = extract_signature_headers(&req.headers)?;

        if !crypto::verify_signature_sha256(
            &timestamp,
            &nonce,
            &self.event_encrypt_key,
            &req.body,
            &sig,
        ) {
            return Err(LarkError::Event(
                "signature verification failed".to_string(),
            ));
        }

        Ok(())
    }
}

/// Return type for card action handlers.
///
/// Handlers can return either a JSON value (served as HTTP 200) or a
/// [`CustomResp`] with a custom HTTP status code and body.
#[derive(Debug, Clone)]
pub enum CardHandlerResult {
    Json(serde_json::Value),
    Custom(CustomResp),
}

impl From<serde_json::Value> for CardHandlerResult {
    fn from(val: serde_json::Value) -> Self {
        Self::Json(val)
    }
}

impl From<CustomResp> for CardHandlerResult {
    fn from(resp: CustomResp) -> Self {
        Self::Custom(resp)
    }
}

/// Custom card response with an explicit HTTP status code.
#[derive(Debug, Clone)]
pub struct CustomResp {
    pub status_code: u16,
    pub body: serde_json::Map<String, serde_json::Value>,
}

pub type CardHandlerFn = Arc<
    dyn Fn(CardAction) -> Pin<Box<dyn Future<Output = Result<CardHandlerResult, LarkError>> + Send>>
        + Send
        + Sync,
>;

#[must_use]
pub struct CardActionHandler {
    verification_token: String,
    event_encrypt_key: String,
    handler: CardHandlerFn,
    skip_sign_verify: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardAction {
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub open_message_id: String,
    #[serde(default)]
    pub open_chat_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub timezone: String,
    #[serde(default)]
    pub action: serde_json::Value,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub delivery_type: String,
    /// The raw HTTP request that delivered this card action.
    /// Populated by [`CardActionHandler`] before calling the handler.
    #[serde(skip)]
    pub req: Option<EventReq>,
}

impl CardActionHandler {
    /// Create a card action handler that returns a JSON response with HTTP 200.
    pub fn new<F, Fut>(
        verification_token: impl Into<String>,
        event_encrypt_key: impl Into<String>,
        handler: F,
    ) -> Self
    where
        F: Fn(CardAction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<serde_json::Value, LarkError>> + Send + 'static,
    {
        Self {
            verification_token: verification_token.into(),
            event_encrypt_key: event_encrypt_key.into(),
            handler: Arc::new(
                move |action: CardAction| -> Pin<
                    Box<dyn Future<Output = Result<CardHandlerResult, LarkError>> + Send>,
                > {
                    let fut = handler(action);
                    Box::pin(async move { fut.await.map(CardHandlerResult::Json) })
                },
            ),
            skip_sign_verify: false,
        }
    }

    /// Create a card action handler that can return a [`CustomResp`] with a
    /// custom HTTP status code and body.
    pub fn new_custom<F, Fut>(
        verification_token: impl Into<String>,
        event_encrypt_key: impl Into<String>,
        handler: F,
    ) -> Self
    where
        F: Fn(CardAction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<CardHandlerResult, LarkError>> + Send + 'static,
    {
        Self {
            verification_token: verification_token.into(),
            event_encrypt_key: event_encrypt_key.into(),
            handler: Arc::new(
                move |action: CardAction| -> Pin<
                    Box<dyn Future<Output = Result<CardHandlerResult, LarkError>> + Send>,
                > { Box::pin(handler(action)) },
            ),
            skip_sign_verify: false,
        }
    }

    pub fn skip_sign_verify(mut self) -> Self {
        self.skip_sign_verify = true;
        self
    }

    pub async fn handle(&self, req: EventReq) -> EventResp {
        match self.do_handle(req).await {
            Ok(resp) => resp,
            Err(e) => {
                tracing::error!("card action handler error: {e}");
                EventResp::error(500, &e.to_string())
            }
        }
    }

    async fn do_handle(&self, req: EventReq) -> Result<EventResp, LarkError> {
        let body_str = std::str::from_utf8(&req.body)
            .map_err(|e| LarkError::Event(format!("invalid utf8 body: {e}")))?;

        let body_str = decrypt_if_needed(&self.event_encrypt_key, body_str)?;

        let parsed: serde_json::Value = serde_json::from_str(&body_str)
            .map_err(|e| LarkError::Event(format!("failed to parse card body: {e}")))?;

        if parsed.get("type").and_then(|v| v.as_str()) == Some("url_verification") {
            return self.handle_challenge(&parsed);
        }

        if !self.skip_sign_verify {
            self.verify_signature_sha1(&req.headers, &body_str)?;
        }

        let mut action: CardAction = serde_json::from_value(parsed)
            .map_err(|e| LarkError::Event(format!("failed to parse card action: {e}")))?;
        action.req = Some(req);

        let result = (self.handler)(action).await?;
        match result {
            CardHandlerResult::Json(val) => Ok(EventResp::success(val)),
            CardHandlerResult::Custom(custom) => {
                let status = if custom.status_code == 0 {
                    200
                } else {
                    custom.status_code
                };
                let body_bytes = serde_json::to_vec(&custom.body).unwrap_or_default();
                Ok(EventResp {
                    status_code: status,
                    headers: HashMap::from([(
                        "Content-Type".to_string(),
                        "application/json".to_string(),
                    )]),
                    body: body_bytes,
                })
            }
        }
    }

    fn handle_challenge(&self, parsed: &serde_json::Value) -> Result<EventResp, LarkError> {
        if let Some(token) = parsed.get("token").and_then(|v| v.as_str())
            && token != self.verification_token
            && !self.verification_token.is_empty()
        {
            return Err(LarkError::Event("verification token mismatch".to_string()));
        }

        let challenge = parsed
            .get("challenge")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        Ok(EventResp::success(
            serde_json::json!({ "challenge": challenge }),
        ))
    }

    fn verify_signature_sha1(
        &self,
        headers: &HashMap<String, Vec<String>>,
        body: &str,
    ) -> Result<(), LarkError> {
        if self.verification_token.is_empty() {
            return Ok(());
        }

        let (timestamp, nonce, sig) = extract_signature_headers(headers)?;

        if !crypto::verify_signature_sha1(&timestamp, &nonce, &self.verification_token, body, &sig)
        {
            return Err(LarkError::Event(
                "card signature verification failed".to_string(),
            ));
        }

        Ok(())
    }
}

fn get_header_ref<'a>(headers: &'a HashMap<String, Vec<String>>, key: &str) -> Option<&'a str> {
    headers.get(key).and_then(|v| v.first()).map(|s| s.as_str())
}

fn get_header(headers: &HashMap<String, Vec<String>>, key: &str) -> String {
    headers
        .get(key)
        .and_then(|v| v.first())
        .cloned()
        .unwrap_or_default()
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

// ── Typed callback types ──

/// Operator info for card action trigger and URL preview callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallbackOperator {
    #[serde(default)]
    pub tenant_key: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub open_id: String,
}

/// Context for card action trigger and URL preview callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallbackContext {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub preview_token: String,
    #[serde(default)]
    pub open_message_id: String,
    #[serde(default)]
    pub open_chat_id: String,
}

/// Action detail for card action trigger callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallbackAction {
    #[serde(default)]
    pub value: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub option: String,
    #[serde(default)]
    pub timezone: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub form_value: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    pub input_value: String,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub checked: bool,
}

/// Request payload for `card.action.trigger` callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardActionTriggerRequest {
    #[serde(default)]
    pub operator: Option<CallbackOperator>,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub action: Option<CallbackAction>,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub delivery_type: String,
    #[serde(default)]
    pub context: Option<CallbackContext>,
}

/// Toast notification in a card action trigger response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Toast {
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub toast_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n: Option<HashMap<String, String>>,
}

impl Toast {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            toast_type: Some("info".to_string()),
            content: Some(content.into()),
            i18n: None,
        }
    }

    pub fn toast_type(mut self, t: impl Into<String>) -> Self {
        self.toast_type = Some(t.into());
        self
    }

    pub fn i18n(mut self, i18n: ToastI18n) -> Self {
        let mut map = HashMap::new();
        if let Some(v) = i18n.zh_cn {
            map.insert("zh_cn".to_string(), v);
        }
        if let Some(v) = i18n.en_us {
            map.insert("en_us".to_string(), v);
        }
        if let Some(v) = i18n.ja_jp {
            map.insert("ja_jp".to_string(), v);
        }
        if let Some(v) = i18n.zh_hk {
            map.insert("zh_hk".to_string(), v);
        }
        if let Some(v) = i18n.zh_tw {
            map.insert("zh_tw".to_string(), v);
        }
        if let Some(v) = i18n.id_id {
            map.insert("id_id".to_string(), v);
        }
        if let Some(v) = i18n.vi_vn {
            map.insert("vi_vn".to_string(), v);
        }
        if let Some(v) = i18n.th_th {
            map.insert("th_th".to_string(), v);
        }
        if let Some(v) = i18n.pt_br {
            map.insert("pt_br".to_string(), v);
        }
        if let Some(v) = i18n.es_es {
            map.insert("es_es".to_string(), v);
        }
        if let Some(v) = i18n.ko_kr {
            map.insert("ko_kr".to_string(), v);
        }
        if let Some(v) = i18n.de_de {
            map.insert("de_de".to_string(), v);
        }
        if let Some(v) = i18n.fr_fr {
            map.insert("fr_fr".to_string(), v);
        }
        if let Some(v) = i18n.it_it {
            map.insert("it_it".to_string(), v);
        }
        if let Some(v) = i18n.ru_ru {
            map.insert("ru_ru".to_string(), v);
        }
        if let Some(v) = i18n.ms_my {
            map.insert("ms_my".to_string(), v);
        }
        self.i18n = Some(map);
        self
    }
}

/// Typed locale fields for toast I18n, matching Go SDK's `I18n` struct.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToastI18n {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_hk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_tw: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vi_vn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th_th: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt_br: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_es: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ko_kr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub de_de: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr_fr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub it_it: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ru_ru: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_my: Option<String>,
}

/// Card reference in callback responses (template or raw card JSON).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallbackCard {
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub card_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl CallbackCard {
    /// Create a template-based card response.
    pub fn template(card: TemplateCard) -> Self {
        Self {
            card_type: Some("template".to_string()),
            data: serde_json::to_value(card).ok(),
        }
    }
}

/// Template card with ID, version, and variable bindings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_variable: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version_name: Option<String>,
}

/// Response for `card.action.trigger` callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardActionTriggerResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toast: Option<Toast>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CallbackCard>,
}

/// Request payload for `url.preview.get` callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct URLPreviewGetRequest {
    #[serde(default)]
    pub operator: Option<CallbackOperator>,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub context: Option<CallbackContext>,
}

/// Inline preview in a URL preview response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlinePreview {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_title: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<PreviewUrl>,
}

/// Multi-platform URL for inline previews.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreviewUrl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
}

/// Response for `url.preview.get` callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct URLPreviewGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<InlinePreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CallbackCard>,
}
