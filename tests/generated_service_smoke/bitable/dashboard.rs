use super::prelude::*;

// ── Bitable ──

#[tokio::test]
async fn bitable_dashboard_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"dashboards":[{"block_id":"dash-1","name":"Overview"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListDashboardQuery::new("app-token-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .bitable()
        .dashboard
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.dashboards[0].name.as_deref(), Some("Overview"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/dashboards?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
