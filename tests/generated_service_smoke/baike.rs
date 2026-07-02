use super::*;

// ── Baike ──

#[tokio::test]
async fn baike_entity_by_query_smoke() {
    let entity_body = r#"{"code":0,"msg":"ok","data":{"entity":{"id":"entity-1"}}}"#;
    let list_body =
        r#"{"code":0,"msg":"ok","data":{"entities":[{"id":"entity-1"}],"has_more":false}}"#;
    let search_body =
        r#"{"code":0,"msg":"ok","data":{"entities":[{"id":"entity-1"}],"has_more":false}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let value_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"entity-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, entity_body),
        http_response(200, entity_body),
        http_response(200, entity_body),
        http_response(200, list_body),
        http_response(200, search_body),
        http_response(200, empty_body),
        http_response(200, value_body),
        http_response(200, value_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateBaikeEntityReqBody {
        main_keys: Some(vec![BaikeTerm {
            key: Some("Term".into()),
            ..Default::default()
        }]),
        description: Some("Description".into()),
        ..Default::default()
    };
    let update_body = CreateBaikeEntityReqBody {
        main_keys: Some(vec![BaikeTerm {
            key: Some("Term updated".into()),
            ..Default::default()
        }]),
        ..Default::default()
    };
    let search_body = SearchBaikeEntityReqBody {
        query: Some("term".into()),
        sources: Some(vec![1]),
        ..Default::default()
    };
    let highlight_body = serde_json::json!({"text":"Term highlight"});
    let extract_body = serde_json::json!({"text":"Term extract"});
    let match_body = serde_json::json!({"text":"Term match"});

    let create_resp = client
        .baike()
        .entity
        .create_by_query(
            &CreateBaikeEntityQuery::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_resp = client
        .baike()
        .entity
        .update_by_query(
            &UpdateBaikeEntityQuery::new("entity-1", &update_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let get_resp = client
        .baike()
        .entity
        .get_by_query(
            &GetBaikeEntityQuery::new("entity-1")
                .provider("provider-1")
                .outer_id("outer-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let list_resp = client
        .baike()
        .entity
        .list_by_query(
            &ListBaikeEntityQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .provider("provider-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let search_resp = client
        .baike()
        .entity
        .search_by_query(
            &SearchBaikeEntityQuery::new(&search_body)
                .page(PageQuery::new().page_size(10).page_token("search-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let highlight_resp = client
        .baike()
        .entity
        .highlight_by_query(
            &HighlightBaikeEntityQuery::new(&highlight_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let extract_resp = client
        .baike()
        .entity
        .extract_by_query(
            &ExtractBaikeEntityQuery::new(&extract_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let match_resp = client
        .baike()
        .entity
        .match_by_query(
            &MatchBaikeEntityQuery::new(&match_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(update_resp.success());
    assert!(get_resp.success());
    assert!(list_resp.success());
    assert!(search_resp.success());
    assert!(highlight_resp.success());
    assert!(extract_resp.success());
    assert!(match_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/baike/v1/entities?"));
    assert!(request.contains("PUT /open-apis/baike/v1/entities/entity-1?"));
    assert!(request.contains("GET /open-apis/baike/v1/entities/entity-1?"));
    assert!(request.contains("GET /open-apis/baike/v1/entities?"));
    assert!(request.contains("POST /open-apis/baike/v1/entities/search?"));
    assert!(request.contains("POST /open-apis/baike/v1/entities/highlight"));
    assert!(request.contains("POST /open-apis/baike/v1/entities/extract"));
    assert!(request.contains("POST /open-apis/baike/v1/entities/match"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("provider=provider-1"));
    assert!(request.contains("outer_id=outer-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("page_size=10"));
    assert!(request.contains("page_token=search-page"));
    assert!(request.contains(r#""key":"Term""#));
    assert!(request.contains(r#""key":"Term updated""#));
    assert!(request.contains(r#""query":"term""#));
    assert!(request.contains(r#""sources":[1]"#));
    assert!(request.contains(r#""text":"Term highlight""#));
    assert!(request.contains(r#""text":"Term extract""#));
    assert!(request.contains(r#""text":"Term match""#));
}

#[tokio::test]
async fn baike_classification_draft_file_by_query_smoke() {
    let classification_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"class-1"}],"has_more":false}}"#;
    let draft_body = r#"{"code":0,"msg":"ok","data":{"draft_id":"draft-1"}}"#;
    let file_body = r#"{"code":0,"msg":"ok","data":{"file_token":"file-token-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, classification_body),
        http_response(200, draft_body),
        http_response(200, draft_body),
        http_response(200, file_body),
    ])
    .await;

    let client = client_for(addr);
    let draft_create_body = serde_json::json!({"entity_id":"entity-1"});
    let draft_update_body = serde_json::json!({"entity_id":"entity-1","status":"updated"});
    let file_upload_body = serde_json::json!({"file_name":"term.png","file":"base64-file"});

    let classification_resp = client
        .baike()
        .classification
        .list_by_query(
            &ListBaikeClassificationQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let create_resp = client
        .baike()
        .draft
        .create_by_query(
            &CreateBaikeDraftQuery::new(&draft_create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_resp = client
        .baike()
        .draft
        .update_by_query(
            &UpdateBaikeDraftQuery::new("draft-1", &draft_update_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let upload_resp = client
        .baike()
        .file
        .upload_by_query(
            &UploadBaikeFileQuery::new(&file_upload_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(classification_resp.success());
    assert!(create_resp.success());
    assert!(update_resp.success());
    assert!(upload_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/baike/v1/classifications?"));
    assert!(request.contains("POST /open-apis/baike/v1/drafts?"));
    assert!(request.contains("PUT /open-apis/baike/v1/drafts/draft-1?"));
    assert!(request.contains("POST /open-apis/baike/v1/files/upload"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""entity_id":"entity-1""#));
    assert!(request.contains(r#""status":"updated""#));
    assert!(request.contains(r#""file_name":"term.png""#));
    assert!(request.contains(r#""file":"base64-file""#));
}

#[tokio::test]
async fn baike_file_download_smoke() {
    let body = "baike-file-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"baike-file.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .baike()
        .file
        .download("file-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("baike-file.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/baike/v1/files/file-token-1/download"));
}
