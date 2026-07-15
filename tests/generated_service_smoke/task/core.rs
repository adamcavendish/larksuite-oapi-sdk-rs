use super::prelude::*;

// ── Task ──

#[tokio::test]
async fn task_core_by_query_smoke() {
    let task_body = r#"{"code":0,"msg":"ok","data":{"task":{"id":"task-1","summary":"New task"}}}"#;
    let batch_body = r#"{"code":0,"msg":"ok","data":{"deleted":["u-1"]}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, task_body),
        http_response(200, task_body),
        http_response(200, task_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
        http_response(200, batch_body),
        http_response(200, batch_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateTaskReqBody {
        summary: Some("New task".into()),
        ..Default::default()
    };
    let patch_body = PatchTaskReqBody {
        task: Some(Task {
            summary: Some("Updated task".into()),
            ..Default::default()
        }),
        update_fields: Some(vec!["summary".to_string()]),
    };
    let batch_delete_body = serde_json::json!({"ids":["u-1"]});

    Box::pin(client.task().task.create_by_query(
        &CreateTaskV1Query::new(&create_body).user_id_type("open_id"),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(client.task().task.get_by_query(
        &GetTaskV1Query::new("task-1").user_id_type("open_id"),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(client.task().task.patch_by_query(
        &PatchTaskV1Query::new("task-1", &patch_body).user_id_type("open_id"),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(
        client
            .task()
            .task
            .delete_by_query(&DeleteTaskQuery::new("task-1"), &RequestOption::default()),
    )
    .await
    .unwrap();
    Box::pin(
        client
            .task()
            .task
            .complete_by_query(&CompleteTaskQuery::new("task-1"), &RequestOption::default()),
    )
    .await
    .unwrap();
    Box::pin(client.task().task.uncomplete_by_query(
        &UncompleteTaskQuery::new("task-1"),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(client.task().task.batch_delete_collaborator_by_query(
        &BatchDeleteCollaboratorQuery::new("task-1", &batch_delete_body).user_id_type("open_id"),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(client.task().task.batch_delete_follower_by_query(
        &BatchDeleteFollowerQuery::new("task-1", &batch_delete_body).user_id_type("open_id"),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v1/tasks?"));
    assert!(request.contains("GET /open-apis/task/v1/tasks/task-1?"));
    assert!(request.contains("PATCH /open-apis/task/v1/tasks/task-1?"));
    assert!(request.contains("DELETE /open-apis/task/v1/tasks/task-1 "));
    assert!(request.contains("POST /open-apis/task/v1/tasks/task-1/complete "));
    assert!(request.contains("POST /open-apis/task/v1/tasks/task-1/uncomplete "));
    assert!(request.contains("POST /open-apis/task/v1/tasks/task-1/batch_delete_collaborator?"));
    assert!(request.contains("POST /open-apis/task/v1/tasks/task-1/batch_delete_follower?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""summary":"New task""#));
    assert!(request.contains(r#""summary":"Updated task""#));
    assert!(request.contains(r#""update_fields":["summary"]"#));
    assert!(request.contains(r#""ids":["u-1"]"#));
}
