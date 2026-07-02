use super::*;

// ── Application v1 ──

#[tokio::test]
async fn application_v1_by_query_smoke() {
    let app_body = r#"{"code":0,"msg":"ok","data":{"app":{"app_id":"cli_a","app_name":"Demo"}}}"#;
    let app_list_body =
        r#"{"code":0,"msg":"ok","data":{"app_list":[{"app_id":"cli_a","app_name":"Demo"}]}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let blacklist_body =
        r#"{"code":0,"msg":"ok","data":{"user_list":["ou-1"],"department_list":["od-1"]}}"#;
    let version_body =
        r#"{"code":0,"msg":"ok","data":{"app_version":{"version_id":"ver-1","app_id":"cli_a"}}}"#;
    let version_list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"version_id":"ver-1","app_id":"cli_a"}]}}"#;
    let usage_body = r#"{"code":0,"msg":"ok","data":{"overview":{"activate_users":3}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, app_body),
        http_response(200, app_list_body),
        http_response(200, empty_body),
        http_response(200, blacklist_body),
        http_response(200, version_body),
        http_response(200, version_list_body),
        http_response(200, usage_body),
        http_response(200, usage_body),
    ])
    .await;

    let client = client_for(addr);
    let blacklist_req = CheckUserIsInAppBlacklistReqBody {
        user_list: Some(vec!["ou-1".to_string()]),
        department_list: Some(vec!["od-1".to_string()]),
    };

    let app_resp = client
        .application()
        .app
        .get_by_query(
            &GetApplicationV1AppQuery::new("cli_a").lang("zh_cn"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let app_list_resp = client
        .application()
        .app
        .list_by_query(
            &ListApplicationV1AppQuery::new()
                .page_size(20)
                .page_token("next-page")
                .lang("zh_cn"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let admin_resp = client
        .application()
        .app
        .check_user_admin_by_query(
            &CheckApplicationV1UserAdminQuery::new("cli_a", "ou-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let blacklist_resp = client
        .application()
        .app
        .check_user_in_blacklist_by_query(
            &CheckApplicationV1BlacklistQuery::new("cli_a", &blacklist_req)
                .user_id_type("open_id")
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let version_resp = client
        .application()
        .app_version
        .get_by_query(
            &GetApplicationV1AppVersionQuery::new("cli_a", "ver-1", "zh_cn"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let version_list_resp = client
        .application()
        .app_version
        .list_by_query(
            &ListApplicationV1AppVersionQuery::new("cli_a", "zh_cn")
                .page_size(20)
                .page_token("next-page")
                .order(1),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let department_usage_resp = client
        .application()
        .app_usage
        .department_overview_by_query(
            &DepartmentOverviewAppUsageQuery::new("cli_a", "2026-06-01", 1)
                .department_id("od-1")
                .recursion(1)
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let usage_resp = client
        .application()
        .app_usage
        .overview_by_query(
            &OverviewAppUsageQuery::new("cli_a", "2026-06-01", 1),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(app_resp.success());
    assert!(app_list_resp.success());
    assert!(admin_resp.success());
    assert!(blacklist_resp.success());
    assert!(version_resp.success());
    assert!(version_list_resp.success());
    assert!(department_usage_resp.success());
    assert!(usage_resp.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications/cli_a?"));
    assert!(request.contains("GET /open-apis/application/v6/applications?"));
    assert!(
        request
            .contains("GET /open-apis/application/v6/applications/cli_a/management/check_admin?")
    );
    assert!(request.contains(
        "POST /open-apis/application/v6/applications/cli_a/visibility/check_white_black_list?"
    ));
    assert!(
        request.contains("GET /open-apis/application/v6/applications/cli_a/app_versions/ver-1?")
    );
    assert!(request.contains("GET /open-apis/application/v6/applications/cli_a/app_versions?"));
    assert!(request.contains(
        "GET /open-apis/application/v6/applications/cli_a/app_usage/department_overview?"
    ));
    assert!(
        request.contains("GET /open-apis/application/v6/applications/cli_a/app_usage/overview?")
    );
    assert!(request.contains("lang=zh_cn"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("order=1"));
    assert!(request.contains("date=2026-06-01"));
    assert!(request.contains("cycle_type=1"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("recursion=1"));
    assert!(request.contains(r#""user_list":["ou-1"]"#));
    assert!(request.contains(r#""department_list":["od-1"]"#));
}
