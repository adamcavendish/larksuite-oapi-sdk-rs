use super::prelude::*;

// ── Task ──

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
