use super::prelude::*;

// ── Helpdesk ──

#[tokio::test]
async fn helpdesk_get_faq_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"faq":{"id":"faq-1","answer":"Try restarting"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .get("faq-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert!(resp.data.is_some());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs/faq-1"));
}

#[tokio::test]
async fn helpdesk_faq_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"faq":{"id":"faq-1","answer":"Try restarting"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .get_by_query(
            &GetHelpdeskFaqQuery::new("faq-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.faq.as_ref())
            .and_then(|faq| faq.id.as_deref()),
        Some("faq-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs/faq-1"));
}

#[tokio::test]
async fn helpdesk_faq_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"faq-1","question":"How?"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .list_by_query(
            &ListFaqQuery::new()
                .category_id("cat-1")
                .keyword("restart")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs?"));
    assert!(request.contains("category_id=cat-1"));
    assert!(request.contains("keyword=restart"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn helpdesk_faq_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"faq-1","question":"How?"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .search_by_query(
            &SearchFaqQuery::new()
                .query("restart")
                .base64("cmVzdGFydA==")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs/search?"));
    assert!(request.contains("query=restart"));
    assert!(request.contains("base64=cmVzdGFydA%3D%3D"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
