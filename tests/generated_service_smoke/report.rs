use super::*;

// ── Report ──

#[tokio::test]
async fn report_rule_query_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rules":[{"rule_id":"rule-1","name":"Weekly"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .report()
        .rule
        .query(
            Some("Weekly"),
            Some(1),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/report/v1/rules/query?"));
    assert!(request.contains("rule_name=Weekly"));
    assert!(request.contains("include_deleted=1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn report_rule_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rules":[{"rule_id":"rule-1","name":"Weekly"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = QueryReportRuleQuery::new()
        .rule_name("Weekly")
        .include_deleted(1)
        .user_id_type("open_id");
    let resp = client
        .report()
        .rule
        .query_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.rules.len(), 1);
    assert_eq!(data.rules[0].rule_id.as_deref(), Some("rule-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/report/v1/rules/query?"));
    assert!(request.contains("rule_name=Weekly"));
    assert!(request.contains("include_deleted=1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn report_task_query_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"task_id":"task-1","rule_id":"rule-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let req_body = QueryReportTaskReqBody {
        rule_id: Some("rule-1".into()),
        user_id: Some("u-1".into()),
        from_time: Some("1700000000".into()),
        to_time: Some("1700000100".into()),
        page_size: Some(20),
        page_token: Some("next-page".into()),
    };
    let resp = client
        .report()
        .task
        .query(&req_body, Some("open_id"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/report/v1/tasks/query?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""rule_id":"rule-1""#));
    assert!(request.contains(r#""user_id":"u-1""#));
    assert!(request.contains(r#""page_size":20"#));
    assert!(request.contains(r#""page_token":"next-page""#));
}

#[tokio::test]
async fn report_task_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"task_id":"task-1","rule_id":"rule-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let req_body = QueryReportTaskReqBody {
        rule_id: Some("rule-1".into()),
        user_id: Some("u-1".into()),
        from_time: Some("1700000000".into()),
        to_time: Some("1700000100".into()),
        page_size: Some(20),
        page_token: Some("next-page".into()),
    };
    let query = QueryReportTaskQuery::new(&req_body).user_id_type("open_id");
    let resp = client
        .report()
        .task
        .query_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].task_id.as_deref(), Some("task-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/report/v1/tasks/query?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""rule_id":"rule-1""#));
    assert!(request.contains(r#""user_id":"u-1""#));
    assert!(request.contains(r#""page_size":20"#));
    assert!(request.contains(r#""page_token":"next-page""#));
}

#[tokio::test]
async fn report_rule_view_remove_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let req_body = RemoveRuleViewReqBody {
        user_ids: Some(vec!["u-1".into(), "u-2".into()]),
    };
    let resp = client
        .report()
        .rule_view
        .remove_by_query(
            &RemoveRuleViewQuery::new("rule-1", &req_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/report/v1/rules/rule-1/views/remove?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""user_ids":["u-1","u-2"]"#));
}
