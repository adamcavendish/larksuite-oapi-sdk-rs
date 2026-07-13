use super::prelude::*;

// ── Lingo ──

#[tokio::test]
async fn lingo_classification_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"class-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .lingo()
        .classification
        .list_by_query(
            &ListLingoClassificationQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.as_ref().map(|data| data.items.len()), Some(1));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/lingo/v1/classifications?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
