mod common;
use common::{http_response, http_response_with_headers, mock_server};

use std::sync::Arc;

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::cache::{Cache, LocalCache};
use larksuite_oapi_sdk_rs::constants::AccessTokenType;
use larksuite_oapi_sdk_rs::error::Error;
use larksuite_oapi_sdk_rs::req::{ApiReq, FormDataField, FormDataValue, ReqBody, RequestOption};

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
}

// ── Happy path: successful JSON response ──

#[tokio::test]
async fn transport_successful_json_response() {
    let body = r#"{"code":0,"msg":"ok","data":{"id":"123"}}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
    let parsed: serde_json::Value = serde_json::from_slice(&resp.raw_body).unwrap();
    assert_eq!(parsed["data"]["id"], "123");
}

// ── Typed request: deserializes data ──

#[tokio::test]
async fn transport_typed_request_deserializes() {
    #[derive(Debug, serde::Deserialize)]
    struct TestData {
        name: String,
    }

    let body = r#"{"code":0,"msg":"ok","data":{"name":"hello"}}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let (_resp, raw) = client
        .do_req_typed::<TestData>(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(raw.data.unwrap().name, "hello");
}

// ── Typed request: API error code returns Err ──

#[tokio::test]
async fn transport_typed_request_api_error() {
    let body = r#"{"code":10001,"msg":"not found"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let err = client
        .do_req_typed::<serde_json::Value>(&api_req, &RequestOption::default())
        .await
        .unwrap_err();
    assert!(matches!(err, Error::Api(_)));
}

// ── File download: skips JSON parsing ──

#[tokio::test]
async fn transport_file_download_skips_json_parse() {
    let (addr, _h) = mock_server(vec![
        http_response_with_headers(
            200,
            "Content-Disposition: attachment; filename=\"file.bin\"\r\nContent-Type: application/octet-stream\r\n",
            "binary-data",
        ),
    ])
    .await;

    let client = client_for(addr);
    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/download");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let option = RequestOption {
        file_download: true,
        ..Default::default()
    };
    let (_resp, raw) = client
        .do_req_typed::<serde_json::Value>(&api_req, &option)
        .await
        .unwrap();
    assert!(raw.data.is_none());
    assert!(raw.code_error.success());
}

// ── HTTP 504 triggers retry and eventually returns MaxRetries ──

#[tokio::test]
async fn transport_504_retries_then_fails() {
    let (addr, _h) = mock_server(vec![
        "HTTP/1.1 504 Gateway Timeout\r\nContent-Length: 0\r\n\r\n".to_string(),
    ])
    .await;

    let client = Client::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .max_retries(2)
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let err = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap_err();
    assert!(
        matches!(err, Error::ServerTimeout(_) | Error::MaxRetries),
        "expected ServerTimeout or MaxRetries, got {err:?}"
    );
}

// ── HTTP 429 triggers retry ──

#[tokio::test]
async fn transport_429_retries_then_fails() {
    let (addr, _h) = mock_server(vec![
        "HTTP/1.1 429 Too Many Requests\r\nContent-Length: 0\r\n\r\n".to_string(),
    ])
    .await;

    let client = Client::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .max_retries(2)
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let err = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap_err();
    assert!(
        matches!(err, Error::RateLimited(_) | Error::MaxRetries),
        "expected RateLimited or MaxRetries, got {err:?}"
    );
}

#[tokio::test]
async fn transport_retry_succeeds_after_504() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![
        "HTTP/1.1 504 Gateway Timeout\r\nContent-Length: 0\r\n\r\n".to_string(),
        http_response(200, body),
    ])
    .await;

    let client = Client::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .max_retries(3)
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── Token invalid error triggers retry ──

#[tokio::test]
async fn transport_token_invalid_retries() {
    let err_body = r#"{"code":99991671,"msg":"invalid access token"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, err_body)]).await;

    let client = Client::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .max_retries(2)
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let err = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap_err();
    assert!(
        matches!(err, Error::Api(_)),
        "expected Api error, got {err:?}"
    );
}

// ── Custom request ID is sent ──

#[tokio::test]
async fn transport_custom_request_id() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let option = RequestOption {
        request_id: Some("custom-req-id-123".to_string()),
        ..Default::default()
    };
    let resp = client.do_req(&api_req, &option).await.unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── Bearer token passed for user access token ──

#[tokio::test]
async fn transport_bearer_token_user() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = Client::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::User];

    let option = RequestOption {
        user_access_token: Some("u-token-abc".to_string()),
        ..Default::default()
    };
    let resp = client.do_req(&api_req, &option).await.unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── Bearer token passed for app access token ──

