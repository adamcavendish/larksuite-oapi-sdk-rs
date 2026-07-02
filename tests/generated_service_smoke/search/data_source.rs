use super::prelude::*;

// ── Search ──

#[tokio::test]
async fn search_data_source_by_query_smoke() {
    let data_source_body = r#"{"code":0,"msg":"ok","data":{"data_source":{"id":"ds-1","name":"Docs","state":1,"description":"Knowledge"}}}"#;
    let list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"ds-1","name":"Docs"}],"has_more":false}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, data_source_body),
        http_response(200, data_source_body),
        http_response(200, data_source_body),
        http_response(200, list_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateDataSourceReqBody {
        name: Some("Docs".into()),
        state: Some(1),
        description: Some("Knowledge".into()),
        searchable_fields: Some(vec!["title".into(), "body".into()]),
        schema_id: Some("schema-1".into()),
        app_id: Some("app-1".into()),
        ..Default::default()
    };
    let patch_body = PatchDataSourceReqBody {
        name: Some("Docs updated".into()),
        state: Some(2),
        description: Some("Knowledge updated".into()),
        ..Default::default()
    };

    client
        .search()
        .data_source
        .create_by_query(
            &CreateDataSourceQuery::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .data_source
        .get_by_query(&GetDataSourceQuery::new("ds-1"), &RequestOption::default())
        .await
        .unwrap();
    client
        .search()
        .data_source
        .patch_by_query(
            &PatchDataSourceQuery::new("ds-1", &patch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .data_source
        .list_by_query(
            &ListDataSourceQuery::new()
                .view("basic")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .data_source
        .delete_by_query(
            &DeleteDataSourceQuery::new("ds-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/search/v2/data_sources "));
    assert!(request.contains("GET /open-apis/search/v2/data_sources/ds-1 "));
    assert!(request.contains("PATCH /open-apis/search/v2/data_sources/ds-1 "));
    assert!(request.contains("GET /open-apis/search/v2/data_sources?"));
    assert!(request.contains("DELETE /open-apis/search/v2/data_sources/ds-1 "));
    assert!(request.contains("view=basic"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""name":"Docs""#));
    assert!(request.contains(r#""name":"Docs updated""#));
    assert!(request.contains(r#""schema_id":"schema-1""#));
}
