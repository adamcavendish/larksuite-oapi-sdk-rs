mod common;

use common::{http_response, http_response_with_headers, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::{
    bitable::v1::{
        ListAppRoleMemberQuery, ListAppRoleQuery, ListAppWorkflowQuery, ListDashboardQuery,
        ListFieldQuery, ListFormFieldQuery, ListRecordQuery, ListTableQuery, ListViewQuery,
        SearchRecordQuery, SearchRecordReqBody,
    },
    common::PageQuery,
    contact::v3::{
        ChildrenDepartmentQuery, FindUserByDepartmentQuery, ListDepartmentQuery,
        ListEmployeeTypeEnumQuery, ListScopeQuery, ListUserQuery, SearchDepartmentQuery,
        SearchDepartmentReqBody,
    },
    directory::{
        ListCollaborationRuleQuery, ListCollaborationTenantQuery, ListDirectoryUserQuery,
        ListShareEntityQuery,
    },
    drive::{
        v1::{ListFileQuery, ListFileVersionQuery, ListFileViewRecordQuery},
        v2::ListFileLikeQuery,
    },
    task::{CreateTaskReqBody, ListTaskQuery},
    vc::{
        GetMeetingListQuery, GetParticipantListQuery, GetParticipantQualityListQuery,
        GetResourceReservationListQuery, GetTopUserReportQuery, ListAlertQuery,
        ListMeetingByNoQuery, RoomConfigQuery,
    },
};

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

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

// ── IM ──

#[tokio::test]
async fn im_get_chat_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"chat_id":"oc_1","name":"team"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat
        .get("oc_1", Some("open_id"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.unwrap().name.as_deref(), Some("team"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/oc_1?user_id_type=open_id"));
}

#[tokio::test]
async fn im_list_chat_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"chat_id":"oc_1","name":"team"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat
        .list(None, None, None, None, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.as_ref().unwrap().len(), 1);
    assert_eq!(
        data.items.as_ref().unwrap()[0].name.as_deref(),
        Some("team")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats"));
}

#[tokio::test]
async fn im_message_resource_download_smoke() {
    let body = "resource-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"resource.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .im()
        .message_resource
        .get("msg-1", "file-key-1", "image", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("resource.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/messages/msg-1/resources/file-key-1?"));
    assert!(request.contains("type=image"));
}

#[tokio::test]
async fn im_file_download_smoke() {
    let body = "file-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"im-file.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .im()
        .file
        .get("file-key-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("im-file.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/files/file-key-1"));
}

#[tokio::test]
async fn im_image_download_smoke() {
    let body = "image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"image.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .im()
        .image
        .get("image-key-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("image.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/images/image-key-1"));
}

// ── Contact ──

#[tokio::test]
async fn contact_department_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-1","name":"Engineering"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListDepartmentQuery::new()
        .user_id_type("open_id")
        .department_id_type("department_id")
        .parent_department_id("od-parent")
        .fetch_child(true)
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .department
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.items.as_ref().unwrap()[0].name.as_deref(),
        Some("Engineering")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/departments?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("parent_department_id=od-parent"));
    assert!(request.contains("fetch_child=true"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_department_children_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-child","name":"Platform"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .department
        .children(
            "od-parent",
            Some("open_id"),
            Some("department_id"),
            Some(false),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.items.as_ref().unwrap()[0].department_id.as_deref(),
        Some("od-child")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/departments/od-parent/children?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("fetch_child=false"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_department_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-search","name":"Search Result"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SearchDepartmentQuery::new()
        .user_id_type("open_id")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let body = SearchDepartmentReqBody {
        query: Some("search".to_string()),
    };
    let resp = client
        .contact()
        .department
        .search_by_query(&query, &body, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/contact/v3/departments/search?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""query":"search""#));
}

#[tokio::test]
async fn contact_department_children_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ChildrenDepartmentQuery::new("od-parent")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .department
        .children_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/departments/od-parent/children?"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_user_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"u-1","name":"Ada"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListUserQuery::new()
        .user_id_type("open_id")
        .department_id_type("department_id")
        .department_id("od-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .user
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.as_ref().unwrap()[0].name.as_deref(), Some("Ada"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/users?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_user_find_by_department_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"u-2","name":"Grace"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .user
        .find_by_department(
            "od-1",
            Some("open_id"),
            Some("department_id"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.items.as_ref().unwrap()[0].name.as_deref(),
        Some("Grace")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/users/find_by_department?"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_user_find_by_department_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = FindUserByDepartmentQuery::new("od-1")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .user
        .find_by_department_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/users/find_by_department?"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_scope_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"department_ids":["od-1"],"user_ids":["u-1"],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListScopeQuery::new()
        .user_id_type("open_id")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .scope
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.department_ids.as_ref().unwrap()[0], "od-1");
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/scopes?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_employee_type_enum_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"enum_id":"enum-1","enum_value":"full_time"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListEmployeeTypeEnumQuery::new()
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .contact()
        .employee_type_enum
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.items.as_ref().unwrap()[0].enum_id.as_deref(),
        Some("enum-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/employee_type_enums?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

// ── Drive ──

#[tokio::test]
async fn drive_get_export_task_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"result":{"token":"t-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .export_task
        .get("ticket-1", "file-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().result.unwrap().token.as_deref(),
        Some("t-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/export_tasks/ticket-1?token=file-token-1"));
}

#[tokio::test]
async fn drive_export_task_download_smoke() {
    let body = "export-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"export.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .export_task
        .download("file-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("export.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/export_tasks/file/file-token-1/download"));
}

#[tokio::test]
async fn drive_list_files_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"files":[{"token":"file-1","name":"doc.pdf","type":"file"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFileQuery::new()
        .folder_token("folder-1")
        .order_by("EditedTime")
        .direction("DESC")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .drive()
        .file
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.files.as_ref().unwrap().len(), 1);
    assert_eq!(
        data.files.as_ref().unwrap()[0].name.as_deref(),
        Some("doc.pdf")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files?"));
    assert!(request.contains("folder_token=folder-1"));
    assert!(request.contains("order_by=EditedTime"));
    assert!(request.contains("direction=DESC"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn drive_list_files_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"files":[{"token":"file-1","name":"doc.pdf","type":"file"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file
        .list(
            None,
            None,
            None,
            None,
            None,
            None,
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.files.as_ref().unwrap().len(), 1);
    assert_eq!(
        data.files.as_ref().unwrap()[0].name.as_deref(),
        Some("doc.pdf")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files"));
}

#[tokio::test]
async fn drive_file_download_smoke() {
    let body = "file-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"file.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file
        .download("file-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("file.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/download"));
}

#[tokio::test]
async fn drive_file_upload_all_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"file_token":"file-token-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file
        .upload_all(
            "report.txt",
            "explorer",
            "folder-token-1",
            3,
            Some("checksum-1"),
            b"abc".to_vec(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().file_token.as_deref(),
        Some("file-token-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/files/upload_all"));
    assert!(request.contains("name=\"file_name\""));
    assert!(request.contains("report.txt"));
    assert!(request.contains("name=\"parent_type\""));
    assert!(request.contains("explorer"));
    assert!(request.contains("name=\"parent_node\""));
    assert!(request.contains("folder-token-1"));
    assert!(request.contains("name=\"size\""));
    assert!(request.contains("\r\n3\r\n"));
    assert!(request.contains("name=\"checksum\""));
    assert!(request.contains("checksum-1"));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("filename=\"report.txt\""));
    assert!(request.contains("abc"));
}

#[tokio::test]
async fn drive_file_upload_part_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file
        .upload_part(
            "upload-id-1",
            2,
            3,
            Some("checksum-1"),
            b"abc".to_vec(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/files/upload_part"));
    assert!(request.contains("name=\"upload_id\""));
    assert!(request.contains("upload-id-1"));
    assert!(request.contains("name=\"seq\""));
    assert!(request.contains("\r\n2\r\n"));
    assert!(request.contains("name=\"size\""));
    assert!(request.contains("\r\n3\r\n"));
    assert!(request.contains("name=\"checksum\""));
    assert!(request.contains("checksum-1"));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("filename=\"part\""));
    assert!(request.contains("abc"));
}

#[tokio::test]
async fn drive_v2_file_like_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"files":[{"token":"like-1","name":"Alice","type":"user"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFileLikeQuery::new("file-token-1")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .drive_v2()
        .file_like
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.files.len(), 1);
    assert_eq!(data.files[0].name.as_deref(), Some("Alice"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v2/files/file-token-1/likes?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn drive_v2_file_like_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"files":[],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive_v2()
        .file_like
        .list(
            "file-token-1",
            Some("open_id"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v2/files/file-token-1/likes?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn drive_file_version_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"name":"v1"}],"page_token":"next-version","has_more":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFileVersionQuery::new("file-token-1", "doc")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .drive()
        .file_version
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.as_ref().unwrap().len(), 1);
    assert_eq!(data.items.as_ref().unwrap()[0].name.as_deref(), Some("v1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/versions?"));
    assert!(request.contains("obj_type=doc"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn drive_file_view_record_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"viewer_id":"u1"}],"page_token":"next-view","has_more":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFileViewRecordQuery::new("file-token-1", "doc")
        .viewer_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .drive()
        .file_view_record
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.as_ref().unwrap().len(), 1);
    assert_eq!(
        data.items.as_ref().unwrap()[0].viewer_id.as_deref(),
        Some("u1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/view_records?"));
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("viewer_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn drive_media_download_smoke() {
    let body = "media-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"media.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .media
        .download(
            "media-token-1",
            Some("extra-value"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("media.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/medias/media-token-1/download?"));
    assert!(request.contains("extra=extra-value"));
}

#[tokio::test]
async fn drive_media_upload_all_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"file_token":"media-token-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .media
        .upload_all(
            "clip.mp4",
            "explorer",
            "folder-token-1",
            4,
            Some("checksum-1"),
            Some("extra-value"),
            b"clip".to_vec(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().file_token.as_deref(),
        Some("media-token-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/medias/upload_all"));
    assert!(request.contains("name=\"file_name\""));
    assert!(request.contains("clip.mp4"));
    assert!(request.contains("name=\"parent_type\""));
    assert!(request.contains("explorer"));
    assert!(request.contains("name=\"parent_node\""));
    assert!(request.contains("folder-token-1"));
    assert!(request.contains("name=\"size\""));
    assert!(request.contains("\r\n4\r\n"));
    assert!(request.contains("name=\"checksum\""));
    assert!(request.contains("checksum-1"));
    assert!(request.contains("name=\"extra\""));
    assert!(request.contains("extra-value"));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("filename=\"clip.mp4\""));
    assert!(request.contains("clip"));
}

#[tokio::test]
async fn drive_media_upload_part_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .media
        .upload_part(
            "upload-id-1",
            2,
            4,
            Some("checksum-1"),
            b"clip".to_vec(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/medias/upload_part"));
    assert!(request.contains("name=\"upload_id\""));
    assert!(request.contains("upload-id-1"));
    assert!(request.contains("name=\"seq\""));
    assert!(request.contains("\r\n2\r\n"));
    assert!(request.contains("name=\"size\""));
    assert!(request.contains("\r\n4\r\n"));
    assert!(request.contains("name=\"checksum\""));
    assert!(request.contains("checksum-1"));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("filename=\"part\""));
    assert!(request.contains("clip"));
}

// ── CoreHR ──

#[tokio::test]
async fn corehr_get_employee_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"employment":{"id":"emp-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr()
        .employee
        .get("emp-1", Some("open_id"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().employment.unwrap().id.as_deref(),
        Some("emp-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/employments/emp-1?user_id_type=open_id"));
}

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

// ── Calendar ──

#[tokio::test]
async fn calendar_get_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"calendar_id":"cal-1","summary":"Team Calendar"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .calendar()
        .calendar
        .get("cal-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.unwrap().summary.as_deref(), Some("Team Calendar"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/calendar/v4/calendars/cal-1"));
}

#[tokio::test]
async fn calendar_list_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"calendar_list":[{"calendar_id":"cal-1","summary":"cal"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .calendar()
        .calendar
        .list(None, None, None, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.calendar_list.len(), 1);
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/calendar/v4/calendars"));
}

// ── Contact ──

#[tokio::test]
async fn contact_get_user_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"user":{"open_id":"ou_1","name":"Alice"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .user
        .get("ou_1", Some("open_id"), None, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let user = resp.data.unwrap().user.unwrap();
    assert_eq!(user.name.as_deref(), Some("Alice"));
    assert_eq!(user.open_id.as_deref(), Some("ou_1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/users/ou_1?user_id_type=open_id"));
}

#[tokio::test]
async fn contact_list_department_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"name":"Engineering","department_id":"d-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .department
        .list(
            None,
            None,
            None,
            None,
            None,
            None,
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.as_ref().unwrap().len(), 1);
    assert_eq!(
        data.items.as_ref().unwrap()[0].name.as_deref(),
        Some("Engineering")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/departments"));
}

// ── Docx ──

#[tokio::test]
async fn docx_get_document_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"document":{"document_id":"doc-1","title":"My Doc"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .docx()
        .document
        .get("doc-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let doc = resp.data.unwrap().document.unwrap();
    assert_eq!(doc.document_id.as_deref(), Some("doc-1"));
    assert_eq!(doc.title.as_deref(), Some("My Doc"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/docx/v1/documents/doc-1"));
}

// ── Sheets ──

#[tokio::test]
async fn sheets_get_spreadsheet_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"spreadsheet":{"spreadsheet_token":"sht-1","title":"Budget"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .sheets()
        .spreadsheet
        .get("sht-1", None, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let ss = resp.data.unwrap().spreadsheet.unwrap();
    assert_eq!(ss.spreadsheet_token.as_deref(), Some("sht-1"));
    assert_eq!(ss.title.as_deref(), Some("Budget"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/sheets/v3/spreadsheets/sht-1"));
}

// ── Task ──

#[tokio::test]
async fn task_create_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"task":{"id":"task-1","summary":"New task"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let req_body = CreateTaskReqBody {
        summary: Some("New task".into()),
        ..Default::default()
    };
    let resp = client
        .task()
        .task
        .create(&req_body, Some("open_id"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let task = resp.data.unwrap().task.unwrap();
    assert_eq!(task.summary.as_deref(), Some("New task"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v1/tasks?user_id_type=open_id"));
    assert!(request.contains(r#""summary":"New task""#));
}

#[tokio::test]
async fn task_get_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"task":{"id":"task-1","summary":"Fix bug"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task()
        .task
        .get("task-1", None, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let task = resp.data.unwrap().task.unwrap();
    assert_eq!(task.id.as_deref(), Some("task-1"));
    assert_eq!(task.summary.as_deref(), Some("Fix bug"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v1/tasks/task-1"));
}

#[tokio::test]
async fn task_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"task-1","summary":"Fix bug"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task()
        .task
        .list(
            Some(20),
            Some("next-page"),
            Some("1700000000"),
            Some("1700000100"),
            Some(false),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v1/tasks?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("start_create_time=1700000000"));
    assert!(request.contains("end_create_time=1700000100"));
    assert!(request.contains("task_completed=false"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"task-1","summary":"Fix bug"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListTaskQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .task_completed(false)
        .user_id_type("open_id");
    let resp = client
        .task()
        .task
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].summary.as_deref(), Some("Fix bug"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v1/tasks?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("task_completed=false"));
    assert!(request.contains("user_id_type=open_id"));
}

// ── Wiki v2 ──

#[tokio::test]
async fn wiki_get_space_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"space":{"name":"Team Wiki","space_id":"sp-1","description":"docs"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .wiki_v2()
        .space
        .get("sp-1", None, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let space = resp.data.unwrap().space.unwrap();
    assert_eq!(space.name.as_deref(), Some("Team Wiki"));
    assert_eq!(space.space_id.as_deref(), Some("sp-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/wiki/v2/spaces/sp-1"));
}

// ── Approval ──

#[tokio::test]
async fn approval_get_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"approval_code":"apv-1","approval_name":"Leave Request","status":"ACTIVE"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .approval
        .get("apv-1", None, None, None, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.approval_name.as_deref(), Some("Leave Request"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/approvals/apv-1"));
}

// ── ACS ──

#[tokio::test]
async fn acs_list_devices_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"device_id":"dev-1","device_name":"Front Gate"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .acs()
        .device
        .list(&RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert!(resp.data.is_some());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/devices"));
}

// ── Helpdesk ──

#[tokio::test]
async fn helpdesk_get_faq_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"faq":{"id":"faq-1","answer":"Try restarting"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .get("faq-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert!(resp.data.is_some());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs/faq-1"));
}

// ── VC ──

#[tokio::test]
async fn vc_room_config_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"room_config":{"room_background":"blue"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = RoomConfigQuery::new(1)
        .country_id("country-1")
        .district_id("district-1")
        .building_id("building-1")
        .floor_name("3F")
        .room_id("room-1")
        .user_id_type("open_id");
    let resp = client
        .vc()
        .room_config
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.room_config.unwrap().room_background.as_deref(),
        Some("blue")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/room_configs/query?"));
    assert!(request.contains("scope=1"));
    assert!(request.contains("country_id=country-1"));
    assert!(request.contains("district_id=district-1"));
    assert!(request.contains("building_id=building-1"));
    assert!(request.contains("floor_name=3F"));
    assert!(request.contains("room_id=room-1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_room_config_query_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .room_config
        .query(
            1,
            Some("country-1"),
            Some("district-1"),
            Some("building-1"),
            Some("3F"),
            Some("room-1"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/room_configs/query?"));
    assert!(request.contains("scope=1"));
    assert!(request.contains("country_id=country-1"));
    assert!(request.contains("district_id=district-1"));
    assert!(request.contains("building_id=building-1"));
    assert!(request.contains("floor_name=3F"));
    assert!(request.contains("room_id=room-1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_meeting_list_by_no_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"meeting_briefs":[{"id":"m-1","topic":"Sprint Review"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListMeetingByNoQuery::new("meeting-no", "1700000000", "1700000100")
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .user_id_type("open_id");
    let resp = client
        .vc()
        .meeting
        .list_by_no_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.unwrap().meeting_briefs.len(), 1);
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/meetings/list_by_no?"));
    assert!(request.contains("meeting_no=meeting-no"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_report_top_user_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .report
        .get_top_user(
            "1700000000",
            "1700000100",
            50,
            1,
            Some(2),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/reports/get_top_user?"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("limit=50"));
    assert!(request.contains("order_by=1"));
    assert!(request.contains("meeting_type=2"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_report_top_user_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetTopUserReportQuery::new("1700000000", "1700000100", 50, 1)
        .meeting_type(2)
        .user_id_type("open_id");
    let resp = client
        .vc()
        .report
        .get_top_user_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/reports/get_top_user?"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("limit=50"));
    assert!(request.contains("order_by=1"));
    assert!(request.contains("meeting_type=2"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_alert_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListAlertQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .start_time("1700000000")
        .end_time("1700000100")
        .query_type(1)
        .query_value("room-1");
    let resp = client
        .vc()
        .alert
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/alerts?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("query_type=1"));
    assert!(request.contains("query_value=room-1"));
}

#[tokio::test]
async fn vc_participant_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .participant_list
        .get(
            "1700000000",
            "1700000100",
            Some(1),
            "meeting-no",
            Some("user-1"),
            Some("room-1"),
            Some(20),
            Some("next-page"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/participant_list?"));
    assert!(request.contains("meeting_start_time=1700000000"));
    assert!(request.contains("meeting_end_time=1700000100"));
    assert!(request.contains("meeting_status=1"));
    assert!(request.contains("meeting_no=meeting-no"));
    assert!(request.contains("user_id=user-1"));
    assert!(request.contains("room_id=room-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_participant_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetParticipantListQuery::new("1700000000", "1700000100", "meeting-no")
        .meeting_status(1)
        .user_id("user-1")
        .room_id("room-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .user_id_type("open_id");
    let resp = client
        .vc()
        .participant_list
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/participant_list?"));
    assert!(request.contains("meeting_start_time=1700000000"));
    assert!(request.contains("meeting_end_time=1700000100"));
    assert!(request.contains("meeting_status=1"));
    assert!(request.contains("meeting_no=meeting-no"));
    assert!(request.contains("user_id=user-1"));
    assert!(request.contains("room_id=room-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_participant_quality_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        GetParticipantQualityListQuery::new("1700000000", "1700000100", "meeting-no", "1700000050")
            .user_id("user-1")
            .room_id("room-1")
            .page(PageQuery::new().page_size(20).page_token("next-page"))
            .user_id_type("open_id");
    let resp = client
        .vc()
        .participant_quality_list
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/participant_quality_list?"));
    assert!(request.contains("meeting_start_time=1700000000"));
    assert!(request.contains("meeting_end_time=1700000100"));
    assert!(request.contains("meeting_no=meeting-no"));
    assert!(request.contains("join_time=1700000050"));
    assert!(request.contains("user_id=user-1"));
    assert!(request.contains("room_id=room-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_resource_reservation_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .resource_reservation_list
        .get(
            "level-1",
            Some(true),
            "1700000000",
            "1700000100",
            Some("room-1,room-2"),
            Some(false),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/resource_reservation_list?"));
    assert!(request.contains("room_level_id=level-1"));
    assert!(request.contains("has_video=true"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("room_ids=room-1%2Croom-2"));
    assert!(request.contains("is_exclude=false"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn vc_resource_reservation_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetResourceReservationListQuery::new("level-1", "1700000000", "1700000100")
        .has_video(true)
        .room_ids("room-1,room-2")
        .is_exclude(false)
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .vc()
        .resource_reservation_list
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/resource_reservation_list?"));
    assert!(request.contains("room_level_id=level-1"));
    assert!(request.contains("has_video=true"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("room_ids=room-1%2Croom-2"));
    assert!(request.contains("is_exclude=false"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn vc_export_download_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"file_token":"download-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .export
        .download("file-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap()["file_token"].as_str(),
        Some("download-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/exports/download?"));
    assert!(request.contains("file_token=file-token-1"));
}

#[tokio::test]
async fn vc_get_meeting_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"meeting":{"id":"m-1","topic":"Sprint Review"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .meeting_list
        .get(
            "m-1",
            "",
            Some(1),
            Some("meeting-no"),
            Some("user-1"),
            Some("room-1"),
            Some(20),
            Some("next-page"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/meeting_list?"));
    assert!(request.contains("start_time=m-1"));
    assert!(request.contains("end_time="));
    assert!(request.contains("meeting_status=1"));
    assert!(request.contains("meeting_no=meeting-no"));
    assert!(request.contains("user_id=user-1"));
    assert!(request.contains("room_id=room-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_get_meeting_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"meeting":{"id":"m-1","topic":"Sprint Review"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetMeetingListQuery::new("m-1", "")
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .user_id_type("open_id");
    let resp = client
        .vc()
        .meeting_list
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert!(resp.data.is_some());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/meeting_list?"));
    assert!(request.contains("start_time=m-1"));
    assert!(request.contains("end_time="));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

// ── Translation ──

#[tokio::test]
async fn translation_detect_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"language":"en"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let req_body = larksuite_oapi_sdk_rs::service::translation::DetectLanguageReqBody {
        text: Some("hello".into()),
    };
    let resp = client
        .translation()
        .text
        .detect_language(&req_body, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.language.as_deref(), Some("en"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/translation/v1/text/detect"));
}
