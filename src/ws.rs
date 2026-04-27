//! Lark WebSocket long-connection event client.
//!
//! [`WsClient`] connects to the Lark WebSocket gateway, handles protobuf-framed
//! messages, reassembles fragmented payloads, and dispatches events to an
//! [`EventDispatcher`].
//!
//! # Example
//!
//! ```rust,no_run
//! use larksuite_oapi_sdk_rs::{Client, EventDispatcher};
//!
//! # #[tokio::main]
//! # async fn main() {
//! let client = Client::builder("APP_ID", "APP_SECRET").build();
//! let dispatcher = EventDispatcher::new("VERIFICATION_TOKEN", "ENCRYPT_KEY");
//! client.ws_client(dispatcher).start().await.unwrap();
//! # }
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

use futures_util::{SinkExt as _, StreamExt as _};
use prost::Message as ProstMessage;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::event::EventDispatcher;

// ── Generated protobuf types ──

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/larksuite.ws.rs"));
}

// Frame method constants (matches Go SDK FrameType)
const METHOD_CONTROL: i32 = 0;
const METHOD_DATA: i32 = 1;

// Message type header values
const MSG_TYPE_EVENT: &str = "event";
const MSG_TYPE_PING: &str = "ping";
const MSG_TYPE_PONG: &str = "pong";

// Header key constants
const HEADER_TYPE: &str = "type";
const HEADER_SUM: &str = "sum";
const HEADER_SEQ: &str = "seq";
const HEADER_MESSAGE_ID: &str = "message_id";
const HEADER_BIZ_RT: &str = "biz_rt";

// ── WS endpoint response ──

#[derive(serde::Deserialize)]
struct WsEndpointResp {
    code: i32,
    msg: Option<String>,
    data: Option<WsEndpointData>,
}

#[derive(serde::Deserialize)]
struct WsEndpointData {
    #[serde(rename = "URL")]
    url: String,
    #[serde(rename = "ClientConfig")]
    client_config: Option<ClientConfig>,
}

#[derive(serde::Deserialize, Debug, Clone, Default)]
#[allow(dead_code)]
struct ClientConfig {
    #[serde(rename = "ReconnectCount", default)]
    reconnect_count: i32,
    #[serde(rename = "ReconnectInterval", default)]
    reconnect_interval: u64,
    #[serde(rename = "ReconnectNonce", default)]
    reconnect_nonce: u64,
    #[serde(rename = "PingInterval", default)]
    ping_interval: u64,
}

// ── ACK response payload ──

#[derive(serde::Serialize)]
struct AckResponse {
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

// ── Fragment reassembly buffer ──

struct FragEntry {
    frames: Vec<Option<Vec<u8>>>,
    created: Instant,
}

impl FragEntry {
    fn new(sum: usize) -> Self {
        Self {
            frames: vec![None; sum],
            created: Instant::now(),
        }
    }

    fn insert(&mut self, index: usize, data: Vec<u8>) {
        if index < self.frames.len() {
            self.frames[index] = Some(data);
        }
    }

    fn complete(&self) -> bool {
        self.frames.iter().all(|f| f.is_some())
    }

    fn assemble(self) -> Vec<u8> {
        self.frames.into_iter().flatten().flatten().collect()
    }

