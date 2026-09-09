use super::prelude::*;

// ── Docs AI ──

#[tokio::test]
async fn docs_ai_document_content_contract_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let async_task_body =
        r#"{"code":0,"msg":"ok","data":{"task":{"task_id":"task id/one","status":"succeeded"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, async_task_body),
        http_response(200, async_task_body),
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
            json_value!({
                "title": "Spec",
                "content": "<p>hello</p>",
                "format": "xml",
                "extra_param": r#"{"open_create_async":true}"#,
            }),
            &user_option,
        )
        .await
        .unwrap();
    let async_task_as_user = client
        .docs_ai()
        .async_task
        .get("task id/one", &user_option)
        .await
        .unwrap();
    let async_task_as_tenant = client
        .docs_ai()
        .async_task
        .get("task id/one", &tenant_option)
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
    assert!(async_task_as_user.success());
    assert!(async_task_as_tenant.success());
    assert_eq!(
        async_task_as_user
            .data
            .as_ref()
            .and_then(|data| data.get("task"))
            .and_then(|task| task.get("status"))
            .and_then(serde_json::Value::as_str),
        Some("succeeded")
    );

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 8);
    let (create_headers, create_body) = requests[0].split_once("\r\n\r\n").unwrap();
    assert_eq!(
        create_headers.lines().next(),
        Some("POST /open-apis/docs_ai/v1/documents HTTP/1.1")
    );
    let create_body: serde_json::Value = serde_json::from_str(create_body).unwrap();
    assert_eq!(
        create_body["extra_param"],
        serde_json::json!(r#"{"open_create_async":true}"#)
    );
    for (request, token) in [(&requests[1], "user-token"), (&requests[2], "tenant-token")] {
        let (headers, body) = request.split_once("\r\n\r\n").unwrap();
        assert_eq!(
            headers.lines().next(),
            Some("GET /open-apis/docs_ai/v1/async_tasks/task%20id%2Fone HTTP/1.1")
        );
        let authorization: Vec<_> = headers
            .lines()
            .filter_map(|line| line.split_once(':'))
            .filter(|(name, _)| name.eq_ignore_ascii_case("authorization"))
            .map(|(_, value)| value.trim())
            .collect();
        assert_eq!(authorization, vec![format!("Bearer {token}")]);
        assert!(body.is_empty());
    }
    let request = requests.join("\n");
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
    assert!(request.contains("GET /open-apis/docs_ai/v1/async_tasks/task%20id%2Fone "));
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
