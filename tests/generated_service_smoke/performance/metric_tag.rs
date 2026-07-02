use super::prelude::*;

// ── Performance ──

#[tokio::test]
async fn performance_metric_tag_v2_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"tag_id":"tag-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .performance_v2()
        .metric_tag
        .list(Some(20), Some("next-page"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/performance/v2/metric_tags?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn performance_metric_tag_v2_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"tag_id":"tag-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let tag_ids = ["tag-1", "tag-2"];
    let query = ListPerformanceMetricTagV2Query::new()
        .tag_ids(tag_ids.as_slice())
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .performance_v2()
        .metric_tag
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0]["tag_id"].as_str(), Some("tag-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/performance/v2/metric_tags?"));
    assert!(request.contains("tag_ids=tag-1"));
    assert!(request.contains("tag_ids=tag-2"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
