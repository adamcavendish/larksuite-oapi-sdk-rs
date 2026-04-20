use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::crypto;
use crate::error::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventReq {
    #[serde(default)]
    pub headers: HashMap<String, Vec<String>>,
    pub body: Vec<u8>,
    #[serde(default)]
    pub request_uri: String,
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
    dyn Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync,
>;

pub type CallbackHandlerFn = Arc<
    dyn Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<serde_json::Value>> + Send>>
        + Send
        + Sync,
>;

pub struct EventDispatcher {
    event_handlers: HashMap<String, EventHandlerFn>,
    callback_handlers: HashMap<String, CallbackHandlerFn>,
    verification_token: String,
    event_encrypt_key: String,
    config: Option<Config>,
}

impl EventDispatcher {
    pub fn new(
        verification_token: impl Into<String>,
        event_encrypt_key: impl Into<String>,
    ) -> Self {
        Self {
            event_handlers: HashMap::new(),
            callback_handlers: HashMap::new(),
            verification_token: verification_token.into(),
            event_encrypt_key: event_encrypt_key.into(),
            config: None,
        }
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn on_event<F, Fut>(mut self, event_type: impl Into<String>, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let event_type = event_type.into();
        let handler = Arc::new(
            move |val: serde_json::Value| -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
                Box::pin(handler(val))
            },
        );
        self.event_handlers.insert(event_type, handler);
        self
    }

    pub fn on_callback<F, Fut>(mut self, callback_type: impl Into<String>, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<serde_json::Value>> + Send + 'static,
    {
        let callback_type = callback_type.into();
        let handler = Arc::new(move |val: serde_json::Value| -> Pin<Box<dyn Future<Output = Result<serde_json::Value>> + Send>> {
            Box::pin(handler(val))
        });
        self.callback_handlers.insert(callback_type, handler);
        self
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

    async fn do_handle(&self, req: EventReq) -> Result<EventResp> {
        let body_str = String::from_utf8(req.body.clone())
            .map_err(|e| Error::Event(format!("invalid utf8 body: {e}")))?;

        let body_str = decrypt_if_needed(&self.event_encrypt_key, &body_str)?;

        let parsed: EventV2Body = serde_json::from_str(&body_str)
            .map_err(|e| Error::Event(format!("failed to parse event body: {e}")))?;

        if parsed.req_type.as_deref() == Some("url_verification") {
            return self.handle_url_verification(&parsed);
        }

        if !self.skip_sign_verify() {
            self.verify_signature(&req)?;
        }

        let event_type = parsed
            .header
            .as_ref()
            .map(|h| h.event_type.as_str())
            .unwrap_or_default();

        if let Some(handler) = self.callback_handlers.get(event_type) {
            let event_data = parsed.event.clone();
            let resp_data = handler(event_data).await?;
            return Ok(EventResp::success(resp_data));
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

    fn handle_url_verification(&self, parsed: &EventV2Body) -> Result<EventResp> {
        if let Some(ref token) = parsed.token
            && token != &self.verification_token
            && !self.verification_token.is_empty()
        {
            return Err(Error::Event("verification token mismatch".to_string()));
        }

        let challenge = parsed.challenge.as_deref().unwrap_or_default();
        Ok(EventResp::success(
            serde_json::json!({ "challenge": challenge }),
        ))
    }

    fn verify_signature(&self, req: &EventReq) -> Result<()> {
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
            return Err(Error::Event("signature verification failed".to_string()));
        }

        Ok(())
    }

    fn skip_sign_verify(&self) -> bool {
        self.config.as_ref().is_some_and(|c| c.skip_sign_verify)
    }
}

pub type CardHandlerFn = Arc<
    dyn Fn(CardAction) -> Pin<Box<dyn Future<Output = Result<serde_json::Value>> + Send>>
        + Send
        + Sync,
>;

pub struct CardActionHandler {
    verification_token: String,
    event_encrypt_key: String,
    handler: CardHandlerFn,
    config: Option<Config>,
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
    pub action: serde_json::Value,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub delivery_type: String,
}

impl CardActionHandler {
    pub fn new<F, Fut>(
        verification_token: impl Into<String>,
        event_encrypt_key: impl Into<String>,
        handler: F,
    ) -> Self
    where
        F: Fn(CardAction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<serde_json::Value>> + Send + 'static,
    {
        Self {
            verification_token: verification_token.into(),
            event_encrypt_key: event_encrypt_key.into(),
            handler: Arc::new(move |action: CardAction| -> Pin<Box<dyn Future<Output = Result<serde_json::Value>> + Send>> {
                Box::pin(handler(action))
            }),
            config: None,
        }
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
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

    async fn do_handle(&self, req: EventReq) -> Result<EventResp> {
        let body_str = String::from_utf8(req.body)
            .map_err(|e| Error::Event(format!("invalid utf8 body: {e}")))?;

        let body_str = decrypt_if_needed(&self.event_encrypt_key, &body_str)?;

        let parsed: serde_json::Value = serde_json::from_str(&body_str)
            .map_err(|e| Error::Event(format!("failed to parse card body: {e}")))?;

        if parsed.get("type").and_then(|v| v.as_str()) == Some("url_verification") {
            return self.handle_challenge(&parsed);
        }

        if !self.skip_sign_verify() {
            self.verify_signature_sha1(&req.headers, &body_str)?;
        }

        let action: CardAction = serde_json::from_value(parsed)
            .map_err(|e| Error::Event(format!("failed to parse card action: {e}")))?;

        let resp = (self.handler)(action).await?;
        Ok(EventResp::success(resp))
    }

    fn handle_challenge(&self, parsed: &serde_json::Value) -> Result<EventResp> {
        if let Some(token) = parsed.get("token").and_then(|v| v.as_str())
            && token != self.verification_token
            && !self.verification_token.is_empty()
        {
            return Err(Error::Event("verification token mismatch".to_string()));
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
    ) -> Result<()> {
        if self.event_encrypt_key.is_empty() {
            return Ok(());
        }

        let (timestamp, nonce, sig) = extract_signature_headers(headers)?;

        if !crypto::verify_signature_sha1(&timestamp, &nonce, &self.event_encrypt_key, body, &sig) {
            return Err(Error::Event(
                "card signature verification failed".to_string(),
            ));
        }

        Ok(())
    }

    fn skip_sign_verify(&self) -> bool {
        self.config.as_ref().is_some_and(|c| c.skip_sign_verify)
    }
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
) -> Result<(String, String, String)> {
    let timestamp = get_header(headers, "X-Lark-Request-Timestamp");
    let nonce = get_header(headers, "X-Lark-Request-Nonce");
    let sig = get_header(headers, "X-Lark-Signature");

    if sig.is_empty() {
        return Err(Error::Event("missing X-Lark-Signature header".to_string()));
    }

    Ok((timestamp, nonce, sig))
}

fn decrypt_if_needed(encrypt_key: &str, body: &str) -> Result<String> {
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
