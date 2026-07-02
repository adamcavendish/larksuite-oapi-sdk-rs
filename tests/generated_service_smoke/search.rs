use super::*;

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

#[tokio::test]
async fn search_message_app_doc_wiki_by_query_smoke() {
    let message_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"message_id":"om_1"}],"has_more":false}}"#;
    let app_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"app_id":"cli_a"}],"has_more":false}}"#;
    let doc_wiki_body =
        r#"{"code":0,"msg":"ok","data":{"total":1,"has_more":false,"res_units":[{"id":"doc-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, message_body),
        http_response(200, app_body),
        http_response(200, doc_wiki_body),
    ])
    .await;

    let client = client_for(addr);
    let message_query_body = SearchMessageReqBody {
        query: Some("status update".into()),
        from_ids: Some(vec!["ou-1".into()]),
        chat_ids: Some(vec!["oc-1".into()]),
        with_url: Some(true),
        page_size: Some(20),
        page_token: Some("next-page".into()),
        ..Default::default()
    };
    let app_query_body = SearchAppReqBody {
        query: Some("approval".into()),
        page_size: Some(10),
        page_token: Some("app-page".into()),
    };
    let doc_wiki_query_body = SearchDocWikiReqBody {
        query: Some("handbook".into()),
        doc_filter: Some(serde_json::json!({"owner_ids":["ou-1"]})),
        page_token: Some("doc-page".into()),
        page_size: Some(5),
        ..Default::default()
    };

    client
        .search()
        .message
        .create_by_query(
            &CreateMessageSearchQuery::new(&message_query_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .app
        .create_by_query(
            &CreateAppSearchQuery::new(&app_query_body).user_id_type("union_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .doc_wiki
        .search_by_query(
            &SearchDocWikiQuery::new(&doc_wiki_query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/search/v2/message?user_id_type=open_id "));
    assert!(request.contains("POST /open-apis/search/v2/app?user_id_type=union_id "));
    assert!(request.contains("POST /open-apis/search/v2/doc_wiki/search "));
    assert!(request.contains(r#""query":"status update""#));
    assert!(request.contains(r#""with_url":true"#));
    assert!(request.contains(r#""query":"approval""#));
    assert!(request.contains(r#""query":"handbook""#));
    assert!(request.contains(r#""owner_ids":["ou-1"]"#));
}

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
