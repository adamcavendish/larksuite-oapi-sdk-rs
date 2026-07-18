use super::prelude::*;

// ── Base ──

#[tokio::test]
async fn base_v2_app_role_by_query_smoke() {
    let role_body = r#"{"code":0,"msg":"ok","data":{"role":{"role_id":"role-1","role_name":"Admin","table_roles":[{"table_id":"tbl-1","rec_rule":{"conditions":[{"field_name":"status","value":["open"]}]}}],"block_roles":[{"block_id":"blk-1","block_perm":2}]}}}"#;
    let list_body = r#"{"code":0,"msg":"ok","data":{"items":[{"role_id":"role-1","role_name":"Admin"}],"has_more":false,"total":1}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, role_body),
        http_response(200, list_body),
        http_response(200, role_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = json_value!({"name":"Admin"});
    let update_body = json_value!({"name":"Admin updated"});

    let create_resp = client
        .base_v2()
        .app_role
        .create_by_query(
            &CreateAppRoleV2Query::new("app-token-1", &create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let list_resp = client
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

    let role = create_resp.data.unwrap().role.unwrap();
    assert_eq!(role.role_name.as_deref(), Some("Admin"));
    assert_eq!(
        role.table_roles.unwrap()[0].table_id.as_deref(),
        Some("tbl-1")
    );
    assert_eq!(role.block_roles.unwrap()[0].block_perm, Some(2));
    let list_data = list_resp.data.unwrap();
    assert_eq!(list_data.items[0].role_id.as_deref(), Some("role-1"));
    assert_eq!(list_data.total, Some(1));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/base/v2/apps/app-token-1/roles "));
    assert!(request.contains("GET /open-apis/base/v2/apps/app-token-1/roles?"));
    assert!(request.contains("PUT /open-apis/base/v2/apps/app-token-1/roles/role-1 "));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""name":"Admin""#));
    assert!(request.contains(r#""name":"Admin updated""#));
}
