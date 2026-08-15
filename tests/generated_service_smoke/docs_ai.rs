use super::prelude::*;

// ── Docs AI ──

#[tokio::test]
async fn docs_ai_document_content_contract_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let user_option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    };
    let tenant_option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..RequestOption::default()
    };
    let document_id = "doxcn doc/a";

    let create = client
        .docs_ai()
        .document
        .create(
            json_value!({"title": "Spec", "content": "<p>hello</p>", "format": "xml"}),
            &user_option,
        )
        .await
        .unwrap();
    let fetch = client
        .docs_ai()
        .document
        .fetch(
            document_id,
            json_value!({
                "format": "xml",
                "extra_param": r#"{"enable_user_cite_reference_map":true,"include_comments":true,"return_html5_block_data":true}"#,
                "read_option": {
                    "read_mode": "range",
                    "start_block_id": "blk first",
                    "end_block_id": "blk/last",
                },
            }),
            &tenant_option,
        )
        .await
        .unwrap();
    let update = client
        .docs_ai()
        .document
        .update(
            document_id,
            json_value!({
                "format": "xml",
                "command": "block_replace",
                "content": "<p>replacement</p>",
                "start_block_id": "blk first",
                "end_block_id": "blk/last",
            }),
            &user_option,
        )
        .await
        .unwrap();
    let history = client
        .docs_ai()
        .history
        .list(
            &ListDocumentHistoryQuery::new(document_id)
                .page(PageQuery::new().page_size(20).page_token("next page")),
            &tenant_option,
        )
        .await
        .unwrap();
    let revert = client
        .docs_ai()
        .history
        .revert(
            document_id,
            json_value!({"history_version_id": "42", "wait_timeout_ms": 0}),
            &user_option,
        )
        .await
        .unwrap();
    let status = client
        .docs_ai()
        .history
        .revert_status(
            &GetDocumentHistoryRevertStatusQuery::new(document_id, "task id"),
            &tenant_option,
        )
        .await
        .unwrap();

    assert!(create.success());
    assert!(fetch.success());
    assert!(update.success());
    assert!(history.success());
    assert!(revert.success());
    assert!(status.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/docs_ai/v1/documents "));
    assert!(request.contains("POST /open-apis/docs_ai/v1/documents/doxcn%20doc%2Fa/fetch "));
    assert!(request.contains("PUT /open-apis/docs_ai/v1/documents/doxcn%20doc%2Fa "));
    assert!(request.contains("GET /open-apis/docs_ai/v1/documents/doxcn%20doc%2Fa/histories?"));
    assert!(
        request.contains("POST /open-apis/docs_ai/v1/documents/doxcn%20doc%2Fa/history/revert ")
    );
    assert!(
        request
            .contains("GET /open-apis/docs_ai/v1/documents/doxcn%20doc%2Fa/history/revert_status?")
    );
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next+page"));
    assert!(request.contains("task_id=task+id"));
    assert!(request.contains("authorization: Bearer user-token"));
    assert!(request.contains("authorization: Bearer tenant-token"));
    assert!(request.contains(r#""extra_param":"#));
    assert!(request.contains("include_comments"));
    assert!(request.contains(r#""start_block_id":"blk first"#));
    assert!(request.contains(r#""end_block_id":"blk/last"#));
    assert!(request.contains(r#""history_version_id":"42"#));
    assert!(request.contains(r#""wait_timeout_ms":0"#));
}
