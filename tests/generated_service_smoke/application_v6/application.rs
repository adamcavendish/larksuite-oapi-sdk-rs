use super::prelude::*;

// ── Application v6 ──

#[tokio::test]
async fn application_v6_contacts_range_configuration_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"scope-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application
        .contacts_range_configuration_by_query(
            &ApplicationContactsRangeConfigurationQuery::new("cli_a")
                .page_size(50)
                .page_token("next-page")
                .department_id_type("open_department_id")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains(
        "GET /open-apis/application/v6/applications/cli_a/contacts_range_configuration?"
    ));
    assert!(request.contains("page_size=50"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"app":{"app_id":"cli_a"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application
        .get_by_query(
            &GetApplicationV6Query::new("cli_a")
                .lang("zh_cn")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.app.as_ref())
            .and_then(|app| app.get("app_id"))
            .and_then(|app_id| app_id.as_str()),
        Some("cli_a")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications/cli_a?"));
    assert!(request.contains("lang=zh_cn"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"app_id":"cli_a"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application
        .list_by_query(
            &ListApplicationV6Query::new()
                .page_size(20)
                .page_token("next-page")
                .user_id_type("open_id")
                .lang("zh_cn")
                .status(1)
                .payment_type(2)
                .owner_type(3),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("lang=zh_cn"));
    assert!(request.contains("status=1"));
    assert!(request.contains("payment_type=2"));
    assert!(request.contains("owner_type=3"));
}

#[tokio::test]
async fn application_v6_patch_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let patch_body = serde_json::json!({"name":"Updated App"});
    let resp = client
        .application_v6()
        .application
        .patch_by_query(
            &PatchApplicationQuery::new("cli_a", &patch_body).lang("zh_cn"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/application/v6/applications/cli_a?"));
    assert!(request.contains("lang=zh_cn"));
    assert!(request.contains(r#""name":"Updated App""#));
}

#[tokio::test]
async fn application_v6_underauditlist_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"app_id":"cli_a"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application
        .underauditlist_by_query(
            &UnderauditlistApplicationQuery::new()
                .lang("zh_cn")
                .page_size(20)
                .page_token("next-page")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications/underauditlist?"));
    assert!(request.contains("lang=zh_cn"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_app_usage_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let usage_body = serde_json::json!({"date":"2026-06-01"});
    client
        .application_v6()
        .application_app_usage
        .department_overview_by_query(
            &DepartmentOverviewApplicationAppUsageQuery::new("cli_a", &usage_body)
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .application_v6()
        .application_app_usage
        .message_push_overview_by_query(
            &MessagePushOverviewApplicationAppUsageQuery::new("cli_a", &usage_body)
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .application_v6()
        .application_app_usage
        .overview_by_query(
            &OverviewApplicationAppUsageQuery::new("cli_a", &usage_body)
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains(
        "POST /open-apis/application/v6/applications/cli_a/app_usage/department_overview?"
    ));
    assert!(request.contains(
        "POST /open-apis/application/v6/applications/cli_a/app_usage/message_push_overview?"
    ));
    assert!(
        request.contains("POST /open-apis/application/v6/applications/cli_a/app_usage/overview?")
    );
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains(r#""date":"2026-06-01""#));
}
