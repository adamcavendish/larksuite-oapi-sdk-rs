use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::event::{CardActionHandler, EventDispatcher, EventReq, EventResp};

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

    let headers: HashMap<String, Vec<String>> = parts
        .headers
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_string(),
                vec![value.to_str().unwrap_or_default().to_string()],
            )
        })
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).or_default().extend(v);
            acc
        });

    let request_uri = parts
        .uri
        .path_and_query()
        .map(|pq| pq.as_str().to_string())
        .unwrap_or_default();

    Ok(EventReq {
        headers,
        body: body_bytes.to_vec(),
        request_uri,
    })
}

fn build_response(event_resp: EventResp) -> Response {
    let mut builder = axum::http::Response::builder().status(event_resp.status_code);
    for (k, v) in &event_resp.headers {
        builder = builder.header(k.as_str(), v.as_str());
    }
    builder
        .body(axum::body::Body::from(event_resp.body))
        .unwrap_or_else(|_| {
            axum::http::Response::builder()
                .status(500)
                .body(axum::body::Body::empty())
                .unwrap()
        })
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
        let body: serde_json::Value = serde_json::from_slice(&response_body(resp).await).unwrap();
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
        let body: serde_json::Value = serde_json::from_slice(&response_body(resp).await).unwrap();
        assert_eq!(body["ok"], true);
    }
}
