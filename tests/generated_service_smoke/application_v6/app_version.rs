use super::prelude::*;

// ── Application v6 ──

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
    let body = r#"{"code":0,"msg":"ok"}"#;
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
