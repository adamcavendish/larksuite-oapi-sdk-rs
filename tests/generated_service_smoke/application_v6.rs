use super::*;

// ── Application v6 ──

#[tokio::test]
async fn application_v6_utility_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let badge_body = serde_json::json!({"user_id":"ou-1","badge_count":3});
    client
        .application_v6()
        .app_badge
        .set_by_query(
            &SetAppBadgeQuery::new(&badge_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .application_v6()
        .scope
        .apply_by_query(&ApplyScopeQuery::new(), &RequestOption::default())
        .await
        .unwrap();
    client
        .application_v6()
        .scope
        .list_by_query(
            &ListApplicationV6ScopeQuery::new(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/application/v6/app_badge/set?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""badge_count":3"#));
    assert!(request.contains("POST /open-apis/application/v6/scopes/apply "));
    assert!(request.contains("GET /open-apis/application/v6/scopes "));
}

#[tokio::test]
async fn application_v6_app_recommend_rule_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"rule-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .app_recommend_rule
        .list_by_query(
            &ListAppRecommendRuleQuery::new()
                .page_size(20)
                .page_token("next-page")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.get("items"))
            .and_then(|items| items.as_array())
            .map(Vec::len),
        Some(1)
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/app_recommend_rules?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

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
            .and_then(|data| data.get("app"))
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
    let body = r#"{"code":0,"msg":"ok","data":{"app":{"app_id":"cli_a"}}}"#;
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

#[tokio::test]
async fn application_v6_app_version_contacts_range_suggest_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"scope-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_app_version
        .contacts_range_suggest_by_query(
            &ContactsRangeSuggestApplicationAppVersionQuery::new("cli_a", "ver-1")
                .department_id_type("open_department_id")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains(
        "GET /open-apis/application/v6/applications/cli_a/app_versions/ver-1/contacts_range_suggest?"
    ));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_app_version_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"app_version":{"version_id":"ver-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_app_version
        .get_by_query(
            &GetApplicationV6AppVersionQuery::new("cli_a", "ver-1")
                .lang("zh_cn")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("GET /open-apis/application/v6/applications/cli_a/app_versions/ver-1?")
    );
    assert!(request.contains("lang=zh_cn"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_app_version_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"version_id":"ver-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_app_version
        .list_by_query(
            &ListApplicationAppVersionQuery::new("cli_a")
                .lang("zh_cn")
                .page_size(20)
                .page_token("next-page")
                .order(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications/cli_a/app_versions?"));
    assert!(request.contains("lang=zh_cn"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("order=1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_app_version_patch_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"app_version":{"version_id":"ver-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let patch_body = serde_json::json!({"status":2});
    let resp = client
        .application_v6()
        .application_app_version
        .patch_by_query(
            &PatchApplicationAppVersionQuery::new("cli_a", "ver-1", &patch_body)
                .user_id_type("open_id")
                .operator_id("ou-1")
                .reject_reason("missing scope"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("PATCH /open-apis/application/v6/applications/cli_a/app_versions/ver-1?")
    );
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("operator_id=ou-1"));
    assert!(request.contains("reject_reason=missing+scope"));
    assert!(request.contains(r#""status":2"#));
}

#[tokio::test]
async fn application_v6_collaborators_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_collaborators
        .get_by_query(
            &GetApplicationCollaboratorsQuery::new("cli_a").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications/cli_a/collaborators?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_collaborators_and_contacts_range_write_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let collaborators_body = serde_json::json!({"collaborators":["ou-1"]});
    let contacts_body = serde_json::json!({"visible_users":["ou-1"]});
    client
        .application_v6()
        .application_collaborators
        .update_by_query(
            &UpdateApplicationCollaboratorsQuery::new("cli_a", &collaborators_body)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .application_v6()
        .application_contacts_range
        .patch_by_query(
            &PatchApplicationContactsRangeQuery::new("cli_a", &contacts_body)
                .user_id_type("open_id")
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PUT /open-apis/application/v6/applications/cli_a/collaborators?"));
    assert!(request.contains("PATCH /open-apis/application/v6/applications/cli_a/contacts_range?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains(r#""collaborators":["ou-1"]"#));
    assert!(request.contains(r#""visible_users":["ou-1"]"#));
}

#[tokio::test]
async fn application_v6_feedback_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"feedback_id":"fb-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_feedback
        .list_by_query(
            &ListApplicationFeedbackQuery::new("cli_a")
                .from_date("2026-06-01")
                .to_date("2026-06-30")
                .feedback_type(1)
                .status(2)
                .user_id_type("open_id")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications/cli_a/feedbacks?"));
    assert!(request.contains("from_date=2026-06-01"));
    assert!(request.contains("to_date=2026-06-30"));
    assert!(request.contains("feedback_type=1"));
    assert!(request.contains("status=2"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn application_v6_feedback_patch_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"feedback_id":"fb-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_feedback
        .patch_by_query(
            &PatchApplicationFeedbackQuery::new("cli_a", "fb-1")
                .user_id_type("open_id")
                .status(2)
                .operator_id("ou-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/application/v6/applications/cli_a/feedbacks/fb-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("status=2"));
    assert!(request.contains("operator_id=ou-1"));
}

#[tokio::test]
async fn application_v6_management_and_owner_write_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let management_body = serde_json::json!({"enabled":true});
    let owner_body = serde_json::json!({"owner_id":"ou-1"});
    client
        .application_v6()
        .application_management
        .update_by_query(
            &UpdateApplicationManagementQuery::new("cli_a", &management_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .application_v6()
        .application_owner
        .update_by_query(
            &UpdateApplicationOwnerQuery::new("cli_a", &owner_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PUT /open-apis/application/v6/applications/cli_a/management "));
    assert!(request.contains("PUT /open-apis/application/v6/applications/cli_a/owner?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""enabled":true"#));
    assert!(request.contains(r#""owner_id":"ou-1""#));
}

#[tokio::test]
async fn application_v6_visibility_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let check_body = serde_json::json!({"users":["ou-1"]});
    let patch_body = serde_json::json!({"visible":true});
    client
        .application_v6()
        .application_visibility
        .check_white_black_list_by_query(
            &CheckWhiteBlackListApplicationVisibilityQuery::new("cli_a", &check_body)
                .user_id_type("open_id")
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .application_v6()
        .application_visibility
        .patch_by_query(
            &PatchApplicationVisibilityQuery::new("cli_a", &patch_body)
                .department_id_type("open_department_id")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains(
        "POST /open-apis/application/v6/applications/cli_a/visibility/check_white_black_list?"
    ));
    assert!(request.contains("PATCH /open-apis/application/v6/applications/cli_a/visibility?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains(r#""users":["ou-1"]"#));
    assert!(request.contains(r#""visible":true"#));
}
