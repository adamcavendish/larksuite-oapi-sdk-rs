use super::prelude::*;

// ── Search ──

#[tokio::test]
async fn search_data_record_by_query_smoke() {
    let record_body =
        r#"{"code":0,"msg":"ok","data":{"record":{"id":"item-1","structured_data":"{}"}}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, record_body),
        http_response(200, record_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let body = CreateDataRecordReqBody {
        id: Some("item-1".into()),
        metadata: Some(DataRecordMetadata {
            title: Some("Quarterly report".into()),
            source_url: Some("https://example.test/report".into()),
            ..Default::default()
        }),
        structured_data: Some(r#"{"department":"docs"}"#.into()),
        content: Some(DataRecordContent {
            format: Some("html".into()),
            content_data: Some("<p>report</p>".into()),
        }),
        ..Default::default()
    };

    client
        .search()
        .data_record
        .create_by_query(
            &CreateDataRecordQuery::new("ds-1", &body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .data_record
        .get_by_query(
            &GetDataRecordQuery::new("ds-1", "item-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .data_record
        .delete_by_query(
            &DeleteDataRecordQuery::new("ds-1", "item-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/search/v2/data_sources/ds-1/items "));
    assert!(request.contains("GET /open-apis/search/v2/data_sources/ds-1/items/item-1 "));
    assert!(request.contains("DELETE /open-apis/search/v2/data_sources/ds-1/items/item-1 "));
    assert!(request.contains(r#""id":"item-1""#));
    assert!(request.contains(r#""title":"Quarterly report""#));
    assert!(request.contains(r#""format":"html""#));
}
