use super::prelude::*;

// ── Approval ──

#[tokio::test]
async fn approval_task_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"task_id":"task-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .task
        .query_by_query(
            &QueryApprovalTaskQuery::new()
                .user_id("ou-1")
                .topic("assigned")
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/tasks/query?"));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("topic=assigned"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn approval_task_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"task_id":"task-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = TaskSearch {
        task_title: Some("Task".to_string()),
        user_id: Some("ou-1".to_string()),
        ..Default::default()
    };
    let resp = client
        .approval()
        .task
        .search_by_query(
            &SearchTaskQuery::new(&body)
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/approval/v4/tasks/search?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""task_title":"Task""#));
    assert!(request.contains(r#""user_id":"ou-1""#));
}
