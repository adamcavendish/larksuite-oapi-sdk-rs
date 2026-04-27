mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
}

#[tokio::test]
async fn corehr_offboarding_iterator_requests_resume_token() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"offboarding_id":"o1"}],"page_token":"token-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"offboarding_id":"o2"}],"page_token":"token-2","has_more":false}}"#;
    let (addr, _h, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let mut iter = client
        .corehr()
        .offboarding
        .search_by_iterator(
            serde_json::json!({"employment_ids":["e1"]}),
            Some(100),
            None,
        )
        .page_token("seed-token");
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    };

    let _ = iter.next(&option).await.unwrap();
    let _ = iter.next(&option).await.unwrap();

    let reqs = requests.lock().unwrap();
    assert!(reqs[0].contains("POST /open-apis/corehr/v1/offboardings/search?"));
    assert!(reqs[0].contains("page_token=seed-token"));
    assert!(reqs[0].contains("\"employment_ids\":[\"e1\"]"));
    assert!(reqs[1].contains("page_token=token-1"));
}

#[tokio::test]
async fn corehr_offboarding_iterator_next_page_token_uses_server_cursor_after_resume() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"offboarding_id":"o1"}],"page_token":"token-1","has_more":true}}"#;
    let (addr, _h, _requests) = mock_server_with_requests(vec![http_response(200, page1)]).await;

    let client = client_for(addr);
    let mut iter = client
        .corehr()
        .offboarding
        .search_by_iterator(
            serde_json::json!({"employment_ids":["e1"]}),
            Some(100),
            None,
        )
        .page_token("seed-token");
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    };

    let _ = iter.next(&option).await.unwrap();
    assert_eq!(iter.next_page_token(), Some("token-1"));
}

#[tokio::test]
async fn corehr_offboarding_iterator_preserves_token_on_empty_page() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"offboarding_id":"o1"}],"page_token":"token-1","has_more":true}}"#;
    let page2 =
        r#"{"code":0,"msg":"ok","data":{"items":[],"page_token":"token-2","has_more":true}}"#;
    let (addr, _h, _requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let mut iter = client.corehr().offboarding.search_by_iterator(
        serde_json::json!({"employment_ids":["e1"]}),
        Some(100),
        None,
    );
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    };

    let first = iter.next(&option).await.unwrap().unwrap();
    let second = iter.next(&option).await.unwrap();

    assert_eq!(first.offboarding_id.as_deref(), Some("o1"));
    assert!(second.is_none());
    assert_eq!(iter.next_page_token(), Some("token-1"));
}
