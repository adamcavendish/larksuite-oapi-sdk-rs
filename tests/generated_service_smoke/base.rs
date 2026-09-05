use super::prelude::*;
use larksuite_oapi_sdk_rs::service::base::v3::{
    DashboardUserIdTypeQuery, GetWorkflowQuery, ListDashboardBlocksQuery, ListDashboardsQuery,
    ListWorkflowQuery,
};

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
async fn base_v3_workflow_contract_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;
    let client = client_for(addr);
    let user_option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    };
    let tenant_option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..RequestOption::default()
    };
    let create_body = json_value!({
        "client_token": "workflow-create-1",
        "title": "Classify feedback",
        "steps": [{"id": "classify", "type": "AIClassificationBranch"}],
    });
    let update_body = json_value!({
        "title": "Analyze feedback",
        "steps": [{"id": "analyze", "type": "AIAnalysisAction"}],
    });

    client
        .base_v3()
        .workflow
        .create("base token", &create_body, &user_option)
        .await
        .unwrap();
    client
        .base_v3()
        .workflow
        .get(
            &GetWorkflowQuery::new("base token", "workflow/id").user_id_type("open_id"),
            &tenant_option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .workflow
        .list(
            &ListWorkflowQuery::new("base token")
                .status("disabled")
                .page(PageQuery::new().page_size(100).page_token("next page")),
            &user_option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .workflow
        .update("base token", "workflow/id", &update_body, &tenant_option)
        .await
        .unwrap();
    client
        .base_v3()
        .workflow
        .enable("base token", "workflow/id", &user_option)
        .await
        .unwrap();
    client
        .base_v3()
        .workflow
        .disable("base token", "workflow/id", &tenant_option)
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/base/v3/bases/base%20token/workflows "));
    assert!(request.contains(
        "GET /open-apis/base/v3/bases/base%20token/workflows/workflow%2Fid?user_id_type=open_id"
    ));
    assert!(request.contains("POST /open-apis/base/v3/bases/base%20token/workflows/list "));
    assert!(request.contains("PUT /open-apis/base/v3/bases/base%20token/workflows/workflow%2Fid "));
    assert!(
        request.contains(
            "PATCH /open-apis/base/v3/bases/base%20token/workflows/workflow%2Fid/enable "
        )
    );
    assert!(
        request.contains(
            "PATCH /open-apis/base/v3/bases/base%20token/workflows/workflow%2Fid/disable "
        )
    );
    assert!(request.contains(r#""client_token":"workflow-create-1""#));
    assert!(request.contains(r#""type":"AIClassificationBranch""#));
    assert!(request.contains(r#""type":"AIAnalysisAction""#));
    assert!(request.contains(r#""status":"disabled""#));
    assert!(request.contains("\"page_size\":100"));
    assert!(request.contains(r#""page_token":"next page""#));
    assert!(request.contains("authorization: Bearer user-token"));
    assert!(request.contains("authorization: Bearer tenant-token"));
    assert!(request.contains("x-app-id: test_app_id"));
}

#[tokio::test]
async fn base_v3_share_contract_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;
    let client = client_for(addr);
    let user_option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    };
    let tenant_option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..RequestOption::default()
    };
    let dashboard_update = UpdateDashboardShareReqBody::new()
        .access_scope("tenant")
        .settings(UpdateDashboardShareSettings::new().show_source(false));
    let form_update = UpdateFormShareReqBody::new()
        .enabled(false)
        .settings(UpdateFormShareSettings::new().require_login(false));

    client
        .base_v3()
        .dashboard_share
        .get("base token", "dashboard id", &user_option)
        .await
        .unwrap();
    client
        .base_v3()
        .dashboard_share
        .update(
            "base token",
            "dashboard id",
            &dashboard_update,
            &tenant_option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .form_share
        .get("base token", "table id", "form id", &tenant_option)
        .await
        .unwrap();
    client
        .base_v3()
        .form_share
        .update(
            "base token",
            "table id",
            "form id",
            &form_update,
            &user_option,
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    for path in [
        "GET /open-apis/base/v3/bases/base%20token/dashboards/dashboard%20id/share ",
        "PATCH /open-apis/base/v3/bases/base%20token/dashboards/dashboard%20id/share ",
        "GET /open-apis/base/v3/bases/base%20token/tables/table%20id/forms/form%20id/share ",
        "PATCH /open-apis/base/v3/bases/base%20token/tables/table%20id/forms/form%20id/share ",
    ] {
        assert!(
            request.contains(path),
            "missing request path {path}:\n{request}"
        );
    }
    for value in [
        "authorization: Bearer user-token",
        "authorization: Bearer tenant-token",
        "x-app-id: test_app_id",
        r#""access_scope":"tenant""#,
        r#""show_source":false"#,
        r#""enabled":false"#,
        r#""require_login":false"#,
    ] {
        assert!(
            request.contains(value),
            "missing request value {value}:\n{request}"
        );
    }
    assert!(!request.contains("enable_auto_analysis"));
    assert!(!request.contains("allow_anonymous"));
}

#[tokio::test]
async fn base_v3_dashboard_contract_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body); 12]).await;
    let client = client_for(addr);
    let user_option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    };
    let tenant_option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..RequestOption::default()
    };
    let user_id_type = DashboardUserIdTypeQuery::new().user_id_type("open_id");

    client
        .base_v3()
        .dashboard
        .list(
            &ListDashboardsQuery::new("base token")
                .page(PageQuery::new().page_size(20).page_token("next-dashboard")),
            &user_option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .dashboard
        .get("base token", "dashboard id", &tenant_option)
        .await
        .unwrap();
    client
        .base_v3()
        .dashboard
        .create(
            "base token",
            json_value!({"name":"Sales","theme":{"theme_style":"light"}}),
            &user_option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .dashboard
        .update(
            "base token",
            "dashboard id",
            json_value!({"name":"Sales 2026"}),
            &tenant_option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .dashboard
        .delete("base token", "dashboard id", &user_option)
        .await
        .unwrap();
    client
        .base_v3()
        .dashboard_block
        .list(
            &ListDashboardBlocksQuery::new("base token", "dashboard id")
                .page(PageQuery::new().page_size(10).page_token("next-block")),
            &tenant_option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .dashboard_block
        .get(
            "base token",
            "dashboard id",
            "block id",
            &user_id_type,
            &user_option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .dashboard_block
        .create(
            "base token",
            "dashboard id",
            json_value!({
                "name":"Revenue",
                "type":"statistics",
                "data_config":{"table_name":"Orders","count_all":true}
            }),
            &user_id_type,
            &tenant_option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .dashboard_block
        .update(
            "base token",
            "dashboard id",
            "block id",
            json_value!({"data_config":{"limit_size":20}}),
            &user_id_type,
            &user_option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .dashboard_block
        .delete("base token", "dashboard id", "block id", &tenant_option)
        .await
        .unwrap();
    client
        .base_v3()
        .dashboard_block
        .get_data("base token", "block id", &user_option)
        .await
        .unwrap();
    client
        .base_v3()
        .dashboard
        .arrange("base token", "dashboard id", &user_id_type, &tenant_option)
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    for method_path in [
        "GET /open-apis/base/v3/bases/base%20token/dashboards?",
        "GET /open-apis/base/v3/bases/base%20token/dashboards/dashboard%20id ",
        "POST /open-apis/base/v3/bases/base%20token/dashboards ",
        "PATCH /open-apis/base/v3/bases/base%20token/dashboards/dashboard%20id ",
        "DELETE /open-apis/base/v3/bases/base%20token/dashboards/dashboard%20id ",
        "GET /open-apis/base/v3/bases/base%20token/dashboards/dashboard%20id/blocks?",
        "GET /open-apis/base/v3/bases/base%20token/dashboards/dashboard%20id/blocks/block%20id?",
        "POST /open-apis/base/v3/bases/base%20token/dashboards/dashboard%20id/blocks?",
        "PATCH /open-apis/base/v3/bases/base%20token/dashboards/dashboard%20id/blocks/block%20id?",
        "DELETE /open-apis/base/v3/bases/base%20token/dashboards/dashboard%20id/blocks/block%20id ",
        "GET /open-apis/base/v3/bases/base%20token/dashboards/blocks/block%20id/data ",
        "POST /open-apis/base/v3/bases/base%20token/dashboards/dashboard%20id/arrange?",
    ] {
        assert!(
            request.contains(method_path),
            "missing request path {method_path}:\n{request}"
        );
    }
    for value in [
        "page_size=20",
        "page_token=next-dashboard",
        "page_size=10",
        "page_token=next-block",
        "user_id_type=open_id",
        "authorization: Bearer user-token",
        "authorization: Bearer tenant-token",
        "x-app-id: test_app_id",
        r#""name":"Sales""#,
        r#""name":"Sales 2026""#,
        r#""type":"statistics""#,
        r#""limit_size":20"#,
    ] {
        assert!(
            request.contains(value),
            "missing request value {value}:\n{request}"
        );
    }
}

#[tokio::test]
async fn base_v3_field_extensions_and_record_share_links_contract_smoke() {
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, r#"{"code":0,"msg":"ok","data":{"current_extension":null}}"#),
        http_response(200, r#"{"code":0,"msg":"ok","data":{}}"#),
        http_response(200, r#"{"code":0,"msg":"ok","data":{}}"#),
        http_response(
            200,
            r#"{"code":0,"msg":"ok","data":{"record_share_links":{"rec-1":"https://example.test/rec-1"}}}"#,
        ),
    ])
    .await;
    let client = client_for(addr);
    let option = RequestOption {
        user_access_token: Some("user-token".into()),
        ..RequestOption::default()
    };
    let extension =
        UpdateFieldExtensionReqBody::builtin_llm_completion(FieldExtensionCompletionInput::new([
            FieldExtensionPromptSegment::text("Summarize "),
            FieldExtensionPromptSegment::field_ref("Description"),
        ]));

    client
        .base_v3()
        .field_extension
        .get("base token", "table id", "field id", &option)
        .await
        .unwrap();
    client
        .base_v3()
        .field_extension
        .update("base token", "table id", "field id", &extension, &option)
        .await
        .unwrap();
    client
        .base_v3()
        .field_extension
        .update_cells(
            "base token",
            "table id",
            "field id",
            &UpdateFieldExtensionCellsReqBody::column(Some("view id")),
            &option,
        )
        .await
        .unwrap();
    let shares = client
        .base_v3()
        .record
        .create_share_links(
            "base token",
            "table id",
            &CreateRecordShareLinksReqBody::new(["rec-1", "rec-2"]),
            &option,
        )
        .await
        .unwrap();

    assert!(shares.success());
    assert_eq!(
        shares
            .data
            .as_ref()
            .and_then(|data| data.record_share_links.as_ref())
            .and_then(|links| links.get("rec-1"))
            .map(String::as_str),
        Some("https://example.test/rec-1")
    );
    let request = requests.lock().unwrap().join("\n");
    for path in [
        "GET /open-apis/base/v3/bases/base%20token/tables/table%20id/fields/field%20id/field_extensions ",
        "PUT /open-apis/base/v3/bases/base%20token/tables/table%20id/fields/field%20id/field_extensions ",
        "POST /open-apis/base/v3/bases/base%20token/tables/table%20id/fields/field%20id/field_extensions/update_cells ",
        "POST /open-apis/base/v3/bases/base%20token/tables/table%20id/records/share_links/batch ",
    ] {
        assert!(
            request.contains(path),
            "missing request path {path}:\n{request}"
        );
    }
    for body_fragment in [
        r#""extension_id":"builtin_llm_completion""#,
        r#""type":"field_ref""#,
        r#""view_id":"view id""#,
        r#""record_ids":["rec-1","rec-2"]"#,
    ] {
        assert!(
            request.contains(body_fragment),
            "missing request body {body_fragment}:\n{request}"
        );
    }
}

#[tokio::test]
async fn base_v3_template_center_contract_smoke() {
    let category_body =
        r#"{"code":0,"msg":"ok","data":{"categories":[{"key":"office","name":"Office"}]}}"#;
    let template_body = r#"{"code":0,"msg":"ok","data":{"templates":[{"token":"tpl_1","name":"Weekly report","introduction":"Status updates","scenarios":["Reporting"],"developer":"Base Team","link":"https://example.com/base/tpl_1","created_at":"2025-12-03T02:53:34Z","updated_at":"2026-06-22T08:18:58Z"}],"has_more":true,"offset":"next"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, category_body),
        http_response(200, template_body),
        http_response(200, template_body),
    ])
    .await;
    let client = client_for(addr);
    let user_option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    };
    let tenant_option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..RequestOption::default()
    };

    let categories = client
        .base_v3()
        .template
        .list_categories(&user_option)
        .await
        .unwrap();
    let templates = client
        .base_v3()
        .template
        .list(
            &ListBaseTemplateQuery::new()
                .category_key("office")
                .limit(20)
                .offset("cursor 1"),
            &tenant_option,
        )
        .await
        .unwrap();
    let search = client
        .base_v3()
        .template
        .search(
            &SearchBaseTemplateQuery::new("AI plans")
                .limit(10)
                .offset("cursor 2"),
            &user_option,
        )
        .await
        .unwrap();

    assert_eq!(
        categories
            .data
            .as_ref()
            .and_then(|data| data.categories.as_ref())
            .and_then(|items| items.first())
            .and_then(|category| category.key.as_deref()),
        Some("office")
    );
    assert_eq!(
        templates
            .data
            .as_ref()
            .and_then(|data| data.templates.as_ref())
            .and_then(|items| items.first())
            .and_then(|template| template.token.as_deref()),
        Some("tpl_1")
    );
    let template = templates
        .data
        .as_ref()
        .and_then(|data| data.templates.as_ref())
        .and_then(|items| items.first())
        .unwrap();
    assert_eq!(template.introduction.as_deref(), Some("Status updates"));
    assert_eq!(
        template.scenarios.as_ref().unwrap(),
        &["Reporting".to_owned()]
    );
    assert_eq!(template.developer.as_deref(), Some("Base Team"));
    assert_eq!(
        template.link.as_deref(),
        Some("https://example.com/base/tpl_1")
    );
    assert_eq!(template.created_at.as_deref(), Some("2025-12-03T02:53:34Z"));
    assert_eq!(template.updated_at.as_deref(), Some("2026-06-22T08:18:58Z"));
    assert_eq!(
        search.data.as_ref().and_then(|data| data.offset.as_deref()),
        Some("next")
    );

    let request = requests.lock().unwrap().join("\n");
    for value in [
        "GET /open-apis/base/v3/bases/templates/category ",
        "GET /open-apis/base/v3/bases/templates?",
        "GET /open-apis/base/v3/bases/templates/search?",
        "category_key=office",
        "keyword=AI+plans",
        "limit=20",
        "limit=10",
        "offset=cursor+1",
        "offset=cursor+2",
        "authorization: Bearer user-token",
        "authorization: Bearer tenant-token",
        "x-app-id: test_app_id",
    ] {
        assert!(request.contains(value), "missing {value}:\n{request}");
    }
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