#[tokio::test]
async fn transport_bearer_token_app() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = Client::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::App];

    let option = RequestOption {
        app_access_token: Some("app-token-xyz".to_string()),
        ..Default::default()
    };
    let resp = client.do_req(&api_req, &option).await.unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── Tenant access token with cache: pre-populate cache, verify it's used ──

#[tokio::test]
async fn transport_tenant_token_from_cache() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let cache = Arc::new(LocalCache::new());
    cache
        .set(
            "tenant_access_token-test_app-",
            "cached-tenant-token",
            std::time::Duration::from_secs(3600),
        )
        .await
        .unwrap();

    let client = Client::builder("test_app", "secret")
        .base_url(format!("http://{addr}"))
        .token_cache(cache)
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── App access token from cache ──

#[tokio::test]
async fn transport_app_token_from_cache() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let cache = Arc::new(LocalCache::new());
    cache
        .set(
            "app_access_token-test_app",
            "cached-app-token",
            std::time::Duration::from_secs(3600),
        )
        .await
        .unwrap();

    let client = Client::builder("test_app", "secret")
        .base_url(format!("http://{addr}"))
        .token_cache(cache)
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::App];

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── POST with JSON body ──

#[tokio::test]
async fn transport_post_json_body() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let option = RequestOption::default();

    let resp = client
        .post(
            "/open-apis/test",
            serde_json::json!({"key": "value"}),
            &option,
        )
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── PUT with JSON body ──

