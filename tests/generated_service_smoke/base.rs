use super::prelude::*;

// ── Base ──

#[tokio::test]
async fn base_v2_app_role_by_query_smoke() {
    let role_body = r#"{"code":0,"msg":"ok","data":{"role":{"role_id":"role-1","name":"Admin"}}}"#;
    let list_body = r#"{"code":0,"msg":"ok","data":{"items":[{"role_id":"role-1","name":"Admin"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, role_body),
        http_response(200, list_body),
        http_response(200, role_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"name":"Admin"});
    let update_body = serde_json::json!({"name":"Admin updated"});

    client
        .base_v2()
        .app_role
        .create_by_query(
            &CreateAppRoleV2Query::new("app-token-1", &create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .base_v2()
        .app_role
        .list_by_query(
            &ListAppRoleV2Query::new("app-token-1")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .base_v2()
        .app_role
        .update_by_query(
            &UpdateAppRoleV2Query::new("app-token-1", "role-1", &update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/base/v2/apps/app-token-1/roles "));
    assert!(request.contains("GET /open-apis/base/v2/apps/app-token-1/roles?"));
    assert!(request.contains("PUT /open-apis/base/v2/apps/app-token-1/roles/role-1 "));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""name":"Admin""#));
    assert!(request.contains(r#""name":"Admin updated""#));
}
