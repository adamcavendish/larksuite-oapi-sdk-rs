use super::prelude::*;

// Task v2 tasklist smoke tests

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
