//! Framework-agnostic HTTP adapter helpers for event callbacks.
//!
//! This module mirrors the Go SDK's `core/httpserverext` package at the SDK
//! boundary: convert an HTTP request into [`EventReq`], run an event or card
//! handler, and convert the SDK response back into an HTTP response. Framework
//! integrations can collect their request body bytes and use these helpers
//! without depending on Axum.

use std::collections::HashMap;

use http::{Request, Response};

use crate::event::{CardActionHandler, EventDispatcher, EventReq, EventResp};

/// Convert an `http` request with an already-collected byte body into an SDK event request.
#[must_use]
pub fn event_req_from_http(req: Request<Vec<u8>>) -> EventReq {
    let (parts, body) = req.into_parts();
    event_req_from_parts(parts, body)
}

/// Convert HTTP request parts and body bytes into an SDK event request.
#[must_use]
pub fn event_req_from_parts(parts: http::request::Parts, body: impl Into<Vec<u8>>) -> EventReq {
    let headers = parts
        .headers
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_string(),
                vec![value.to_str().unwrap_or_default().to_string()],
            )
        })
        .fold(HashMap::<String, Vec<String>>::new(), |mut acc, (k, v)| {
            acc.entry(k).or_default().extend(v);
            acc
        });

    let request_uri = parts
        .uri
        .path_and_query()
        .map(|pq| pq.as_str().to_string())
        .unwrap_or_default();

    EventReq {
        headers,
        body: body.into(),
        request_uri,
    }
}

/// Convert an SDK event response into a generic `http` response.
#[must_use]
pub fn event_resp_into_http(resp: EventResp) -> Response<Vec<u8>> {
    let mut builder = Response::builder().status(resp.status_code);
    for (name, value) in &resp.headers {
        builder = builder.header(name.as_str(), value.as_str());
    }
    builder
        .body(resp.body)
        .unwrap_or_else(|_| Response::builder().status(500).body(Vec::new()).unwrap())
}

/// Handle a generic HTTP request with an [`EventDispatcher`].
pub async fn event_handler(
    dispatcher: &EventDispatcher,
    req: Request<Vec<u8>>,
) -> Response<Vec<u8>> {
    let event_req = event_req_from_http(req);
    event_resp_into_http(dispatcher.handle(event_req).await)
}

/// Handle a generic HTTP request with a [`CardActionHandler`].
pub async fn card_action_handler(
    handler: &CardActionHandler,
    req: Request<Vec<u8>>,
) -> Response<Vec<u8>> {
    let event_req = event_req_from_http(req);
    event_resp_into_http(handler.handle(event_req).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::event::CardActionHandler;
    use http::header::HeaderName;

    #[test]
    fn event_req_from_http_preserves_request_uri_headers_and_body() {
        let req = Request::builder()
            .method("POST")
            .uri("/webhook/event?tenant=t1")
            .header("x-request-id", "req-1")
            .body(br#"{"hello":"world"}"#.to_vec())
            .unwrap();

        let event_req = event_req_from_http(req);

        assert_eq!(event_req.request_uri, "/webhook/event?tenant=t1");
        assert_eq!(event_req.request_id(), "req-1");
        assert_eq!(event_req.body, br#"{"hello":"world"}"#);
    }

    #[test]
    fn event_resp_into_http_preserves_status_headers_and_body() {
        let resp = EventResp {
            status_code: 202,
            headers: HashMap::from([("X-Test".to_string(), "ok".to_string())]),
            body: b"accepted".to_vec(),
        };

        let http_resp = event_resp_into_http(resp);

        assert_eq!(http_resp.status(), 202);
        assert_eq!(http_resp.headers()["x-test"], "ok");
        assert_eq!(http_resp.body(), b"accepted");
    }

    #[tokio::test]
    async fn event_handler_runs_dispatcher() {
        let dispatcher = EventDispatcher::new("token", "");
        let body = serde_json::json!({
            "type": "url_verification",
            "token": "token",
            "challenge": "challenge-1"
        });
        let req = Request::builder()
            .method("POST")
            .uri("/webhook/event")
            .body(serde_json::to_vec(&body).unwrap())
            .unwrap();

        let resp = event_handler(&dispatcher, req).await;
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();

        assert_eq!(resp.status(), 200);
        assert_eq!(body["challenge"], "challenge-1");
    }

    #[tokio::test]
    async fn card_action_handler_runs_handler() {
        let handler = CardActionHandler::new("", "", |_action| async {
            Ok(serde_json::json!({
                "ok": true
            }))
        })
        .skip_sign_verify();

        let req = Request::builder()
            .method("POST")
            .uri("/webhook/card")
            .body(
                serde_json::to_vec(&serde_json::json!({
                    "action": {}
                }))
                .unwrap(),
            )
            .unwrap();

        let resp = card_action_handler(&handler, req).await;
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();

        assert_eq!(resp.status(), 200);
        assert_eq!(body["ok"], true);
    }

    #[tokio::test]
    async fn lowercase_signature_headers_are_accepted() {
        let encrypt_key = "test-sign-key";
        let timestamp = "1234567890";
        let nonce = "nonce";
        let dispatcher = EventDispatcher::new("", encrypt_key);
        let body = serde_json::json!({
            "schema": "2.0",
            "header": {
                "event_id": "test-id",
                "event_type": "unknown.event",
                "app_id": "cli_test",
                "tenant_key": "t1",
                "create_time": "0"
            },
            "event": {}
        });
        let body = serde_json::to_vec(&body).unwrap();
        let signature = {
            use sha2::{Digest as _, Sha256};

            let mut hasher = Sha256::new();
            hasher.update(timestamp.as_bytes());
            hasher.update(nonce.as_bytes());
            hasher.update(encrypt_key.as_bytes());
            hasher.update(&body);
            hex::encode(hasher.finalize())
        };
        let req = Request::builder()
            .method("POST")
            .uri("/webhook/event")
            .header(
                HeaderName::from_static("x-lark-request-timestamp"),
                timestamp,
            )
            .header(HeaderName::from_static("x-lark-request-nonce"), nonce)
            .header(HeaderName::from_static("x-lark-signature"), signature)
            .body(body)
            .unwrap();

        let resp = event_handler(&dispatcher, req).await;

        assert_eq!(resp.status(), 200);
    }
}
