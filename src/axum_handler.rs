use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::event::{CardActionHandler, EventDispatcher, EventReq};

// ── EventDispatcher axum handler ──

pub async fn event_handler(
    State(dispatcher): State<Arc<EventDispatcher>>,
    req: Request,
) -> Response {
    let (parts, body) = req.into_parts();

    let body_bytes: Bytes = match axum::body::to_bytes(body, 10 * 1024 * 1024).await {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("failed to read request body: {e}"),
            )
                .into_response();
        }
    };

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

    let event_req = EventReq {
        headers,
        body: body_bytes.to_vec(),
        request_uri,
    };

    let event_resp = dispatcher.handle(event_req).await;

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

// ── CardActionHandler axum handler ──

pub async fn card_action_handler(
    State(handler): State<Arc<CardActionHandler>>,
    req: Request,
) -> Response {
    let (parts, body) = req.into_parts();

    let body_bytes: Bytes = match axum::body::to_bytes(body, 10 * 1024 * 1024).await {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("failed to read request body: {e}"),
            )
                .into_response();
        }
    };

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

    let event_req = EventReq {
        headers,
        body: body_bytes.to_vec(),
        request_uri,
    };

    let event_resp = handler.handle(event_req).await;

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
