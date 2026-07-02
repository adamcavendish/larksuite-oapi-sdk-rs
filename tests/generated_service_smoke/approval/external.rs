use super::prelude::*;

// ── Approval ──

#[tokio::test]
async fn approval_external_approval_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"approval_code":"apv-1","approval_name":"External"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .external_approval
        .get_by_query(
            &GetExternalApprovalQuery::new("apv-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.approval_name.as_deref()),
        Some("External")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/external_approvals/apv-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn approval_external_task_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"task_id":"task-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = ListExternalTaskReqBody {
        approval_codes: vec!["apv-1".to_string()],
        user_ids: vec!["ou-1".to_string()],
        ..Default::default()
    };
    let resp = client
        .approval()
        .external_task
        .list_by_query(
            &ListExternalTaskQuery::new(&body)
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/external_tasks?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""approval_codes":["apv-1"]"#));
    assert!(request.contains(r#""user_ids":["ou-1"]"#));
}
