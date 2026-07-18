use super::prelude::*;

// ── Baike ──

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
    let draft_create_body = json_value!({"entity_id":"entity-1"});
    let draft_update_body = json_value!({"entity_id":"entity-1","status":"updated"});
    let file_upload_body = json_value!({"file_name":"term.png","file":"base64-file"});

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
