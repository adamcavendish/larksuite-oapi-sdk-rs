use super::prelude::*;

// ── Task v2 ──

#[tokio::test]
async fn task_v2_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"task":{"guid":"task-guid-1","summary":"Fix bug"}}}"#;
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
    assert_eq!(task["guid"].as_str(), Some("task-guid-1"));
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
    assert_eq!(data.items[0]["guid"].as_str(), Some("task-guid-1"));
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
async fn task_v2_attachment_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"attachment":{"guid":"att-1","name":"spec.pdf"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetAttachmentV2Query::new("att-1").user_id_type("open_id");
    let resp = client
        .task_v2()
        .attachment
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let attachment = resp.data.unwrap().attachment.unwrap();
    assert_eq!(attachment["guid"].as_str(), Some("att-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/attachments/att-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_attachment_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"att-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task_v2()
        .attachment
        .list(
            Some("task"),
            Some("task-guid-1"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/attachments?"));
    assert!(request.contains("resource_type=task"));
    assert!(request.contains("resource_id=task-guid-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn task_v2_attachment_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"att-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListAttachmentV2Query::new()
        .resource_type("task")
        .resource_id("task-guid-1")
        .updated_mesc("1700000000000")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .task_v2()
        .attachment
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0]["guid"].as_str(), Some("att-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/attachments?"));
    assert!(request.contains("resource_type=task"));
    assert!(request.contains("resource_id=task-guid-1"));
    assert!(request.contains("updated_mesc=1700000000000"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn task_v2_comment_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"comment":{"id":"comment-1","content":"Looks good"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetCommentV2Query::new("comment-1").user_id_type("open_id");
    let resp = client
        .task_v2()
        .comment
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let comment = resp.data.unwrap().comment.unwrap();
    assert_eq!(comment["id"].as_str(), Some("comment-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/comments/comment-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_comment_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"comment-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task_v2()
        .comment
        .list(
            Some("task"),
            Some("task-guid-1"),
            Some(20),
            Some("next-page"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/comments?"));
    assert!(request.contains("resource_type=task"));
    assert!(request.contains("resource_id=task-guid-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_comment_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"comment-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCommentV2Query::new()
        .resource_type("task")
        .resource_id("task-guid-1")
        .direction("asc")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .task_v2()
        .comment
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0]["id"].as_str(), Some("comment-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/comments?"));
    assert!(request.contains("resource_type=task"));
    assert!(request.contains("resource_id=task-guid-1"));
    assert!(request.contains("direction=asc"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn task_v2_custom_field_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"custom_field":{"guid":"field-1","name":"Priority"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetCustomFieldV2Query::new("field-1").user_id_type("open_id");
    let resp = client
        .task_v2()
        .custom_field
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let custom_field = resp.data.unwrap().custom_field.unwrap();
    assert_eq!(custom_field["guid"].as_str(), Some("field-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/custom_fields/field-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_custom_field_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"field-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task_v2()
        .custom_field
        .list(
            Some("tasklist"),
            Some("tasklist-guid-1"),
            Some(20),
            Some("next-page"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/custom_fields?"));
    assert!(request.contains("resource_type=tasklist"));
    assert!(request.contains("resource_id=tasklist-guid-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_custom_field_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"field-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCustomFieldV2Query::new()
        .resource_type("tasklist")
        .resource_id("tasklist-guid-1")
        .update_msec("1700000000000")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .task_v2()
        .custom_field
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0]["guid"].as_str(), Some("field-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/custom_fields?"));
    assert!(request.contains("resource_type=tasklist"));
    assert!(request.contains("resource_id=tasklist-guid-1"));
    assert!(request.contains("update_msec=1700000000000"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
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

#[tokio::test]
async fn task_v2_attachment_upload_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"attachment":{"guid":"att-1","name":"spec.pdf"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let upload_body = serde_json::json!({"file_token":"file-1"});
    let resp = client
        .task_v2()
        .attachment
        .upload_by_query(
            &UploadAttachmentV2Query::new(&upload_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/attachments/upload "));
    assert!(request.contains(r#""file_token":"file-1""#));
}

#[tokio::test]
async fn task_v2_comment_write_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"comment":{"id":"comment-1","content":"Looks good"}}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"content":"Looks good"});
    let patch_body = serde_json::json!({"content":"Ship it"});
    client
        .task_v2()
        .comment
        .create_by_query(
            &CreateCommentV2Query::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .comment
        .patch_by_query(
            &PatchCommentV2Query::new("comment-1", &patch_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/comments?"));
    assert!(request.contains("PATCH /open-apis/task/v2/comments/comment-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""content":"Looks good""#));
    assert!(request.contains(r#""content":"Ship it""#));
}

#[tokio::test]
async fn task_v2_custom_field_write_by_query_smoke() {
    let field_body =
        r#"{"code":0,"msg":"ok","data":{"custom_field":{"guid":"field-1","name":"Priority"}}}"#;
    let option_body = r#"{"code":0,"msg":"ok","data":{"option":{"guid":"option-1"}}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, field_body),
        http_response(200, field_body),
        http_response(200, field_body),
        http_response(200, empty_body),
        http_response(200, option_body),
        http_response(200, option_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"name":"Priority"});
    let patch_body = serde_json::json!({"name":"Priority updated"});
    let bind_body = serde_json::json!({"resource_type":"tasklist","resource_id":"tasklist-1"});
    let option_body = serde_json::json!({"name":"High"});
    let option_patch_body = serde_json::json!({"name":"Urgent"});

    client
        .task_v2()
        .custom_field
        .create_by_query(
            &CreateCustomFieldV2Query::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .custom_field
        .patch_by_query(
            &PatchCustomFieldV2Query::new("field-1", &patch_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .custom_field
        .add_by_query(
            &AddCustomFieldV2Query::new("field-1", &bind_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .custom_field
        .remove_by_query(
            &RemoveCustomFieldV2Query::new("field-1", &bind_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .custom_field
        .create_option_by_query(
            &CreateCustomFieldOptionV2Query::new("field-1", &option_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .custom_field
        .patch_option_by_query(
            &PatchCustomFieldOptionV2Query::new("field-1", "option-1", &option_patch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/custom_fields?"));
    assert!(request.contains("PATCH /open-apis/task/v2/custom_fields/field-1?"));
    assert!(request.contains("POST /open-apis/task/v2/custom_fields/field-1/add "));
    assert!(request.contains("POST /open-apis/task/v2/custom_fields/field-1/remove "));
    assert!(request.contains("POST /open-apis/task/v2/custom_fields/field-1/options "));
    assert!(request.contains("PATCH /open-apis/task/v2/custom_fields/field-1/options/option-1 "));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""name":"Priority""#));
    assert!(request.contains(r#""name":"Priority updated""#));
    assert!(request.contains(r#""resource_type":"tasklist""#));
    assert!(request.contains(r#""name":"High""#));
    assert!(request.contains(r#""name":"Urgent""#));
}

#[tokio::test]
async fn task_v2_section_by_query_smoke() {
    let section_body =
        r#"{"code":0,"msg":"ok","data":{"section":{"guid":"section-1","name":"Backlog"}}}"#;
    let section_list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"section-1"}],"has_more":false}}"#;
    let task_list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"task-guid-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, section_body),
        http_response(200, section_body),
        http_response(200, section_body),
        http_response(200, section_list_body),
        http_response(200, task_list_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"name":"Backlog"});
    let patch_body = serde_json::json!({"name":"Doing"});
    let task_params = TaskListParams::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .completed(false)
        .created_from("2026-01-01")
        .created_to("2026-01-31")
        .user_id_type("open_id");

    client
        .task_v2()
        .section
        .create_by_query(
            &CreateSectionV2Query::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .section
        .get_by_query(
            &GetSectionV2Query::new("section-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .section
        .patch_by_query(
            &PatchSectionV2Query::new("section-1", &patch_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .section
        .list_by_query(
            &ListSectionV2Query::new()
                .resource_type("tasklist")
                .resource_id("tasklist-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .section
        .tasks_by_query(
            &TasksSectionV2Query::new("section-1").params(task_params),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/sections?"));
    assert!(request.contains("GET /open-apis/task/v2/sections/section-1?"));
    assert!(request.contains("PATCH /open-apis/task/v2/sections/section-1?"));
    assert!(request.contains("GET /open-apis/task/v2/sections?"));
    assert!(request.contains("GET /open-apis/task/v2/sections/section-1/tasks?"));
    assert!(request.contains("resource_type=tasklist"));
    assert!(request.contains("resource_id=tasklist-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("completed=false"));
    assert!(request.contains("created_from=2026-01-01"));
    assert!(request.contains("created_to=2026-01-31"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""name":"Backlog""#));
    assert!(request.contains(r#""name":"Doing""#));
}

#[tokio::test]
async fn task_v2_tasklist_by_query_smoke() {
    let tasklist_body =
        r#"{"code":0,"msg":"ok","data":{"tasklist":{"guid":"tasklist-1","name":"Roadmap"}}}"#;
    let tasklist_list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"tasklist-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, tasklist_body),
        http_response(200, tasklist_body),
        http_response(200, tasklist_body),
        http_response(200, tasklist_list_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"name":"Roadmap"});
    let patch_body = serde_json::json!({"name":"Roadmap updated"});

    client
        .task_v2()
        .tasklist
        .create_by_query(
            &CreateTasklistV2Query::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .get_by_query(
            &GetTasklistV2Query::new("tasklist-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .patch_by_query(
            &PatchTasklistV2Query::new("tasklist-1", &patch_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .list_by_query(
            &ListTasklistV2Query::new()
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/tasklists?"));
    assert!(request.contains("GET /open-apis/task/v2/tasklists/tasklist-1?"));
    assert!(request.contains("PATCH /open-apis/task/v2/tasklists/tasklist-1?"));
    assert!(request.contains("GET /open-apis/task/v2/tasklists?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""name":"Roadmap""#));
    assert!(request.contains(r#""name":"Roadmap updated""#));
}

#[tokio::test]
async fn task_v2_tasklist_members_and_tasks_by_query_smoke() {
    let tasklist_body =
        r#"{"code":0,"msg":"ok","data":{"tasklist":{"guid":"tasklist-1","name":"Roadmap"}}}"#;
    let task_list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"task-guid-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, tasklist_body),
        http_response(200, tasklist_body),
        http_response(200, task_list_body),
    ])
    .await;

    let client = client_for(addr);
    let members_body = serde_json::json!({"members":["u-1"]});
    let task_params = TaskListParams::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .completed(false)
        .created_from("2026-01-01")
        .created_to("2026-01-31")
        .user_id_type("open_id");

    client
        .task_v2()
        .tasklist
        .add_members_by_query(
            &AddMembersTasklistV2Query::new("tasklist-1", &members_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .remove_members_by_query(
            &RemoveMembersTasklistV2Query::new("tasklist-1", &members_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .tasks_by_query(
            &TasksTasklistV2Query::new("tasklist-1").params(task_params),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/tasklists/tasklist-1/add_members?"));
    assert!(request.contains("POST /open-apis/task/v2/tasklists/tasklist-1/remove_members?"));
    assert!(request.contains("GET /open-apis/task/v2/tasklists/tasklist-1/tasks?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("completed=false"));
    assert!(request.contains("created_from=2026-01-01"));
    assert!(request.contains("created_to=2026-01-31"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""members":["u-1"]"#));
}

#[tokio::test]
async fn task_v2_tasklist_activity_subscription_by_query_smoke() {
    let activity_body = r#"{"code":0,"msg":"ok","data":{"activity_subscription":{"guid":"activity-1","name":"Changes"}}}"#;
    let activity_list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"activity-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, activity_body),
        http_response(200, activity_body),
        http_response(200, activity_body),
        http_response(200, activity_list_body),
    ])
    .await;

    let client = client_for(addr);
    let activity_body = serde_json::json!({"event":"task_changed"});
    let activity_patch_body = serde_json::json!({"event":"comment_changed"});

    client
        .task_v2()
        .tasklist
        .create_activity_subscription_by_query(
            &CreateActivitySubscriptionV2Query::new("tasklist-1", &activity_body)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .get_activity_subscription_by_query(
            &GetActivitySubscriptionV2Query::new("tasklist-1", "activity-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .patch_activity_subscription_by_query(
            &PatchActivitySubscriptionV2Query::new(
                "tasklist-1",
                "activity-1",
                &activity_patch_body,
            )
            .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .list_activity_subscriptions_by_query(
            &ListActivitySubscriptionV2Query::new("tasklist-1")
                .limit(50)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("POST /open-apis/task/v2/tasklists/tasklist-1/activity_subscriptions?")
    );
    assert!(request.contains(
        "GET /open-apis/task/v2/tasklists/tasklist-1/activity_subscriptions/activity-1?"
    ));
    assert!(request.contains(
        "PATCH /open-apis/task/v2/tasklists/tasklist-1/activity_subscriptions/activity-1?"
    ));
    assert!(
        request.contains("GET /open-apis/task/v2/tasklists/tasklist-1/activity_subscriptions?")
    );
    assert!(request.contains("limit=50"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""event":"task_changed""#));
    assert!(request.contains(r#""event":"comment_changed""#));
}
