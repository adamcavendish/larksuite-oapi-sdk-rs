use super::prelude::*;

// ── Workplace ──

#[tokio::test]
async fn workplace_access_data_by_query_smoke() {
    let workplace_body = r#"{"code":0,"msg":"ok","data":{"items":[{"date":"2026-01-01","all_workplace_access_count":10}],"has_more":false}}"#;
    let block_body = r#"{"code":0,"msg":"ok","data":{"items":[{"date":"2026-01-01","block_id":"block-1","access_count":5}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, workplace_body),
        http_response(200, workplace_body),
        http_response(200, block_body),
        http_response(200, block_body),
        http_response(200, block_body),
    ])
    .await;

    let client = client_for(addr);
    let workplace_query = WorkplaceAccessDataQuery::new("2026-01-01", "2026-01-31")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let custom_query = CustomWorkplaceAccessDataQuery::new("2026-01-01", "2026-01-31")
        .workplace_id("workplace-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let block_query = WorkplaceBlockAccessDataQuery::new("2026-01-01", "2026-01-31")
        .block_id("block-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));

    client
        .workplace()
        .workplace_access_data
        .search_by_query(&workplace_query, &RequestOption::default())
        .await
        .unwrap();
    client
        .workplace()
        .workplace_access_data
        .get_by_query(&workplace_query, &RequestOption::default())
        .await
        .unwrap();
    client
        .workplace()
        .custom_workplace_access_data
        .search_by_query(&custom_query, &RequestOption::default())
        .await
        .unwrap();
    client
        .workplace()
        .custom_workplace_access_data
        .get_by_query(&custom_query, &RequestOption::default())
        .await
        .unwrap();
    client
        .workplace()
        .workplace_block_access_data
        .search_by_query(&block_query, &RequestOption::default())
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/workplace/v1/workplace_access_data/search?"));
    assert!(request.contains("GET /open-apis/workplace/v1/workplace_access_data/search?"));
    assert!(request.contains("POST /open-apis/workplace/v1/custom_workplace_access_data/search?"));
    assert!(request.contains("GET /open-apis/workplace/v1/custom_workplace_access_data/search?"));
    assert!(request.contains("POST /open-apis/workplace/v1/workplace_block_access_data/search?"));
    assert!(request.contains("from_date=2026-01-01"));
    assert!(request.contains("to_date=2026-01-31"));
    assert!(request.contains("workplace_id=workplace-1"));
    assert!(request.contains("block_id=block-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
