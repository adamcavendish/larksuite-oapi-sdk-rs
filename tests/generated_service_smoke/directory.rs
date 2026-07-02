use super::prelude::*;

// ── Directory ──

#[tokio::test]
async fn directory_user_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"u-1","name":"Alice"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListDirectoryUserQuery::new()
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(50).page_token("next-page"));
    let resp = client
        .directory()
        .user
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].name.as_deref(), Some("Alice"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/directory/v1/users?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=50"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn directory_user_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .directory()
        .user
        .list(
            Some("open_id"),
            Some(50),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/directory/v1/users?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=50"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn directory_collaboration_rule_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCollaborationRuleQuery::new()
        .target_tenant_key("target-tenant")
        .tenant_id("tenant-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .directory()
        .collaboration_rule
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/directory/v1/collaboration_rules?"));
    assert!(request.contains("target_tenant_key=target-tenant"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn directory_collaboration_rule_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .directory()
        .collaboration_rule
        .list(
            Some("target-tenant"),
            Some("tenant-1"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/directory/v1/collaboration_rules?"));
    assert!(request.contains("target_tenant_key=target-tenant"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn directory_collaboration_rule_write_by_query_smoke() {
    let create_body = r#"{"code":0,"msg":"ok","data":{"collaboration_rule_id":"rule-1"}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, create_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let create_payload = serde_json::json!({"name":"Rule"});
    let update_payload = serde_json::json!({"name":"Rule updated"});
    let create_resp = client
        .directory()
        .collaboration_rule
        .create_by_query(
            &CreateCollaborationRuleQuery::new(&create_payload)
                .target_tenant_key("target-tenant")
                .tenant_id("tenant-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete_resp = client
        .directory()
        .collaboration_rule
        .delete_by_query(
            &DeleteCollaborationRuleQuery::new("rule-1")
                .target_tenant_key("target-tenant")
                .tenant_id("tenant-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_resp = client
        .directory()
        .collaboration_rule
        .update_by_query(
            &UpdateCollaborationRuleQuery::new("rule-1", &update_payload)
                .target_tenant_key("target-tenant")
                .tenant_id("tenant-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(delete_resp.success());
    assert!(update_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/directory/v1/collaboration_rules?"));
    assert!(request.contains("DELETE /open-apis/directory/v1/collaboration_rules/rule-1?"));
    assert!(request.contains("PUT /open-apis/directory/v1/collaboration_rules/rule-1?"));
    assert!(request.contains("target_tenant_key=target-tenant"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains(r#""name":"Rule""#));
    assert!(request.contains(r#""name":"Rule updated""#));
}

#[tokio::test]
async fn directory_collaboration_tenant_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCollaborationTenantQuery::new()
        .tenant_id("tenant-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .directory()
        .collaboration_tenant
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/directory/v1/collaboration_tenants?"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn directory_collaboration_tenant_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .directory()
        .collaboration_tenant
        .list(
            Some("tenant-1"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/directory/v1/collaboration_tenants?"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn directory_share_entity_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListShareEntityQuery::new()
        .target_tenant_key("target-tenant")
        .target_department_id("department-1")
        .target_group_id("group-1")
        .is_select_subject(true)
        .tenant_id("tenant-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .directory()
        .share_entity
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/directory/v1/share_entities?"));
    assert!(request.contains("target_tenant_key=target-tenant"));
    assert!(request.contains("target_department_id=department-1"));
    assert!(request.contains("target_group_id=group-1"));
    assert!(request.contains("is_select_subject=true"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn directory_share_entity_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .directory()
        .share_entity
        .list(
            Some("target-tenant"),
            Some("department-1"),
            Some("group-1"),
            Some(true),
            Some("tenant-1"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/directory/v1/share_entities?"));
    assert!(request.contains("target_tenant_key=target-tenant"));
    assert!(request.contains("target_department_id=department-1"));
    assert!(request.contains("target_group_id=group-1"));
    assert!(request.contains("is_select_subject=true"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn directory_department_create_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"department_id":"od-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"name":"Engineering"});
    let resp = client
        .directory()
        .department
        .create_by_query(
            &CreateDirectoryDepartmentQuery::new(&body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true)
                .tenant_id("tenant-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/directory/v1/departments?"));
    assert!(request.contains("employee_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("is_admin_role=true"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains(r#""name":"Engineering""#));
}

#[tokio::test]
async fn directory_department_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"keyword":"Engineering"});
    let resp = client
        .directory()
        .department
        .search_by_query(
            &SearchDirectoryDepartmentQuery::new(&body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true)
                .tenant_id("tenant-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/directory/v1/departments/search?"));
    assert!(request.contains("employee_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("is_admin_role=true"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains(r#""keyword":"Engineering""#));
}

#[tokio::test]
async fn directory_department_delete_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .directory()
        .department
        .delete_by_query(
            &DeleteDepartmentQuery::new("od-1")
                .is_admin_role(true)
                .employee_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("DELETE /open-apis/directory/v1/departments/od-1?"));
    assert!(request.contains("is_admin_role=true"));
    assert!(request.contains("employee_id_type=open_id"));
}

#[tokio::test]
async fn directory_employee_create_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"employee_id":"emp-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"name":"Alice"});
    let resp = client
        .directory()
        .employee
        .create_by_query(
            &CreateDirectoryEmployeeQuery::new(&body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true)
                .tenant_id("tenant-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/directory/v1/employees?"));
    assert!(request.contains("employee_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("is_admin_role=true"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains(r#""name":"Alice""#));
}

#[tokio::test]
async fn directory_employee_mget_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"employee_id":"emp-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"employee_ids":["emp-1"]});
    let resp = client
        .directory()
        .employee
        .mget_by_query(
            &MgetDirectoryEmployeeQuery::new(&body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/directory/v1/employees/mget?"));
    assert!(request.contains("employee_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("is_admin_role=true"));
    assert!(request.contains(r#""employee_ids":["emp-1"]"#));
}

#[tokio::test]
async fn directory_employee_patch_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"name":"Alice Updated"});
    let resp = client
        .directory()
        .employee
        .patch_by_query(
            &PatchDirectoryEmployeeQuery::new("emp-1", &body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/directory/v1/employees/emp-1?"));
    assert!(request.contains("employee_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("is_admin_role=true"));
    assert!(request.contains(r#""name":"Alice Updated""#));
}

#[tokio::test]
async fn directory_employee_action_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let delete_body = serde_json::json!({"reason":"left"});
    let regular_body = serde_json::json!({"effective_date":"2026-06-01"});
    let resurrect_body = serde_json::json!({"reason":"return"});
    let resigned_body = serde_json::json!({"resign_date":"2026-06-30"});
    let delete_resp = client
        .directory()
        .employee
        .delete_by_query(
            &DeleteDirectoryEmployeeQuery::new("emp-1")
                .body(&delete_body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let regular_resp = client
        .directory()
        .employee
        .regular_by_query(
            &RegularDirectoryEmployeeQuery::new("emp-1")
                .body(&regular_body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let resurrect_resp = client
        .directory()
        .employee
        .resurrect_by_query(
            &ResurrectDirectoryEmployeeQuery::new("emp-1")
                .body(&resurrect_body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let resigned_resp = client
        .directory()
        .employee
        .to_be_resigned_by_query(
            &ToBeResignedDirectoryEmployeeQuery::new("emp-1", &resigned_body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(delete_resp.success());
    assert!(regular_resp.success());
    assert!(resurrect_resp.success());
    assert!(resigned_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("DELETE /open-apis/directory/v1/employees/emp-1?"));
    assert!(request.contains("PATCH /open-apis/directory/v1/employees/emp-1/regular?"));
    assert!(request.contains("POST /open-apis/directory/v1/employees/emp-1/resurrect?"));
    assert!(request.contains("PATCH /open-apis/directory/v1/employees/emp-1/to_be_resigned?"));
    assert!(request.contains("employee_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("is_admin_role=true"));
    assert!(request.contains(r#""reason":"left""#));
    assert!(request.contains(r#""effective_date":"2026-06-01""#));
    assert!(request.contains(r#""reason":"return""#));
    assert!(request.contains(r#""resign_date":"2026-06-30""#));
}