    fn expired(&self) -> bool {
        self.created.elapsed() > Duration::from_secs(5)
    }
}

// ── Header helpers ──

fn get_header(headers: &[proto::Header], key: &str) -> Option<String> {
    headers
        .iter()
        .find(|h| h.key == key)
        .map(|h| h.value.clone())
}

fn get_header_int(headers: &[proto::Header], key: &str) -> i32 {
    get_header(headers, key)
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

fn make_header(key: &str, value: &str) -> proto::Header {
    proto::Header {
        key: key.to_string(),
        value: value.to_string(),
    }
}

// ── WebSocket client ──

pub struct WsClient {
    config: Arc<Config>,
    dispatcher: Arc<EventDispatcher>,
    ping_interval: Arc<AtomicU64>,
    reconnect_interval: Arc<AtomicU64>,
    reconnect_nonce: Arc<AtomicU64>,
    auto_reconnect: Arc<AtomicBool>,
}

impl WsClient {
    pub fn new(config: Config, dispatcher: EventDispatcher) -> Self {
        Self {
            config: Arc::new(config),
            dispatcher: Arc::new(dispatcher),
            ping_interval: Arc::new(AtomicU64::new(120)),
            reconnect_interval: Arc::new(AtomicU64::new(120)),
            reconnect_nonce: Arc::new(AtomicU64::new(30)),
            auto_reconnect: Arc::new(AtomicBool::new(true)),
        }
    }

    /// Disable automatic reconnection. When set to `false`, `start()` will
    /// return after the first connection closes instead of reconnecting.
    pub fn auto_reconnect(self, enable: bool) -> Self {
        self.auto_reconnect.store(enable, Ordering::Relaxed);
        self
    }

    fn apply_config(&self, conf: &ClientConfig) {
        if conf.ping_interval > 0 {
            self.ping_interval
                .store(conf.ping_interval, Ordering::Relaxed);
        }
        if conf.reconnect_interval > 0 {
            self.reconnect_interval
                .store(conf.reconnect_interval, Ordering::Relaxed);
        }
        if conf.reconnect_nonce > 0 {
            self.reconnect_nonce
                .store(conf.reconnect_nonce, Ordering::Relaxed);
        }
    }

    /// Run the WebSocket connection. If auto-reconnect is enabled (default),
    /// reconnects indefinitely on errors. Otherwise returns after one session.
    pub async fn start(self) -> Result<()> {
        let mut consecutive_failures: u32 = 0;
        loop {
            match self.run_once().await {
                Ok(()) => {
                    tracing::info!("ws connection closed");
                    consecutive_failures = 0;
                }
                Err(e) => {
                    tracing::warn!("ws connection error: {e}");
                    consecutive_failures += 1;
                }
            }
            if !self.auto_reconnect.load(Ordering::Relaxed) {
                return Ok(());
            }
            // Fast reconnect (5s) on first failure after a working connection.
            // Use server-configured interval on repeated failures to avoid
            // hammering the endpoint.
            let wait = if consecutive_failures <= 1 {
                Duration::from_secs(5)
            } else {
                let interval = self.reconnect_interval.load(Ordering::Relaxed);
                let nonce = self.reconnect_nonce.load(Ordering::Relaxed);
                let jitter = if nonce > 0 {
                    uuid::Uuid::new_v4().as_u64_pair().0 % nonce
                } else {
                    0
                };
                Duration::from_secs(interval + jitter)
            };
            tracing::info!("reconnecting in {wait:?}");
            sleep(wait).await;
        }
    }

    async fn run_once(&self) -> Result<()> {
        let (url, service_id) = self.get_ws_url().await?;
        tracing::info!("connecting to ws endpoint: {url}");

        let (ws_stream, _) = connect_async(&url)
            .await
            .map_err(|e| Error::Event(format!("ws connect failed: {e}")))?;

        tracing::info!("ws connected");

        let mut pending_frags: HashMap<String, FragEntry> = HashMap::new();

        let (write, mut read) = ws_stream.split();
        let write = Arc::new(Mutex::new(write));

        // Start ping loop
        let ping_write = write.clone();
        let ping_interval = self.ping_interval.clone();
        let ping_service_id = service_id;
        let ping_handle = tokio::spawn(async move {
            loop {
                let secs = ping_interval.load(Ordering::Relaxed);
                sleep(Duration::from_secs(secs)).await;
                let frame = proto::Frame {
                    seq_id: 0,
                    log_id: 0,
                    service: ping_service_id,
                    method: METHOD_CONTROL,
                    headers: vec![make_header(HEADER_TYPE, MSG_TYPE_PING)],
                    payload_encoding: None,
                    payload_type: None,
                    payload: None,
                    log_id_new: None,
                };
                let encoded = frame.encode_to_vec();
                let mut w = ping_write.lock().await;
                if (*w).send(Message::Binary(encoded.into())).await.is_err() {
                    break;
                }
                tracing::debug!("ws ping sent");
            }
        });

        while let Some(msg) = read.next().await {
            let msg = match msg {
                Ok(m) => m,
                Err(e) => {
                    tracing::warn!("ws recv error: {e}");
                    break;
                }
            };
            match msg {
                Message::Binary(data) => {
                    if let Err(e) = self
                        .handle_binary_message(&data, &mut pending_frags, &write)
                        .await
                    {
                        tracing::warn!("ws frame handling error: {e}");
                    }
                }
                Message::Close(_) => {
                    tracing::info!("ws server closed connection");
                    break;
                }
                _ => {}
            }
        }

        ping_handle.abort();
        Ok(())
    }

    async fn handle_binary_message<S>(
        &self,
        data: &[u8],
        pending_frags: &mut HashMap<String, FragEntry>,
        write: &Arc<Mutex<S>>,
    ) -> Result<()>
    where
        S: futures_util::Sink<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
    {
        let frame = proto::Frame::decode(data)
            .map_err(|e| Error::Event(format!("proto frame decode error: {e}")))?;

        match frame.method {
            METHOD_CONTROL => {
                let msg_type = get_header(&frame.headers, HEADER_TYPE).unwrap_or_default();
                match msg_type.as_str() {
                    MSG_TYPE_PING => {
                        tracing::debug!("ws ping received, seq={}", frame.seq_id);
                        let pong = proto::Frame {
                            method: METHOD_CONTROL,
                            service: frame.service,
                            seq_id: frame.seq_id,
                            log_id: frame.log_id,
                            headers: vec![make_header(HEADER_TYPE, MSG_TYPE_PONG)],
                            payload_encoding: None,
                            payload_type: None,
                            payload: None,
                            log_id_new: None,
                        };
                        let encoded = pong.encode_to_vec();
                        let mut w = write.lock().await;
                        let _ = (*w).send(Message::Binary(encoded.into())).await;
                    }
                    MSG_TYPE_PONG => {
                        tracing::debug!("ws pong received");
                        if let Some(payload) = &frame.payload
                            && !payload.is_empty()
                            && let Ok(conf) = serde_json::from_slice::<ClientConfig>(payload)
                        {
                            tracing::debug!("ws client config updated: {conf:?}");
                            self.apply_config(&conf);
                        }
                    }
                    _ => {
                        tracing::debug!("ws unknown control type: {msg_type}");
                    }
                }
            }
            METHOD_DATA => {
                let msg_type = get_header(&frame.headers, HEADER_TYPE).unwrap_or_default();

                // Card frames are not handled (same as Go SDK)
                if msg_type != MSG_TYPE_EVENT {
                    return Ok(());
                }

                let sum = get_header_int(&frame.headers, HEADER_SUM);
                let seq = get_header_int(&frame.headers, HEADER_SEQ);
                let msg_id = get_header(&frame.headers, HEADER_MESSAGE_ID).unwrap_or_default();

                let payload = if sum <= 1 {
                    frame.payload.clone().unwrap_or_default()
                } else {
                    // Clean expired entries
                    pending_frags.retain(|_, v| !v.expired());

                    let entry = pending_frags
                        .entry(msg_id.clone())
                        .or_insert_with(|| FragEntry::new(sum as usize));
                    entry.insert(seq as usize, frame.payload.clone().unwrap_or_default());
                    if entry.complete() {
                        match pending_frags.remove(&msg_id) {
                            Some(e) => e.assemble(),
                            None => return Ok(()),
                        }
                    } else {
                        return Ok(());
                    }
                };

                let start = Instant::now();
                let status = self.dispatch_event(payload).await;
                let biz_rt = start.elapsed().as_millis().to_string();

                // Send ACK frame
                let ack_code = if status { 200u16 } else { 500 };
                let ack_payload = serde_json::to_vec(&AckResponse {
                    code: ack_code,
                    headers: None,
                    data: None,
                })
                .unwrap_or_default();

                let mut ack_headers = frame.headers.clone();
                ack_headers.push(make_header(HEADER_BIZ_RT, &biz_rt));

                let ack_frame = proto::Frame {
                    seq_id: frame.seq_id,
                    log_id: frame.log_id,
                    service: frame.service,
                    method: frame.method,
                    headers: ack_headers,
                    payload_encoding: frame.payload_encoding.clone(),
                    payload_type: frame.payload_type.clone(),
                    payload: Some(ack_payload),
                    log_id_new: frame.log_id_new.clone(),
                };
                let encoded = ack_frame.encode_to_vec();
                let mut w = write.lock().await;
                let _ = (*w).send(Message::Binary(encoded.into())).await;
            }
            _ => {
                tracing::debug!("ws unknown method: {}", frame.method);
            }
        }

        Ok(())
    }

    async fn dispatch_event(&self, payload: Vec<u8>) -> bool {
        use crate::event::EventReq;

        let req = EventReq {
            headers: HashMap::new(),
            body: payload,
            request_uri: String::new(),
        };

        let resp = self.dispatcher.handle(req).await;
        if resp.status_code != 200 {
            tracing::warn!(
                "event dispatch returned {}: {}",
                resp.status_code,
                String::from_utf8_lossy(&resp.body)
            );
            return false;
        }

        true
    }

    async fn get_ws_url(&self) -> Result<(String, i32)> {
        let base = self.config.base_url.trim_end_matches('/');
        let url = format!("{base}/callback/ws/endpoint");

        let body = serde_json::json!({
            "AppID": self.config.app_id,
            "AppSecret": self.config.app_secret,
        });

        let resp = self
            .config
            .http_client
            .post(&url)?
            .header_str("locale", "zh")
            .map_err(|e| Error::Event(format!("invalid locale header: {e}")))?
            .json(&body)
            .map_err(|e| Error::Event(format!("failed to serialize endpoint body: {e}")))?
            .send()
            .await?;

        let body: WsEndpointResp = resp.json().await?;

        if body.code != 0 {
            return Err(Error::Event(format!(
                "ws endpoint returned code {}: {}",
                body.code,
                body.msg.unwrap_or_default()
            )));
        }

        let data = body
            .data
            .ok_or_else(|| Error::Event("ws endpoint returned no data".to_string()))?;

        if let Some(ref conf) = data.client_config {
            self.apply_config(conf);
        }

        // Extract service_id from URL query params
        let service_id = data
            .url
            .split('?')
            .nth(1)
            .and_then(|qs| {
                qs.split('&').find_map(|pair| {
                    let (k, v) = pair.split_once('=')?;
                    if k == "service_id" {
                        v.parse::<i32>().ok()
                    } else {
                        None
                    }
                })
            })
            .unwrap_or(0);

        Ok((data.url, service_id))
    }
}
