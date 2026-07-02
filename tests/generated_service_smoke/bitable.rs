use super::*;

// ── Bitable ──

#[tokio::test]
async fn bitable_table_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"table_id":"tbl-1","name":"Tasks"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListTableQuery::new("app-token-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .bitable()
        .table
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].name.as_deref(), Some("Tasks"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/tables?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn bitable_view_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"view_id":"view-1","view_name":"Grid"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListViewQuery::new("app-token-1", "tbl-1")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .bitable()
        .view
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].view_name.as_deref(), Some("Grid"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/tables/tbl-1/views?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn bitable_view_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"view_id":"view-1","view_name":"Grid"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .bitable()
        .view
        .list(
            "app-token-1",
            "tbl-1",
            Some(20),
            Some("next-page"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/tables/tbl-1/views?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn bitable_field_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"field_id":"fld-1","field_name":"Status","type":1}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFieldQuery::new("app-token-1", "tbl-1")
        .view_id("view-1")
        .text_field_as_array(true)
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .bitable()
        .field
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].field_name.as_deref(), Some("Status"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/tables/tbl-1/fields?"));
    assert!(request.contains("view_id=view-1"));
    assert!(request.contains("text_field_as_array=true"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn bitable_field_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"field_id":"fld-1","field_name":"Status","type":1}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .bitable()
        .field
        .list(
            "app-token-1",
            "tbl-1",
            Some("view-1"),
            Some(true),
            Some("open_id"),
            Some("next-page"),
            Some(20),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/tables/tbl-1/fields?"));
    assert!(request.contains("view_id=view-1"));
    assert!(request.contains("text_field_as_array=true"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn bitable_record_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"record_id":"rec-1","fields":{"Name":"Task"}}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListRecordQuery::new("app-token-1", "tbl-1")
        .view_id("view-1")
        .filter("filter-1")
        .sort("sort-1")
        .field_names("Name")
        .text_field_as_array(true)
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .bitable()
        .record
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].record_id.as_deref(), Some("rec-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/tables/tbl-1/records?"));
    assert!(request.contains("view_id=view-1"));
    assert!(request.contains("filter=filter-1"));
    assert!(request.contains("sort=sort-1"));
    assert!(request.contains("field_names=Name"));
    assert!(request.contains("text_field_as_array=true"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn bitable_record_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"record_id":"rec-1","fields":{"Name":"Task"}}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .bitable()
        .record
        .list(
            "app-token-1",
            "tbl-1",
            Some("view-1"),
            Some("filter-1"),
            Some("sort-1"),
            Some("Name"),
            Some(true),
            Some("open_id"),
            Some("next-page"),
            Some(20),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/tables/tbl-1/records?"));
    assert!(request.contains("view_id=view-1"));
    assert!(request.contains("filter=filter-1"));
    assert!(request.contains("sort=sort-1"));
    assert!(request.contains("field_names=Name"));
    assert!(request.contains("text_field_as_array=true"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn bitable_record_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"record_id":"rec-1","fields":{"Name":"Task"}}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SearchRecordQuery::new("app-token-1", "tbl-1")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let body = SearchRecordReqBody {
        automatic_fields: Some(true),
        ..Default::default()
    };
    let resp = client
        .bitable()
        .record
        .search_by_query(&query, &body, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].record_id.as_deref(), Some("rec-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request
            .contains("POST /open-apis/bitable/v1/apps/app-token-1/tables/tbl-1/records/search?")
    );
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""automatic_fields":true"#));
}

#[tokio::test]
async fn bitable_dashboard_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"dashboards":[{"block_id":"dash-1","name":"Overview"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListDashboardQuery::new("app-token-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .bitable()
        .dashboard
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.dashboards[0].name.as_deref(), Some("Overview"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/dashboards?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

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
async fn bitable_workflow_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"workflows":[{"workflow_id":"flow-1","title":"Notify"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListAppWorkflowQuery::new("app-token-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .bitable()
        .workflow
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.workflows[0].title.as_deref(), Some("Notify"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/workflows?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn bitable_workflow_list_positional_adapter_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"workflows":[{"workflow_id":"flow-1","title":"Notify"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .bitable()
        .workflow
        .list(
            "app-token-1",
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/workflows?"));
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

#[tokio::test]
async fn bitable_form_field_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"field_id":"fld-1","title":"Name"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFormFieldQuery::new("app-token-1", "tbl-1", "form-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .bitable()
        .form_field
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].title.as_deref(), Some("Name"));
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains(
            "GET /open-apis/bitable/v1/apps/app-token-1/tables/tbl-1/forms/form-1/fields?"
        )
    );
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