#[tokio::test]
async fn transport_put_json_body() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .put(
            "/open-apis/test",
            serde_json::json!({"updated": true}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── PATCH with JSON body ──

#[tokio::test]
async fn transport_patch_json_body() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .patch(
            "/open-apis/test",
            serde_json::json!({"patched": true}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── DELETE ──

#[tokio::test]
async fn transport_delete() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .delete("/open-apis/test", &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── Helpdesk auth header ──

#[tokio::test]
async fn transport_helpdesk_auth() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = Client::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .helpdesk_credential("hd_id", "hd_token")
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/helpdesk/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let option = RequestOption {
        need_helpdesk_auth: true,
        ..Default::default()
    };
    let resp = client.do_req(&api_req, &option).await.unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── Default headers are sent ──

#[tokio::test]
async fn transport_default_headers() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let mut headers = http::HeaderMap::new();
    headers.insert("X-Custom-Header", "custom-value".parse().unwrap());

    let client = Client::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .default_headers(headers)
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── Per-request headers ──

#[tokio::test]
async fn transport_per_request_headers() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let mut req_headers = http::HeaderMap::new();
    req_headers.insert("X-Per-Request", "per-value".parse().unwrap());

    let option = RequestOption {
        headers: Some(req_headers),
        ..Default::default()
    };
    let resp = client.do_req(&api_req, &option).await.unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── Non-JSON content type: token error not detected ──

#[tokio::test]
async fn transport_non_json_response_not_parsed_as_error() {
    let (addr, _h) = mock_server(vec![http_response_with_headers(
        200,
        "Content-Type: text/plain\r\n",
        "not json at all",
    )])
    .await;

    let client = client_for(addr);
    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
    assert_eq!(
        std::str::from_utf8(&resp.raw_body).unwrap(),
        "not json at all"
    );
}

// ── Self-built app token fetch ──

#[tokio::test]
async fn transport_self_built_token_fetch() {
    let token_resp = r#"{"code":0,"msg":"ok","app_access_token":"fetched_token","expire":7200}"#;
    let api_resp = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _h) = mock_server(vec![
        http_response(200, token_resp),
        http_response(200, api_resp),
    ])
    .await;

    let client = Client::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::App];

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── Self-built tenant token fetch ──

#[tokio::test]
async fn transport_self_built_tenant_token_fetch() {
    let token_resp = r#"{"code":0,"msg":"ok","tenant_access_token":"tenant_tok","expire":7200}"#;
    let api_resp = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![
        http_response(200, token_resp),
        http_response(200, api_resp),
    ])
    .await;

    let client = Client::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── validate_token_type: user token on tenant-only API ──

#[tokio::test]
async fn transport_rejects_user_token_on_tenant_only_api() {
    let client = Client::builder("app", "secret")
        .disable_token_cache()
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

    let option = RequestOption {
        user_access_token: Some("user-tok".to_string()),
        ..Default::default()
    };
    let err = client.do_req(&api_req, &option).await.unwrap_err();
    assert!(matches!(err, Error::IllegalParam(_)));
}

// ── validate_token_type: app token on user-only API ──

#[tokio::test]
async fn transport_rejects_app_token_on_user_only_api() {
    let client = Client::builder("app", "secret")
        .disable_token_cache()
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::User];

    let option = RequestOption {
        app_access_token: Some("app-tok".to_string()),
        ..Default::default()
    };
    let err = client.do_req(&api_req, &option).await.unwrap_err();
    assert!(matches!(err, Error::IllegalParam(_)));
}

// ── validate_token_type: tenant token on app-only API ──

#[tokio::test]
async fn transport_rejects_tenant_token_on_app_only_api() {
    let client = Client::builder("app", "secret")
        .disable_token_cache()
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::App];

    let option = RequestOption {
        tenant_access_token: Some("tenant-tok".to_string()),
        ..Default::default()
    };
    let err = client.do_req(&api_req, &option).await.unwrap_err();
    assert!(matches!(err, Error::IllegalParam(_)));
}

// ── AccessTokenType::None skips validation ──

#[tokio::test]
async fn transport_none_token_type_skips_validation() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let option = RequestOption {
        user_access_token: Some("u-tok".to_string()),
        app_access_token: Some("a-tok".to_string()),
        ..Default::default()
    };
    let resp = client.do_req(&api_req, &option).await.unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── FormData multipart upload ──

#[tokio::test]
async fn transport_formdata_text_field() {
    let body = r#"{"code":0,"msg":"ok","data":{"file_key":"fk_123"}}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/drive/v1/files/upload_all");
    api_req.supported_access_token_types = vec![AccessTokenType::None];
    api_req.body = Some(ReqBody::FormData(vec![FormDataField {
        name: "file_name".to_string(),
        value: FormDataValue::Text("test.txt".to_string()),
    }]));

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

#[tokio::test]
async fn transport_formdata_file_upload() {
    let body = r#"{"code":0,"msg":"ok","data":{"file_key":"fk_456"}}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/drive/v1/files/upload_all");
    api_req.supported_access_token_types = vec![AccessTokenType::None];
    api_req.body = Some(ReqBody::FormData(vec![
        FormDataField {
            name: "file_name".to_string(),
            value: FormDataValue::Text("report.pdf".to_string()),
        },
        FormDataField {
            name: "file".to_string(),
            value: FormDataValue::File {
                filename: "report.pdf".to_string(),
                data: b"PDF_CONTENT".to_vec(),
                content_type: Some("application/pdf".to_string()),
            },
        },
    ]));

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

#[tokio::test]
async fn transport_formdata_file_no_content_type() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/im/v1/images");
    api_req.supported_access_token_types = vec![AccessTokenType::None];
    api_req.body = Some(ReqBody::FormData(vec![FormDataField {
        name: "image".to_string(),
        value: FormDataValue::File {
            filename: "photo.png".to_string(),
            data: vec![0x89, 0x50, 0x4E, 0x47],
            content_type: None,
        },
    }]));

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── App ticket invalid triggers apply_app_ticket ──

#[tokio::test]
async fn transport_app_ticket_invalid_triggers_retry() {
    let ticket_invalid_resp = r#"{"code":10012,"msg":"app ticket invalid"}"#;
    let success_resp = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _h) = mock_server(vec![
        http_response(200, ticket_invalid_resp),
        http_response(200, success_resp),
    ])
    .await;

    let client = client_for(addr);
    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── determine_token_type with token cache enabled and app token supported ──

#[tokio::test]
async fn transport_cache_enabled_app_token_type() {
    let token_resp = r#"{"code":0,"msg":"ok","app_access_token":"auto_app_tok","expire":7200}"#;
    let api_resp = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _h) = mock_server(vec![
        http_response(200, token_resp),
        http_response(200, api_resp),
    ])
    .await;

    let client = Client::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::App];

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── determine_token_type with user token takes priority over tenant ──

#[tokio::test]
async fn transport_user_token_priority_over_tenant() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = Client::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let option = RequestOption {
        user_access_token: Some("user_tok".to_string()),
        ..Default::default()
    };
    let resp = client.do_req(&api_req, &option).await.unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── empty supported_access_token_types treated as None ──

#[tokio::test]
async fn transport_empty_supported_tokens_treated_as_none() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = Client::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .build();

    let api_req = ApiReq::new(http::Method::GET, "/open-apis/test");

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}

// ── log_level filtering disables debug logging ──

#[tokio::test]
async fn transport_log_level_filters_debug() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _h) = mock_server(vec![http_response(200, body)]).await;

    let client = Client::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .log_level(tracing::Level::ERROR)
        .build();

    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::None];

    let resp = client
        .do_req(&api_req, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(resp.status_code, 200);
}
