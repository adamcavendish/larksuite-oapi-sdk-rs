use super::prelude::*;

// ── Performance ──

#[tokio::test]
async fn performance_activity_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"activity_id":"act-1","name":"Review"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .performance()
        .activity
        .list(Some(20), Some("next-page"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/performance/v1/activities?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn performance_activity_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"activity_id":"act-1","name":"Review"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListPerformanceActivityQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .performance()
        .activity
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].activity_id.as_deref(), Some("act-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/performance/v1/activities?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
