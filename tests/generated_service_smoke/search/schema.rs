use super::prelude::*;

// ── Search ──

#[tokio::test]
async fn search_data_source_schema_by_query_smoke() {
    let schema_body = r#"{"code":0,"msg":"ok","data":{"schema":{"id":"schema-1"}}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, schema_body),
        http_response(200, schema_body),
        http_response(200, schema_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateSchemaReqBody {
        id: Some("schema-1".into()),
        properties: Some(vec![SchemaProperty {
            name: Some("title".into()),
            r#type: Some("text".into()),
            is_searchable: Some(true),
            ..Default::default()
        }]),
        ..Default::default()
    };
    let patch_body = PatchSchemaReqBody {
        properties: Some(vec![SchemaProperty {
            name: Some("updated_at".into()),
            r#type: Some("timestamp".into()),
            is_sortable: Some(true),
            ..Default::default()
        }]),
        ..Default::default()
    };

    client
        .search()
        .data_source_schema
        .create_by_query(
            &CreateDataSourceSchemaQuery::new("ds-1", &create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .data_source_schema
        .get_by_query(
            &GetDataSourceSchemaQuery::new("ds-1", "schema-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .data_source_schema
        .patch_by_query(
            &PatchDataSourceSchemaQuery::new("ds-1", "schema-1", &patch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .data_source_schema
        .delete_by_query(
            &DeleteDataSourceSchemaQuery::new("ds-1", "schema-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/search/v2/data_sources/ds-1/schemas "));
    assert!(request.contains("GET /open-apis/search/v2/data_sources/ds-1/schemas/schema-1 "));
    assert!(request.contains("PATCH /open-apis/search/v2/data_sources/ds-1/schemas/schema-1 "));
    assert!(request.contains("DELETE /open-apis/search/v2/data_sources/ds-1/schemas/schema-1 "));
    assert!(request.contains(r#""id":"schema-1""#));
    assert!(request.contains(r#""name":"title""#));
    assert!(request.contains(r#""name":"updated_at""#));
}

#[tokio::test]
async fn search_schema_by_query_smoke() {
    let schema_body = r#"{"code":0,"msg":"ok","data":{"schema":{"id":"schema-1"}}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, schema_body),
        http_response(200, schema_body),
        http_response(200, schema_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateSchemaReqBody {
        id: Some("schema-1".into()),
        properties: Some(vec![SchemaProperty {
            name: Some("owner".into()),
            r#type: Some("text".into()),
            is_filterable: Some(true),
            ..Default::default()
        }]),
        ..Default::default()
    };
    let patch_body = PatchSchemaReqBody {
        properties: Some(vec![SchemaProperty {
            name: Some("priority".into()),
            r#type: Some("number".into()),
            is_sortable: Some(true),
            ..Default::default()
        }]),
        ..Default::default()
    };

    client
        .search()
        .schema
        .create_by_query(
            &CreateSchemaQuery::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .schema
        .get_by_query(&GetSchemaQuery::new("schema-1"), &RequestOption::default())
        .await
        .unwrap();
    client
        .search()
        .schema
        .patch_by_query(
            &PatchSchemaQuery::new("schema-1", &patch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .schema
        .delete_by_query(
            &DeleteSchemaQuery::new("schema-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/search/v2/schemas "));
    assert!(request.contains("GET /open-apis/search/v2/schemas/schema-1 "));
    assert!(request.contains("PATCH /open-apis/search/v2/schemas/schema-1 "));
    assert!(request.contains("DELETE /open-apis/search/v2/schemas/schema-1 "));
    assert!(request.contains(r#""name":"owner""#));
    assert!(request.contains(r#""name":"priority""#));
}
