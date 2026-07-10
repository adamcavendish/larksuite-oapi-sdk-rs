use super::prelude::*;

#[tokio::test]
async fn hire_reference_list_by_query_smoke() {
    let subject_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"subject-1","name":{"en_us":"Subject"}}],"page_token":"next-page","has_more":false}}"#;
    let folder_body = r#"{"code":0,"msg":"ok","data":{"items":[{"folder_id":"folder-1","folder_name":"Pool"}],"page_token":"next-page","has_more":false}}"#;
    let i18n_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"item-1","name":{"zh_cn":"名称","en_us":"Name"}}],"page_token":"next-page","has_more":false}}"#;
    let todo_body = r#"{"code":0,"msg":"ok","data":{"items":[{"evaluation":{"id":"eval-1"}}],"page_token":"next-page","has_more":false}}"#;
    let role_get_body = r#"{"code":0,"msg":"ok","data":{"role":{"id":"role-1","socail_permission_collection":{"data_permissions":[{"id":"permission-1","select_status":1}]}}}}"#;
    let role_body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou_user_1","role_id":"role-1","business_management_scopes":[{"entity":{"code":"application"},"scope_rule":{"rule_type":1}}]}],"page_token":"next-page","has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, subject_body),
        http_response(200, folder_body),
        http_response(200, i18n_body),
        http_response(200, todo_body),
        http_response(200, role_get_body),
        http_response(200, role_body),
    ])
    .await;

    let client = client_for(addr);
    let subject_ids = ["subject-1", "subject-2"];
    let page = PageQuery::new().page_size(20).page_token("seed-token");
    let tenant_option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    };
    let user_option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..Default::default()
    };

    client
        .hire()
        .subject
        .list_by_query(
            &ListHireSubjectQuery::new()
                .page(page)
                .user_id_type("open_id")
                .subject_ids(Some(subject_ids.as_slice())),
            &tenant_option,
        )
        .await
        .unwrap();
    client
        .hire()
        .talent_folder
        .list_by_query(
            &ListHireTalentFolderQuery::new()
                .page(page)
                .user_id_type("open_id"),
            &tenant_option,
        )
        .await
        .unwrap();
    client
        .hire()
        .termination_reason
        .list_by_query(
            &ListHireTerminationReasonQuery::new().page(page),
            &tenant_option,
        )
        .await
        .unwrap();
    client
        .hire()
        .todo
        .list_by_query(
            &ListHireTodoQuery::new()
                .page_size("20")
                .page_token("seed-token")
                .user_id("ou_user_1")
                .user_id_type("open_id")
                .type_("evaluation"),
            &user_option,
        )
        .await
        .unwrap();
    let role = client
        .hire()
        .role
        .get("role-1", &tenant_option)
        .await
        .unwrap();
    let user_roles = client
        .hire()
        .user_role
        .list_by_query(
            &ListHireUserRoleQuery::new()
                .page(page)
                .user_id("ou_user_1")
                .role_id("role-1")
                .update_start_time("1710000000")
                .update_end_time("1710009999")
                .user_id_type("open_id"),
            &tenant_option,
        )
        .await
        .unwrap();

    assert_eq!(
        role.data
            .unwrap()
            .role
            .unwrap()
            .socail_permission_collection
            .unwrap()
            .data_permissions
            .unwrap()[0]
            .select_status,
        Some(1)
    );
    assert_eq!(
        user_roles.data.unwrap().items[0]
            .business_management_scopes
            .as_ref()
            .unwrap()[0]
            .scope_rule
            .as_ref()
            .unwrap()
            .rule_type,
        Some(1)
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/subjects?"));
    assert!(request.contains("GET /open-apis/hire/v1/talent_folders?"));
    assert!(request.contains("GET /open-apis/hire/v1/termination_reasons?"));
    assert!(request.contains("GET /open-apis/hire/v1/todos?"));
    assert!(request.contains("GET /open-apis/hire/v1/roles/role-1 "));
    assert!(request.contains("GET /open-apis/hire/v1/user_roles?"));
    assert_eq!(request.matches("page_size=20").count(), 5);
    assert_eq!(request.matches("page_token=seed-token").count(), 5);
    assert!(request.contains("subject_ids=subject-1"));
    assert!(request.contains("subject_ids=subject-2"));
    assert!(request.contains("user_id=ou_user_1"));
    assert!(request.contains("role_id=role-1"));
    assert!(request.contains("type=evaluation"));
}
