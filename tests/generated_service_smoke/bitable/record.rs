use super::prelude::*;

// ── Bitable ──

#[tokio::test]
async fn bitable_record_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"record_id":"rec-1","fields":{"Name":"Task"}}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListRecordQuery::new("app-token-1", "tbl-1")
        .view_id("view-1")
        .filter("filter-1")
        .sort("sort-1")
        .field_names("Name")
        .text_field_as_array(true)
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .bitable()
        .record
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].record_id.as_deref(), Some("rec-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/tables/tbl-1/records?"));
    assert!(request.contains("view_id=view-1"));
    assert!(request.contains("filter=filter-1"));
    assert!(request.contains("sort=sort-1"));
    assert!(request.contains("field_names=Name"));
    assert!(request.contains("text_field_as_array=true"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn bitable_record_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"record_id":"rec-1","fields":{"Name":"Task"}}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .bitable()
        .record
        .list(
            "app-token-1",
            "tbl-1",
            Some("view-1"),
            Some("filter-1"),
            Some("sort-1"),
            Some("Name"),
            Some(true),
            Some("open_id"),
            Some("next-page"),
            Some(20),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/tables/tbl-1/records?"));
    assert!(request.contains("view_id=view-1"));
    assert!(request.contains("filter=filter-1"));
    assert!(request.contains("sort=sort-1"));
    assert!(request.contains("field_names=Name"));
    assert!(request.contains("text_field_as_array=true"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn bitable_record_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"record_id":"rec-1","fields":{"Name":"Task"}}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SearchRecordQuery::new("app-token-1", "tbl-1")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let body = SearchRecordReqBody {
        automatic_fields: Some(true),
        ..Default::default()
    };
    let resp = client
        .bitable()
        .record
        .search_by_query(&query, &body, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].record_id.as_deref(), Some("rec-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request
            .contains("POST /open-apis/bitable/v1/apps/app-token-1/tables/tbl-1/records/search?")
    );
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""automatic_fields":true"#));
}
