use super::prelude::*;

// Task v2 task smoke tests

#[tokio::test]
async fn task_v2_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"task":{"guid":"task-guid-1","summary":"Fix bug","due":{"timestamp":"1700000000000","is_all_day":false},"members":[{"id":"ou-1","type":"user"}],"attachments":[{"guid":"att-1","resource":{"type":"task","id":"task-guid-1"}}],"origin":{"platform_i18n_name":{"en_us":"Import"},"href":{"url":"https://example.com"}},"custom_fields":[{"guid":"field-1","member_value":[{"id":"ou-1"}]}],"dependencies":[{"type":"finish_to_start","task_guid":"task-0"}]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetTaskV2Query::new("task-guid-1").user_id_type("open_id");
    let resp = client
        .task_v2()
        .task
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let task = resp.data.unwrap().task.unwrap();
    assert_eq!(task.guid.as_deref(), Some("task-guid-1"));
    assert_eq!(task.summary.as_deref(), Some("Fix bug"));
    assert_eq!(
        task.due.unwrap().timestamp.as_deref(),
        Some("1700000000000")
    );
    assert_eq!(task.members.unwrap()[0].type_.as_deref(), Some("user"));
    assert_eq!(
        task.attachments.unwrap()[0]
            .resource
            .as_ref()
            .unwrap()
            .id
            .as_deref(),
        Some("task-guid-1")
    );
    assert_eq!(
        task.origin
            .unwrap()
            .platform_i18n_name
            .unwrap()
            .en_us
            .as_deref(),
        Some("Import")
    );
    assert_eq!(
        task.custom_fields.unwrap()[0]
            .member_value
            .as_ref()
            .unwrap()[0]
            .id
            .as_deref(),
        Some("ou-1")
    );
    assert_eq!(
        task.dependencies.unwrap()[0].task_guid.as_deref(),
        Some("task-0")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/tasks/task-guid-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_list_positional_adapter_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"task-guid-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task_v2()
        .task
        .list(
            Some(20),
            Some("next-page"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/tasks?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"task-guid-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListTaskV2Query::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .completed(false)
        .task_type("my_tasks")
        .agent_task_status(1)
        .user_id_type("open_id");
    let resp = client
        .task_v2()
        .task
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].guid.as_deref(), Some("task-guid-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/tasks?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("completed=false"));
    assert!(request.contains("type=my_tasks"));
    assert!(request.contains("agent_task_status=1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_task_write_by_query_smoke() {
    let task_body =
        r#"{"code":0,"msg":"ok","data":{"task":{"guid":"task-guid-1","summary":"Fix bug"}}}"#;
    let list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"task-guid-1"}],"has_more":false}}"#;
    let tasklist_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"tasklist-guid-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, task_body),
        http_response(200, task_body),
        http_response(200, task_body),
        http_response(200, tasklist_body),
        http_response(200, task_body),
        http_response(200, list_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"summary":"Fix bug"});
    let patch_body = serde_json::json!({"summary":"Fix bug updated"});
    let members_body = serde_json::json!({"members":["u-1"]});
    let subtask_body = serde_json::json!({"summary":"Child task"});

    client
        .task_v2()
        .task
        .create_by_query(
            &CreateTaskV2Query::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .task
        .patch_by_query(
            &PatchTaskV2Query::new("task-guid-1", &patch_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .task
        .add_members_by_query(
            &AddMembersTaskV2Query::new("task-guid-1", &members_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .task
        .tasklists_by_query(
            &TasklistsTaskV2Query::new("task-guid-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .task
        .create_subtask_by_query(
            &CreateTaskSubtaskV2Query::new("task-guid-1", &subtask_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .task
        .list_subtasks_by_query(
            &ListTaskSubtaskV2Query::new("task-guid-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/tasks?"));
    assert!(request.contains("PATCH /open-apis/task/v2/tasks/task-guid-1?"));
    assert!(request.contains("POST /open-apis/task/v2/tasks/task-guid-1/add_members "));
    assert!(request.contains("GET /open-apis/task/v2/tasks/task-guid-1/tasklists?"));
    assert!(request.contains("POST /open-apis/task/v2/tasks/task-guid-1/subtasks?"));
    assert!(request.contains("GET /open-apis/task/v2/tasks/task-guid-1/subtasks?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""summary":"Fix bug""#));
    assert!(request.contains(r#""summary":"Fix bug updated""#));
    assert!(request.contains(r#""members":["u-1"]"#));
    assert!(request.contains(r#""summary":"Child task""#));
}

#[tokio::test]
async fn task_v2_delete_by_query_smoke() {
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, empty_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    client
        .task_v2()
        .task
        .delete_by_query(&DeleteTaskV2Query::new("task-1"), &RequestOption::default())
        .await
        .unwrap();
    client
        .task_v2()
        .attachment
        .delete_by_query(
            &DeleteAttachmentV2Query::new("attachment-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .comment
        .delete_by_query(
            &DeleteCommentV2Query::new("comment-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .section
        .delete_by_query(
            &DeleteSectionV2Query::new("section-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .delete_by_query(
            &DeleteTasklistV2Query::new("tasklist-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .delete_activity_subscription_by_query(
            &DeleteActivitySubscriptionV2Query::new("tasklist-1", "sub-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("DELETE /open-apis/task/v2/tasks/task-1 "));
    assert!(request.contains("DELETE /open-apis/task/v2/attachments/attachment-1 "));
    assert!(request.contains("DELETE /open-apis/task/v2/comments/comment-1 "));
    assert!(request.contains("DELETE /open-apis/task/v2/sections/section-1 "));
    assert!(request.contains("DELETE /open-apis/task/v2/tasklists/tasklist-1 "));
    assert!(
        request.contains(
            "DELETE /open-apis/task/v2/tasklists/tasklist-1/activity_subscriptions/sub-1 "
        )
    );
}
