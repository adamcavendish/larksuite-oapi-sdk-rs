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

#[tokio::test]
async fn base_v3_record_read_contract_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"record_id_list":["rec-1"],"data":[["Task"]],"has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let user_option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    };
    let tenant_option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..RequestOption::default()
    };
    let fields = ["fld-name", "fld-status"];

    let list = client
        .base_v3()
        .record
        .list(
            &ListBaseV3RecordQuery::new("base token", "table id")
                .field_ids(&fields)
                .view_id("view-1")
                .filter(r#"{"conjunction":"and"}"#)
                .sort(r#"[{"field_name":"Name","desc":false}]"#)
                .offset(100)
                .limit(2),
            &user_option,
        )
        .await
        .unwrap();
    let search = client
        .base_v3()
        .record
        .search(
            "base token",
            "table id",
            json_value!({
                "keyword": "Task",
                "search_fields": ["Name"],
                "offset": 0,
                "limit": 2,
            }),
            &tenant_option,
        )
        .await
        .unwrap();

    assert!(list.success());
    assert!(search.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("GET /open-apis/base/v3/bases/base%20token/tables/table%20id/records?")
    );
    assert!(
        request.contains(
            "POST /open-apis/base/v3/bases/base%20token/tables/table%20id/records/search "
        )
    );
    assert!(request.contains("field_id=fld-name"));
    assert!(request.contains("field_id=fld-status"));
    assert!(request.contains("view_id=view-1"));
    assert!(request.contains("offset=100"));
    assert!(request.contains("limit=2"));
    assert!(request.contains("authorization: Bearer user-token"));
    assert!(request.contains("authorization: Bearer tenant-token"));
    assert!(request.contains("x-app-id: test_app_id"));
    assert!(request.contains(r#""keyword":"Task""#));
    assert!(request.contains(r#""search_fields":["Name"]"#));
}

#[tokio::test]
async fn base_v3_application_mode_contract_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;
    let client = client_for(addr);
    let option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    };
    let tenant_option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..RequestOption::default()
    };

    let tenant_error = client
        .base_v3()
        .app
        .get("app token", &tenant_option)
        .await
        .unwrap_err();
    assert!(
        tenant_error
            .to_string()
            .contains("tenant access token is not supported")
    );

    client
        .base_v3()
        .workspace
        .create(json_value!({"name": "Growth workspace"}), &option)
        .await
        .unwrap();
    client
        .base_v3()
        .workspace
        .list_entities(
            &ListWorkspaceEntitiesQuery::new("workspace token")
                .entity_type("baseapp")
                .page(PageQuery::new().page_size(20).page_token("workspace-next")),
            &option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .workspace
        .move_in(
            "workspace token",
            json_value!({"entity_token": "base-token"}),
            &option,
        )
        .await
        .unwrap();

    client
        .base_v3()
        .app
        .create(
            json_value!({
                "name": "Sales app",
                "workspace_token": "workspace token",
                "theme": {"theme_style": "cloudBlue"},
            }),
            &option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .app
        .get("app token", &option)
        .await
        .unwrap();

    client
        .base_v3()
        .page
        .list(
            &ListBaseAppPagesQuery::new("app token")
                .page(PageQuery::new().page_size(20).page_token("page-next")),
            &option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .page
        .get("app token", "page id", &option)
        .await
        .unwrap();
    client
        .base_v3()
        .page
        .create("app token", json_value!({"name": "Overview"}), &option)
        .await
        .unwrap();
    client
        .base_v3()
        .page
        .rename(
            "app token",
            "page id",
            json_value!({"name": "Revenue"}),
            &option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .page
        .delete("app token", "page id", &option)
        .await
        .unwrap();

    client
        .base_v3()
        .block
        .list(
            &ListBaseAppBlocksQuery::new("app token", "page id")
                .page(PageQuery::new().page_size(20).page_token("block-next")),
            &option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .block
        .get("app token", "page id", "block id", &option)
        .await
        .unwrap();
    client
        .base_v3()
        .block
        .create(
            "app token",
            "page id",
            json_value!({"name": "Orders", "type": "text", "data_config": {"text": "# Orders"}}),
            &option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .block
        .update(
            "app token",
            "page id",
            "block id",
            json_value!({"name": "Revenue"}),
            &option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .block
        .get_data(
            &GetBaseAppBlockDataQuery::new("app token", "chart token", "base token"),
            &option,
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    for path in [
        "POST /open-apis/base/v3/workspaces ",
        "GET /open-apis/base/v3/workspaces/workspace%20token/entities?",
        "POST /open-apis/base/v3/workspaces/workspace%20token/move_in ",
        "POST /open-apis/base/v3/base_apps ",
        "GET /open-apis/base/v3/base_apps/app%20token ",
        "GET /open-apis/base/v3/base_apps/app%20token/pages?",
        "GET /open-apis/base/v3/base_apps/app%20token/pages/page%20id ",
        "POST /open-apis/base/v3/base_apps/app%20token/pages ",
        "PATCH /open-apis/base/v3/base_apps/app%20token/pages/page%20id ",
        "DELETE /open-apis/base/v3/base_apps/app%20token/pages/page%20id ",
        "GET /open-apis/base/v3/base_apps/app%20token/pages/page%20id/blocks?",
        "GET /open-apis/base/v3/base_apps/app%20token/pages/page%20id/blocks/block%20id ",
        "POST /open-apis/base/v3/base_apps/app%20token/pages/page%20id/blocks ",
        "PATCH /open-apis/base/v3/base_apps/app%20token/pages/page%20id/blocks/block%20id ",
        "GET /open-apis/base/v3/base_apps/app%20token/blocks/chart%20token/data?",
    ] {
        assert!(
            request.contains(path),
            "missing request path {path}:\n{request}"
        );
    }
    for value in [
        "entity_type=baseapp",
        "page_size=20",
        "page_token=workspace-next",
        "page_token=page-next",
        "page_token=block-next",
        "base_token=base+token",
        "authorization: Bearer user-token",
        "x-app-id: test_app_id",
        r#""workspace_token":"workspace token""#,
        r#""entity_token":"base-token""#,
        r#""name":"Revenue""#,
        r##""data_config":{"text":"# Orders"}"##,
    ] {
        assert!(
            request.contains(value),
            "missing request value {value}:\n{request}"
        );
    }
}
