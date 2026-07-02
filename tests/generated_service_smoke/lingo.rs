use super::*;

// ── Lingo ──

#[tokio::test]
async fn lingo_file_download_smoke() {
    let body = "lingo-file-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"lingo-file.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .lingo()
        .file
        .download("file-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("lingo-file.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/lingo/v1/files/file-token-1/download"));
}

#[tokio::test]
async fn lingo_entity_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"entity":{"id":"entity-1","description":"term"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .lingo()
        .entity
        .get_by_query(
            &GetLingoEntityQuery::new("entity-1")
                .provider("provider-1")
                .outer_id("outer-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.entity.as_ref())
            .and_then(|entity| entity.id.as_deref()),
        Some("entity-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/lingo/v1/entities/entity-1?"));
    assert!(request.contains("provider=provider-1"));
    assert!(request.contains("outer_id=outer-1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn lingo_entity_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"entities":[{"id":"entity-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .lingo()
        .entity
        .list_by_query(
            &ListLingoEntityQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .repo_id("repo-1")
                .provider("provider-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.entities.first())
            .and_then(|entity| entity.id.as_deref()),
        Some("entity-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/lingo/v1/entities?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("repo_id=repo-1"));
    assert!(request.contains("provider=provider-1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn lingo_entity_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"entities":[{"id":"entity-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = SearchLingoEntityReqBody {
        query: Some("term".to_string()),
        ..Default::default()
    };
    let resp = client
        .lingo()
        .entity
        .search_by_query(
            &SearchLingoEntityQuery::new(&body)
                .repo_id("repo-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.entities.first())
            .and_then(|entity| entity.id.as_deref()),
        Some("entity-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/lingo/v1/entities/search?"));
    assert!(request.contains("repo_id=repo-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""query":"term""#));
}

#[tokio::test]
async fn lingo_classification_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"class-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .lingo()
        .classification
        .list_by_query(
            &ListLingoClassificationQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert!(
        resp.data
            .as_ref()
            .and_then(|data| data.get("items"))
            .is_some()
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/lingo/v1/classifications?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn lingo_entity_write_by_query_smoke() {
    let entity_body = r#"{"code":0,"msg":"ok","data":{"entity":{"id":"entity-1"}}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let json_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"entity-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, entity_body),
        http_response(200, entity_body),
        http_response(200, empty_body),
        http_response(200, json_body),
        http_response(200, json_body),
    ])
    .await;

    let client = client_for(addr);
    let entity_create_body = CreateLingoEntityReqBody {
        main_keys: Some(vec![LingoTerm {
            key: Some("Term".to_string()),
            ..Default::default()
        }]),
        description: Some("Description".to_string()),
        ..Default::default()
    };
    let entity_update_body = CreateLingoEntityReqBody {
        main_keys: Some(vec![LingoTerm {
            key: Some("Term updated".to_string()),
            ..Default::default()
        }]),
        ..Default::default()
    };
    let highlight_body = serde_json::json!({"text":"Term highlight"});
    let match_body = serde_json::json!({"text":"Term match"});

    let create_resp = client
        .lingo()
        .entity
        .create_by_query(
            &CreateLingoEntityQuery::new(&entity_create_body)
                .repo_id("repo-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_resp = client
        .lingo()
        .entity
        .update_by_query(
            &UpdateLingoEntityQuery::new("entity-1", &entity_update_body)
                .repo_id("repo-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete_resp = client
        .lingo()
        .entity
        .delete_by_query(
            &DeleteLingoEntityQuery::new("entity-1")
                .provider("provider-1")
                .outer_id("outer-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let highlight_resp = client
        .lingo()
        .entity
        .highlight_by_query(
            &HighlightEntityQuery::new(&highlight_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let match_resp = client
        .lingo()
        .entity
        .match_by_query(
            &MatchEntityQuery::new(&match_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(update_resp.success());
    assert!(delete_resp.success());
    assert!(highlight_resp.success());
    assert!(match_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/lingo/v1/entities?"));
    assert!(request.contains("PUT /open-apis/lingo/v1/entities/entity-1?"));
    assert!(request.contains("DELETE /open-apis/lingo/v1/entities/entity-1?"));
    assert!(request.contains("POST /open-apis/lingo/v1/entities/highlight"));
    assert!(request.contains("POST /open-apis/lingo/v1/entities/match"));
    assert!(request.contains("repo_id=repo-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("provider=provider-1"));
    assert!(request.contains("outer_id=outer-1"));
    assert!(request.contains(r#""key":"Term""#));
    assert!(request.contains(r#""key":"Term updated""#));
    assert!(request.contains(r#""text":"Term highlight""#));
    assert!(request.contains(r#""text":"Term match""#));
}

#[tokio::test]
async fn lingo_draft_file_repo_by_query_smoke() {
    let draft_body = r#"{"code":0,"msg":"ok","data":{"draft_id":"draft-1"}}"#;
    let file_body = r#"{"code":0,"msg":"ok","data":{"file_token":"file-token-1"}}"#;
    let repo_body = r#"{"code":0,"msg":"ok","data":{"items":[{"repo_id":"repo-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, draft_body),
        http_response(200, draft_body),
        http_response(200, file_body),
        http_response(200, repo_body),
    ])
    .await;

    let client = client_for(addr);
    let draft_create_body = serde_json::json!({"entity_id":"entity-1"});
    let draft_update_body = serde_json::json!({"entity_id":"entity-1","status":"updated"});
    let file_upload_body = serde_json::json!({"file_name":"term.png","file":"base64-file"});

    let create_resp = client
        .lingo()
        .draft
        .create_by_query(
            &CreateLingoDraftQuery::new(&draft_create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_resp = client
        .lingo()
        .draft
        .update_by_query(
            &UpdateLingoDraftQuery::new("draft-1", &draft_update_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let upload_resp = client
        .lingo()
        .file
        .upload_by_query(
            &UploadLingoFileQuery::new(&file_upload_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let repo_resp = client
        .lingo()
        .repo
        .list_by_query(&ListLingoRepoQuery::new(), &RequestOption::default())
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(update_resp.success());
    assert!(upload_resp.success());
    assert!(repo_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/lingo/v1/drafts?"));
    assert!(request.contains("PUT /open-apis/lingo/v1/drafts/draft-1?"));
    assert!(request.contains("POST /open-apis/lingo/v1/files/upload"));
    assert!(request.contains("GET /open-apis/lingo/v1/repos"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""entity_id":"entity-1""#));
    assert!(request.contains(r#""status":"updated""#));
    assert!(request.contains(r#""file_name":"term.png""#));
    assert!(request.contains(r#""file":"base64-file""#));
}
