use super::prelude::*;

// ── Hire ──

#[tokio::test]
async fn hire_talent_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"talent":{"id":"talent-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetHireTalentQuery::new("talent-1").user_id_type("open_id");
    let resp = client
        .hire()
        .talent
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().talent.unwrap().id.as_deref(),
        Some("talent-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/talents/talent-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_talent_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"talent-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireTalentQuery::new()
        .keyword("product")
        .update_start_time("1618500278663")
        .update_end_time("1618500279999")
        .sort_by(1)
        .query_option("ignore_empty_error")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .talent
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("talent-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/talents?"));
    assert!(request.contains("keyword=product"));
    assert!(request.contains("update_start_time=1618500278663"));
    assert!(request.contains("update_end_time=1618500279999"));
    assert!(request.contains("sort_by=1"));
    assert!(request.contains("query_option=ignore_empty_error"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
