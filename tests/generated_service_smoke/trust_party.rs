use super::*;

// ── Trust Party v1 ──

#[tokio::test]
async fn trust_party_collaboration_tenant_get_and_list_by_query_smoke() {
    let get_body = r#"{"code":0,"msg":"ok","data":{"target_tenant":{"tenant_key":"tenant-1"}}}"#;
    let list_body = r#"{"code":0,"msg":"ok","data":{"target_tenant_list":[{"tenant_key":"tenant-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, get_body),
        http_response(200, list_body),
    ])
    .await;

    let client = client_for(addr);
    let get = client
        .trust_party()
        .collaboration_tenant
        .get_by_query(
            &GetCollaborationTenantQuery::new("tenant-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let list = client
        .trust_party()
        .collaboration_tenant
        .list_by_query(
            &ListTrustPartyCollaborationTenantQuery::new()
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(get.success());
    assert!(list.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/trust_party/v1/collaboration_tenants/tenant-1 "));
    assert!(request.contains("GET /open-apis/trust_party/v1/collaboration_tenants?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn trust_party_visible_organization_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"collaboration_entity_list":[{"id":"dept-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .trust_party()
        .collaboration_tenant
        .visible_organization_by_query(
            &VisibleOrganizationCollaborationTenantQuery::new("tenant-1")
                .department_id_type("open_department_id")
                .target_department_id("od-1")
                .group_id_type("open_group_id")
                .target_group_id("ug-1")
                .page_size(50)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains(
        "GET /open-apis/trust_party/v1/collaboration_tenants/tenant-1/visible_organization?"
    ));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("target_department_id=od-1"));
    assert!(request.contains("group_id_type=open_group_id"));
    assert!(request.contains("target_group_id=ug-1"));
    assert!(request.contains("page_size=50"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn trust_party_collaboration_department_and_user_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let department = client
        .trust_party()
        .collaboration_tenant_collaboration_department
        .get_by_query(
            &GetCollaborationTenantCollaborationDepartmentQuery::new("tenant-1", "dept-1")
                .target_department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let user = client
        .trust_party()
        .collaboration_tenant_collaboration_user
        .get_by_query(
            &GetCollaborationTenantCollaborationUserQuery::new("tenant-1", "user-1")
                .target_user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(department.success());
    assert!(user.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains(
        "GET /open-apis/trust_party/v1/collaboration_tenants/tenant-1/collaboration_departments/dept-1?"
    ));
    assert!(request.contains("target_department_id_type=open_department_id"));
    assert!(request.contains(
        "GET /open-apis/trust_party/v1/collaboration_tenants/tenant-1/collaboration_users/user-1?"
    ));
    assert!(request.contains("target_user_id_type=open_id"));
}
