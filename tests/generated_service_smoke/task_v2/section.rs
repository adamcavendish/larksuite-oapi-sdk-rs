use super::prelude::*;

// Task v2 section smoke tests

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
