use super::prelude::*;

// ── Task ──

#[tokio::test]
async fn task_create_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"task":{"id":"task-1","summary":"New task"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let req_body = CreateTaskReqBody {
        summary: Some("New task".into()),
        ..Default::default()
    };
    let resp = client
        .task()
        .task
        .create(&req_body, Some("open_id"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let task = resp.data.unwrap().task.unwrap();
    assert_eq!(task.summary.as_deref(), Some("New task"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v1/tasks?user_id_type=open_id"));
    assert!(request.contains(r#""summary":"New task""#));
}

#[tokio::test]
async fn task_get_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"task":{"id":"task-1","summary":"Fix bug"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task()
        .task
        .get("task-1", None, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let task = resp.data.unwrap().task.unwrap();
    assert_eq!(task.id.as_deref(), Some("task-1"));
    assert_eq!(task.summary.as_deref(), Some("Fix bug"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v1/tasks/task-1"));
}

#[tokio::test]
async fn task_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"task-1","summary":"Fix bug"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task()
        .task
        .list(
            Some(20),
            Some("next-page"),
            Some("1700000000"),
            Some("1700000100"),
            Some(false),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v1/tasks?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("start_create_time=1700000000"));
    assert!(request.contains("end_create_time=1700000100"));
    assert!(request.contains("task_completed=false"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"task-1","summary":"Fix bug"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListTaskQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .task_completed(false)
        .user_id_type("open_id");
    let resp = client
        .task()
        .task
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].summary.as_deref(), Some("Fix bug"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v1/tasks?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("task_completed=false"));
    assert!(request.contains("user_id_type=open_id"));
}
