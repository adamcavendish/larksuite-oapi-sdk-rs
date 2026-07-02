use super::prelude::*;

// ── Bitable ──

#[tokio::test]
async fn bitable_role_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"role_id":"role-1","role_name":"Editor"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListAppRoleQuery::new("app-token-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .bitable()
        .role
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].role_name.as_deref(), Some("Editor"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/roles?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn bitable_role_member_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id":"member-1","member_name":"Ada"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListAppRoleMemberQuery::new("app-token-1", "role-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .bitable()
        .role_member
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].member_name.as_deref(), Some("Ada"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/roles/role-1/members?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn bitable_role_member_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id":"member-1","member_name":"Ada"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .bitable()
        .role_member
        .list(
            "app-token-1",
            "role-1",
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/roles/role-1/members?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
