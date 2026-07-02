use super::*;

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
        task: Some(serde_json::json!({"summary":"Updated task"})),
        update_fields: Some(vec!["summary".to_string()]),
    };
    let batch_delete_body = serde_json::json!({"ids":["u-1"]});

    client
        .task()
        .task
        .create_by_query(
            &CreateTaskV1Query::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .task
        .get_by_query(
            &GetTaskV1Query::new("task-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .task
        .patch_by_query(
            &PatchTaskV1Query::new("task-1", &patch_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .task
        .delete_by_query(&DeleteTaskQuery::new("task-1"), &RequestOption::default())
        .await
        .unwrap();
    client
        .task()
        .task
        .complete_by_query(&CompleteTaskQuery::new("task-1"), &RequestOption::default())
        .await
        .unwrap();
    client
        .task()
        .task
        .uncomplete_by_query(
            &UncompleteTaskQuery::new("task-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .task
        .batch_delete_collaborator_by_query(
            &BatchDeleteCollaboratorQuery::new("task-1", &batch_delete_body)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .task
        .batch_delete_follower_by_query(
            &BatchDeleteFollowerQuery::new("task-1", &batch_delete_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
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

#[tokio::test]
async fn task_comment_by_query_smoke() {
    let comment_body =
        r#"{"code":0,"msg":"ok","data":{"comment":{"id":"comment-1","content":"First"}}}"#;
    let comment_list_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"comment-1","content":"First"}],"has_more":false}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, comment_body),
        http_response(200, comment_body),
        http_response(200, comment_body),
        http_response(200, empty_body),
        http_response(200, comment_list_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateCommentReqBody {
        content: Some("First".into()),
        ..Default::default()
    };
    let update_body = UpdateCommentReqBody {
        content: Some("Updated".into()),
    };

    client
        .task()
        .comment
        .create_by_query(
            &CreateTaskCommentQuery::new("task-1", &create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .comment
        .get_by_query(
            &GetTaskCommentQuery::new("task-1", "comment-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .comment
        .update_by_query(
            &UpdateTaskCommentQuery::new("task-1", "comment-1", &update_body)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .comment
        .delete_by_query(
            &DeleteCommentQuery::new("task-1", "comment-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .comment
        .list_by_query(
            &ListTaskCommentQuery::new("task-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v1/tasks/task-1/comments?"));
    assert!(request.contains("GET /open-apis/task/v1/tasks/task-1/comments/comment-1?"));
    assert!(request.contains("PUT /open-apis/task/v1/tasks/task-1/comments/comment-1?"));
    assert!(request.contains("DELETE /open-apis/task/v1/tasks/task-1/comments/comment-1 "));
    assert!(request.contains("GET /open-apis/task/v1/tasks/task-1/comments?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""content":"First""#));
    assert!(request.contains(r#""content":"Updated""#));
}

#[tokio::test]
async fn task_member_by_query_smoke() {
    let follower_body =
        r#"{"code":0,"msg":"ok","data":{"follower":{"id":"u-1","id_type":"open_id"}}}"#;
    let collaborator_body =
        r#"{"code":0,"msg":"ok","data":{"collaborator":{"id":"u-1","id_type":"open_id"}}}"#;
    let list_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"u-1","id_type":"open_id"}],"has_more":false}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, follower_body),
        http_response(200, empty_body),
        http_response(200, list_body),
        http_response(200, collaborator_body),
        http_response(200, empty_body),
        http_response(200, list_body),
    ])
    .await;

    let client = client_for(addr);
    let follower_body = CreateFollowerReqBody {
        id: Some("u-1".into()),
        id_type: Some("open_id".into()),
    };
    let collaborator_body = CreateCollaboratorReqBody {
        id: Some("u-1".into()),
        id_type: Some("open_id".into()),
    };

    client
        .task()
        .follower
        .create_by_query(
            &CreateFollowerQuery::new("task-1", &follower_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .follower
        .delete_by_query(
            &DeleteFollowerQuery::new("task-1", "u-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .follower
        .list_by_query(
            &ListFollowerQuery::new("task-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .collaborator
        .create_by_query(
            &CreateCollaboratorQuery::new("task-1", &collaborator_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .collaborator
        .delete_by_query(
            &DeleteCollaboratorQuery::new("task-1", "u-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .collaborator
        .list_by_query(
            &ListCollaboratorQuery::new("task-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v1/tasks/task-1/followers?"));
    assert!(request.contains("DELETE /open-apis/task/v1/tasks/task-1/followers/u-1?"));
    assert!(request.contains("GET /open-apis/task/v1/tasks/task-1/followers?"));
    assert!(request.contains("POST /open-apis/task/v1/tasks/task-1/collaborators?"));
    assert!(request.contains("DELETE /open-apis/task/v1/tasks/task-1/collaborators/u-1?"));
    assert!(request.contains("GET /open-apis/task/v1/tasks/task-1/collaborators?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""id":"u-1""#));
    assert!(request.contains(r#""id_type":"open_id""#));
}

#[tokio::test]
async fn task_reminder_by_query_smoke() {
    let reminder_body = r#"{"code":0,"msg":"ok","data":{"reminder":{"id":"reminder-1","relative_fire_minute":30}}}"#;
    let reminder_list_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"reminder-1","relative_fire_minute":30}],"has_more":false}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, reminder_body),
        http_response(200, reminder_body),
        http_response(200, reminder_body),
        http_response(200, empty_body),
        http_response(200, reminder_list_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateReminderReqBody {
        relative_fire_minute: Some(30),
        is_notify_app_center: Some(true),
        is_notify_lark: Some(false),
    };
    let update_body = UpdateReminderReqBody {
        relative_fire_minute: Some(60),
        is_notify_app_center: Some(false),
        is_notify_lark: Some(true),
    };

    client
        .task()
        .reminder
        .create_by_query(
            &CreateReminderQuery::new("task-1", &create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .reminder
        .get_by_query(
            &GetReminderQuery::new("task-1", "reminder-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .reminder
        .update_by_query(
            &UpdateReminderQuery::new("task-1", "reminder-1", &update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .reminder
        .delete_by_query(
            &DeleteReminderQuery::new("task-1", "reminder-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .reminder
        .list_by_query(
            &ListReminderQuery::new("task-1")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v1/tasks/task-1/reminders "));
    assert!(request.contains("GET /open-apis/task/v1/tasks/task-1/reminders/reminder-1 "));
    assert!(request.contains("PUT /open-apis/task/v1/tasks/task-1/reminders/reminder-1 "));
    assert!(request.contains("DELETE /open-apis/task/v1/tasks/task-1/reminders/reminder-1 "));
    assert!(request.contains("GET /open-apis/task/v1/tasks/task-1/reminders?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""relative_fire_minute":30"#));
    assert!(request.contains(r#""is_notify_app_center":true"#));
    assert!(request.contains(r#""relative_fire_minute":60"#));
    assert!(request.contains(r#""is_notify_lark":true"#));
}
