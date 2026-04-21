//! Lark WebSocket long-connection event client.
//!
//! [`WsClient`] connects to the Lark WebSocket gateway, handles protobuf-framed
//! messages, reassembles fragmented payloads, and dispatches events to an
//! [`EventDispatcher`].
//!
//! # Example
//!
//! ```rust,no_run
//! use larksuite_oapi_sdk_rs::{Config, EventDispatcher};
//! use larksuite_oapi_sdk_rs::ws::WsClient;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let config = Config::new("APP_ID", "APP_SECRET");
//! let dispatcher = EventDispatcher::new("VERIFICATION_TOKEN", "ENCRYPT_KEY");
//! WsClient::new(config, dispatcher).start().await.unwrap();
//! # }
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use prost::Message as ProstMessage;
use tokio::time::sleep;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::event::EventDispatcher;
use crate::token::TokenManager;

// ── Generated protobuf types ──

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/larksuite.ws.rs"));
}

// Frame type constants (matches Lark WS gateway protocol)
const FRAME_TYPE_DATA: u32 = 0;
const FRAME_TYPE_CONTROL: u32 = 1;
const FRAME_TYPE_PONG: u32 = 2;

// Control sub-types (in Frame.headers["type"])
const CTRL_TYPE_PING: &str = "ping";
const CTRL_TYPE_CLOSE: &str = "close";

// ── WS endpoint response ──

#[derive(serde::Deserialize)]
struct WsEndpointResp {
    code: i32,
    msg: Option<String>,
    data: Option<WsEndpointData>,
}

#[derive(serde::Deserialize)]
struct WsEndpointData {
    url: String,
}

// ── Fragment reassembly buffer ──

struct FragBuffer {
    frames: Vec<Option<Vec<u8>>>,
}

impl FragBuffer {
    fn new(sum: usize) -> Self {
        Self {
            frames: vec![None; sum],
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
}

// ── WebSocket client ──

pub struct WsClient {
    config: Arc<Config>,
    dispatcher: Arc<EventDispatcher>,
}

impl WsClient {
    pub fn new(config: Config, dispatcher: EventDispatcher) -> Self {
        Self {
            config: Arc::new(config),
            dispatcher: Arc::new(dispatcher),
        }
    }

    /// Run forever, reconnecting on errors with exponential backoff.
    pub async fn start(self) -> Result<()> {
        let mut backoff = Duration::from_secs(1);
        loop {
            match self.run_once().await {
                Ok(()) => {
                    tracing::info!("ws connection closed, reconnecting");
                    backoff = Duration::from_secs(1);
                }
                Err(e) => {
                    tracing::warn!("ws connection error: {e}, reconnecting in {backoff:?}");
                    sleep(backoff).await;
                    backoff = (backoff * 2).min(Duration::from_secs(60));
                }
            }
        }
    }

    async fn run_once(&self) -> Result<()> {
        let url = self.get_ws_url().await?;
        tracing::info!("connecting to ws endpoint: {url}");

        let (ws_stream, _) = connect_async(&url)
            .await
            .map_err(|e| Error::Event(format!("ws connect failed: {e}")))?;

        tracing::info!("ws connected");

        let mut pending_frags: HashMap<u64, FragBuffer> = HashMap::new();

        use futures_util::{SinkExt as _, StreamExt as _};
        let (mut write, mut read) = ws_stream.split();

        while let Some(msg) = read.next().await {
            let msg = msg.map_err(|e| Error::Event(format!("ws recv error: {e}")))?;
            match msg {
                Message::Binary(data) => {
                    if let Some(pong) = self
                        .handle_binary_message(&data, &mut pending_frags)
                        .await?
                    {
                        let _ = write.send(pong).await;
                    }
                }
                Message::Close(_) => {
                    tracing::info!("ws server closed connection");
                    return Ok(());
                }
                _ => {}
            }
        }

        Ok(())
    }

    async fn handle_binary_message(
        &self,
        data: &[u8],
        pending_frags: &mut HashMap<u64, FragBuffer>,
    ) -> Result<Option<Message>> {
        let frame = proto::Frame::decode(data)
            .map_err(|e| Error::Event(format!("proto frame decode error: {e}")))?;

        match frame.frame_type {
            FRAME_TYPE_CONTROL => {
                let ctrl_type = frame
                    .headers
                    .get("type")
                    .map(|s| s.as_str())
                    .unwrap_or_default();
                match ctrl_type {
                    CTRL_TYPE_PING => {
                        tracing::debug!("ws ping received, seq={}", frame.seq_id);
                        let pong = proto::Frame {
                            frame_type: FRAME_TYPE_PONG,
                            seq_id: frame.seq_id,
                            ..Default::default()
                        };
                        let encoded = pong.encode_to_vec();
                        return Ok(Some(Message::Binary(encoded.into())));
                    }
                    CTRL_TYPE_CLOSE => {
                        tracing::info!("ws close control received");
                    }
                    _ => {}
                }
            }
            FRAME_TYPE_DATA => {
                let payload = if frame.sum <= 1 {
                    frame.message.to_vec()
                } else {
                    let sum = frame.sum as usize;
                    let index = frame.index as usize;
                    let buf = pending_frags
                        .entry(frame.seq_id)
                        .or_insert_with(|| FragBuffer::new(sum));
                    buf.insert(index, frame.message.to_vec());
                    if buf.complete() {
                        pending_frags.remove(&frame.seq_id).unwrap().assemble()
                    } else {
                        return Ok(None);
                    }
                };

                self.dispatch_event(payload).await?;
            }
            FRAME_TYPE_PONG => {}
            _ => {
                tracing::debug!("unknown frame type: {}", frame.frame_type);
            }
        }

        Ok(None)
    }

    async fn dispatch_event(&self, payload: Vec<u8>) -> Result<()> {
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
        }

        Ok(())
    }

    async fn get_ws_url(&self) -> Result<String> {
        let token_mgr = TokenManager::new(self.config.token_cache.clone());
        let token = token_mgr
            .get_tenant_access_token(&self.config, None, None)
            .await?;

        let device_id = uuid::Uuid::new_v4().to_string();
        let base = self.config.base_url.trim_end_matches('/');
        let url = format!("{base}/callback/ws/endpoint?DeviceID={device_id}");

        let resp = self
            .config
            .http_client
            .get(&url)?
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {token}")
                    .parse::<http::HeaderValue>()
                    .unwrap(),
            )
            .header_str("DeviceID", &device_id)
            .map_err(|e| Error::Event(format!("invalid device id header: {e}")))?
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

        body.data
            .map(|d| d.url)
            .ok_or_else(|| Error::Event("ws endpoint returned no url".to_string()))
    }
}
