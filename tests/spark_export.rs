mod common;

use common::{http_response_with_headers, mock_server_with_gated_body, mock_server_with_requests};
use larksuite_oapi_sdk_rs::service::spark::v1::{ExportAppError, ExportAppLocator};
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};
use std::time::Duration;

fn client(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .max_retries(1)
        .build()
        .unwrap()
}

fn user() -> RequestOption {
    RequestOption {
        user_access_token: Some("export-user".into()),
        ..Default::default()
    }
}

#[tokio::test]
async fn export_locators_use_exact_json_route_and_user_auth() {
    for (locator, expected, content_type) in [
        (
            ExportAppLocator::AppId("app/1"),
            serde_json::json!({"app_id":"app/1"}),
            "application/zip",
        ),
        (
            ExportAppLocator::MetaToken("meta/1"),
            serde_json::json!({"meta_token":"meta/1"}),
            "Application/Octet-Stream; charset=binary",
        ),
    ] {
        let headers = format!(
            "Content-Type: {content_type}\r\nContent-Disposition: attachment; filename=\"source.zip\"\r\n"
        );
        let (addr, handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
            200,
            &headers,
            "PK-archive",
        )])
        .await;
        let mut response = client(addr)
            .spark()
            .app
            .export(locator, &user())
            .await
            .unwrap();
        assert_eq!(response.file_name.as_deref(), Some("source.zip"));
        assert_eq!(response.content_length, Some(10));
        assert!(response.api_resp.raw_body.is_empty());
        let mut bytes = Vec::new();
        while let Some(chunk) = response.body.next_chunk().await.unwrap() {
            bytes.extend_from_slice(&chunk);
        }
        assert_eq!(bytes, b"PK-archive");
        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        let (headers, body) = requests[0].split_once("\r\n\r\n").unwrap();
        assert_eq!(
            headers.lines().next().unwrap(),
            "POST /open-apis/spark/v1/apps/export HTTP/1.1"
        );
        assert!(
            headers
                .to_ascii_lowercase()
                .contains("authorization: bearer export-user")
        );
        assert!(
            headers
                .to_ascii_lowercase()
                .contains("content-type: application/json")
        );
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(body).unwrap(),
            expected
        );
        handle.abort();
    }
}

#[tokio::test]
async fn export_rejects_empty_locators_and_non_user_credentials_before_sending() {
    let (addr, handle, requests) =
        mock_server_with_requests(vec![common::http_response(200, r#"{"code":0}"#)]).await;
    for locator in [
        ExportAppLocator::AppId(""),
        ExportAppLocator::MetaToken("  "),
    ] {
        assert!(matches!(
            client(addr).spark().app.export(locator, &user()).await,
            Err(ExportAppError::Request(LarkError::IllegalParam(_)))
        ));
    }
    for option in [
        RequestOption::default(),
        RequestOption {
            tenant_access_token: Some("tenant".into()),
            ..Default::default()
        },
        RequestOption {
            user_access_token: Some("".into()),
            ..Default::default()
        },
    ] {
        assert!(
            client(addr)
                .spark()
                .app
                .export(ExportAppLocator::AppId("app"), &option)
                .await
                .is_err()
        );
    }
    assert!(requests.lock().unwrap().is_empty());
    handle.abort();
}

#[tokio::test]
async fn export_rejects_error_envelopes_and_preserves_status_and_metadata() {
    for (status, content_type, body, code) in [
        (
            200,
            "application/json",
            r#"{"code":40901,"msg":"not published"}"#,
            Some(40901),
        ),
        (
            422,
            "application/json",
            r#"{"code":40901,"msg":"not published"}"#,
            Some(40901),
        ),
        (200, "text/plain", "permission denied", None),
        (200, "", r#"{"code":999,"msg":"error"}"#, Some(999)),
        (200, "application/json", r#"{"code":0,"msg":"ok"}"#, None),
        (200, "application/json", "{broken", None),
        (403, "application/zip", "denied", None),
        (404, "text/plain", "app not found", None),
        (413, "text/plain", "too large", None),
    ] {
        let mut headers = "X-Request-Id: export-request\r\n".to_string();
        if !content_type.is_empty() {
            headers.push_str(&format!("Content-Type: {content_type}\r\n"));
        }
        let (addr, handle, _) =
            mock_server_with_requests(vec![http_response_with_headers(status, &headers, body)])
                .await;
        let error = client(addr)
            .spark()
            .app
            .export(ExportAppLocator::AppId("app"), &user())
            .await
            .unwrap_err();
        let ExportAppError::InvalidResponse {
            api_resp,
            code_error,
        } = error
        else {
            panic!("unexpected error: {error:?}");
        };
        assert_eq!(api_resp.status_code, status);
        assert_eq!(api_resp.header["x-request-id"], "export-request");
        assert_eq!(api_resp.raw_body, body.as_bytes());
        assert_eq!(code_error.map(|e| e.code), code);
        handle.abort();
    }
}

#[tokio::test]
async fn export_returns_archive_before_body_finishes() {
    let (addr, handle, _, release) = mock_server_with_gated_body(
        200,
        "Content-Type: application/zip\r\n",
        "PK-first",
        vec!["-last"],
    )
    .await;
    let mut response = tokio::time::timeout(
        Duration::from_secs(2),
        client(addr)
            .spark()
            .app
            .export(ExportAppLocator::AppId("app"), &user()),
    )
    .await
    .expect("export must not buffer archive")
    .unwrap();
    let first = tokio::time::timeout(Duration::from_secs(2), response.body.next_chunk())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(first, "PK-first");
    release.send(()).unwrap();
    let mut rest = Vec::new();
    while let Some(chunk) = response.body.next_chunk().await.unwrap() {
        rest.extend_from_slice(&chunk);
    }
    assert_eq!(rest, b"-last");
    handle.await.unwrap();
}

#[tokio::test]
async fn export_error_reads_stop_at_limit_even_when_server_keeps_body_open() {
    static ERROR_BYTES: [u8; 4096] = [b'x'; 4096];
    let prefix = std::str::from_utf8(&ERROR_BYTES).unwrap();
    for (status, content_type) in [(200, "text/plain"), (403, "application/json")] {
        let headers = format!("Content-Type: {content_type}\r\n");
        let (addr, handle, _, release) =
            mock_server_with_gated_body(status, &headers, prefix, vec!["never-read"]).await;
        let error = tokio::time::timeout(
            Duration::from_secs(2),
            client(addr)
                .spark()
                .app
                .export(ExportAppLocator::AppId("app"), &user()),
        )
        .await
        .expect("error handling must stop at 4 KiB")
        .unwrap_err();
        let ExportAppError::InvalidResponse {
            api_resp,
            code_error,
        } = error
        else {
            panic!("unexpected error: {error:?}");
        };
        assert_eq!(api_resp.raw_body.len(), 4096);
        assert!(code_error.is_none());
        drop(release);
        handle.await.unwrap();
    }
}
