mod common;

use std::collections::BTreeMap;

use common::{http_response, http_response_with_headers, mock_server_with_requests};
use larksuite_oapi_sdk_rs::service::spark::v1::PageSourceListQuery;
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};
use serde_json::json;

fn client(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .max_retries(2)
        .build()
        .unwrap()
}

fn user_option() -> RequestOption {
    RequestOption {
        user_access_token: Some("source-user".into()),
        ..Default::default()
    }
}

#[tokio::test]
async fn list_preserves_pagination_size_and_query_values() {
    let body = json!({
        "code": 0,
        "data": {
            "items": [{
                "name": "main.ts",
                "path": "src/main.ts",
                "type": "file",
                "size": 9007199254740993_i64
            }],
            "next_page_token": "next+/=",
            "has_more": true,
            "future": "preserved"
        }
    });
    let (addr, handle, requests) =
        mock_server_with_requests(vec![http_response(200, &body.to_string())]).await;
    let response = client(addr)
        .spark()
        .app
        .list_page_source(
            &PageSourceListQuery::new(" share-token ")
                .path("src/ 目录 ")
                .page_size(100)
                .page_token(" cursor+/= "),
            &user_option(),
        )
        .await
        .unwrap();

    assert!(response.success());
    let data = response.data.as_ref().unwrap();
    assert_eq!(data.items[0].size, 9007199254740993_i64);
    assert_eq!(data.items[0].r#type, "file");
    assert_eq!(data.next_page_token.as_deref(), Some("next+/="));
    assert!(data.has_more);

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let target = requests[0]
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap();
    let url = url::Url::parse(&format!("http://localhost{target}")).unwrap();
    assert_eq!(url.path(), "/open-apis/spark/v1/apps/source/files");
    assert_eq!(
        url.query_pairs().into_owned().collect::<BTreeMap<_, _>>(),
        BTreeMap::from([
            ("meta_token".into(), "share-token".into()),
            ("path".into(), "src/ 目录 ".into()),
            ("page_size".into(), "100".into()),
            ("page_token".into(), " cursor+/= ".into()),
        ])
    );
    assert!(
        requests[0]
            .to_ascii_lowercase()
            .contains("authorization: bearer source-user\r\n")
    );
    handle.abort();
}

#[tokio::test]
async fn list_validates_user_token_and_page_size_without_io() {
    let (addr, handle, requests) = mock_server_with_requests(vec![http_response(200, "{}")]).await;
    let client = client(addr);
    for token in [None, Some(""), Some(" ")] {
        let option = RequestOption {
            user_access_token: token.map(str::to_string),
            tenant_access_token: Some("wrong-token".into()),
            ..Default::default()
        };
        assert!(matches!(
            client
                .spark()
                .app
                .list_page_source(&PageSourceListQuery::new("share-token"), &option)
                .await,
            Err(LarkError::IllegalParam(_))
        ));
    }
    for page_size in [0, 101] {
        assert!(matches!(
            client
                .spark()
                .app
                .list_page_source(
                    &PageSourceListQuery::new("share-token").page_size(page_size),
                    &user_option(),
                )
                .await,
            Err(LarkError::IllegalParam(_))
        ));
    }
    assert!(requests.lock().unwrap().is_empty());
    handle.abort();
}

#[tokio::test]
async fn list_preserves_platform_errors() {
    let (addr, handle, requests) = mock_server_with_requests(vec![http_response(
        403,
        r#"{"code":99991672,"msg":"permission denied"}"#,
    )])
    .await;
    let error = client(addr)
        .spark()
        .app
        .list_page_source(&PageSourceListQuery::new("share-token"), &user_option())
        .await
        .unwrap_err();
    assert!(matches!(error, LarkError::Api(error) if error.code == 99991672));
    assert_eq!(requests.lock().unwrap().len(), 1);
    handle.abort();
}

#[tokio::test]
async fn get_streams_raw_success_body_and_encodes_query() {
    let content = r#"{"code":40300,"msg":"source text"}"#;
    let (addr, handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Type: text/plain; charset=utf-8\r\n",
        content,
    )])
    .await;
    let mut response = client(addr)
        .spark()
        .app
        .get_page_source("share-token", "src/ 中文.ts ", &user_option())
        .await
        .unwrap();
    let mut bytes = Vec::new();
    while let Some(chunk) = response.body.next_chunk().await.unwrap() {
        bytes.extend_from_slice(&chunk);
    }
    assert_eq!(bytes, content.as_bytes());

    let requests = requests.lock().unwrap();
    let target = requests[0]
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap();
    let url = url::Url::parse(&format!("http://localhost{target}")).unwrap();
    assert_eq!(url.path(), "/open-apis/spark/v1/apps/source/file");
    assert_eq!(
        url.query_pairs().into_owned().collect::<BTreeMap<_, _>>(),
        BTreeMap::from([
            ("meta_token".into(), "share-token".into()),
            ("path".into(), "src/ 中文.ts ".into()),
        ])
    );
    handle.abort();
}

#[tokio::test]
async fn get_rejects_invalid_inputs_without_io() {
    let (addr, handle, requests) =
        mock_server_with_requests(vec![http_response(200, "ignored")]).await;
    let client = client(addr);
    for (meta_token, path) in [
        ("", "src/main.ts"),
        ("https://example/page/token", "src/main.ts"),
        ("share-token", ""),
        ("share-token", "../secret"),
        ("share-token", "/etc/passwd"),
        ("share-token", "C:/secret"),
    ] {
        assert!(matches!(
            client
                .spark()
                .app
                .get_page_source(meta_token, path, &user_option())
                .await,
            Err(LarkError::IllegalParam(_))
        ));
    }
    assert!(requests.lock().unwrap().is_empty());
    handle.abort();
}
