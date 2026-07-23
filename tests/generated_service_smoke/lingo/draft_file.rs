use super::prelude::*;

// ── Lingo ──

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
    let draft_create_body = json_value!({"entity_id":"entity-1"});
    let draft_update_body = json_value!({"entity_id":"entity-1","status":"updated"});
    let file_upload_body = FormDataBuilder::new()
        .file(
            "file",
            "term.png",
            b"file-bytes".to_vec(),
            Some("image/png"),
        )
        .field("file_name", "term.png")
        .build();

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
    assert!(request.contains("name=\"file_name\""));
    assert!(request.contains("file-bytes"));
}
