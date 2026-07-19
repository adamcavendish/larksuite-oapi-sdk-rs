use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::event::{CardActionHandler, EventDispatcher, EventReq};
use crate::http_handler::{event_req_from_parts, event_resp_into_http};

async fn parse_request(req: Request) -> std::result::Result<EventReq, Response> {
    let (parts, body) = req.into_parts();

    let body_bytes: Bytes = axum::body::to_bytes(body, 10 * 1024 * 1024)
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("failed to read request body: {e}"),
            )
                .into_response()
        })?;

    Ok(event_req_from_parts(parts, body_bytes.to_vec()))
}

fn build_response(event_resp: crate::event::EventResp) -> Response {
    event_resp_into_http(event_resp).map(axum::body::Body::from)
}

pub async fn event_handler(
    State(dispatcher): State<Arc<EventDispatcher>>,
    req: Request,
) -> Response {
    let event_req = match parse_request(req).await {
        Ok(r) => r,
        Err(resp) => return resp,
    };
    build_response(dispatcher.handle(event_req).await)
}

pub async fn card_action_handler(
    State(handler): State<Arc<CardActionHandler>>,
    req: Request,
) -> Response {
    let event_req = match parse_request(req).await {
        Ok(r) => r,
        Err(resp) => return resp,
    };
    build_response(handler.handle(event_req).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::event::EventResp;
    use axum::body::{Body, Bytes};

    async fn response_body(resp: Response) -> Bytes {
        axum::body::to_bytes(resp.into_body(), 1024 * 1024)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn event_handler_url_verification() {
        let dispatcher = Arc::new(EventDispatcher::new("token", ""));
        let body = serde_json::json!({
            "type": "url_verification",
            "token": "token",
            "challenge": "challenge-1"
        });
        let req = Request::builder()
            .method("POST")
            .uri("/webhook/event")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&body).unwrap()))
            .unwrap();

        let resp = event_handler(State(dispatcher), req).await;

        assert_eq!(resp.status(), 200);
        let body: crate::JsonValue = serde_json::from_slice(&response_body(resp).await).unwrap();
        assert_eq!(body["challenge"], "challenge-1");
    }

    #[tokio::test]
    async fn event_handler_missing_signature_returns_500() {
        let dispatcher = Arc::new(EventDispatcher::new("", "encrypt-key"));
        // P2 protocol event with no signature headers — must fail verification
        let body = serde_json::json!({
            "schema": "2.0",
            "header": {
                "event_id": "evt-1",
                "event_type": "im.message.receive_v1",
                "app_id": "cli_test",
                "tenant_key": "t1",
                "create_time": "0"
            },
            "event": {}
        });
        let req = Request::builder()
            .method("POST")
            .uri("/webhook/event")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&body).unwrap()))
            .unwrap();

        let resp = event_handler(State(dispatcher), req).await;

        assert_eq!(resp.status(), 500);
    }

    #[tokio::test]
    async fn card_action_handler_returns_json() {
        let handler = Arc::new(
            CardActionHandler::new("", "", |_action| async {
                Ok(serde_json::json!({"ok": true}))
            })
            .skip_sign_verify(),
        );

        let req = Request::builder()
            .method("POST")
            .uri("/webhook/card")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&serde_json::json!({"action": {}})).unwrap(),
            ))
            .unwrap();

        let resp = card_action_handler(State(handler), req).await;

        assert_eq!(resp.status(), 200);
        let body: crate::JsonValue = serde_json::from_slice(&response_body(resp).await).unwrap();
        assert_eq!(body["ok"], true);
    }

    #[tokio::test]
    async fn parse_request_uses_shared_http_conversion() {
        let mut req = Request::builder()
            .method("POST")
            .uri("/webhook/event?tenant=t1")
            .body(Body::from(br#"{"hello":"world"}"#.to_vec()))
            .unwrap();
        req.headers_mut()
            .append("x-custom", "first".parse().unwrap());
        req.headers_mut()
            .append("x-custom", "second".parse().unwrap());

        let event_req = parse_request(req).await.unwrap();

        assert_eq!(event_req.request_uri, "/webhook/event?tenant=t1");
        assert_eq!(event_req.headers["x-custom"], ["first", "second"]);
        assert_eq!(event_req.body, br#"{"hello":"world"}"#);
    }

    #[tokio::test]
    async fn build_response_uses_shared_http_materialization() {
        let response = build_response(EventResp {
            status_code: 202,
            headers: std::collections::HashMap::from([("X-Test".to_string(), "ok".to_string())]),
            body: b"accepted".to_vec(),
        });

        assert_eq!(response.status(), 202);
        assert_eq!(response.headers()["x-test"], "ok");
        assert_eq!(response_body(response).await.as_ref(), b"accepted");
    }

    #[tokio::test]
    async fn build_response_preserves_shared_invalid_header_fallback() {
        let response = build_response(EventResp {
            status_code: 200,
            headers: std::collections::HashMap::from([(
                "X-Test".to_string(),
                "invalid\nvalue".to_string(),
            )]),
            body: b"accepted".to_vec(),
        });

        assert_eq!(response.status(), 500);
        assert!(response.headers().is_empty());
        assert!(response_body(response).await.is_empty());
    }
}
