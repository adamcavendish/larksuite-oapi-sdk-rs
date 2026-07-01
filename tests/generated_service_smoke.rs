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
    corehr::v1::{
        GetDepartmentQuery as GetCorehrDepartmentQuery, GetEmployeeQuery as GetCorehrEmployeeQuery,
        ListCompanyQuery as ListCorehrCompanyQuery, ListContractQuery as ListCorehrContractQuery,
        ListCountryRegionQuery as ListCorehrCountryRegionQuery,
        ListCurrencyQuery as ListCorehrCurrencyQuery,
        ListDepartmentQuery as ListCorehrDepartmentQuery,
        ListEmployeeQuery as ListCorehrEmployeeQuery,
        ListEmployeeTypeQuery as ListCorehrEmployeeTypeQuery,
        ListJobDataQuery as ListCorehrJobDataQuery, ListJobFamilyQuery as ListCorehrJobFamilyQuery,
        ListJobLevelQuery as ListCorehrJobLevelQuery, ListJobQuery as ListCorehrJobQuery,
        ListLocationQuery as ListCorehrLocationQuery,
        ListNationalIdTypeQuery as ListCorehrNationalIdTypeQuery,
        ListPreHireQuery as ListCorehrPreHireQuery,
        ListSecurityGroupQuery as ListCorehrSecurityGroupQuery,
        ListSubdivisionQuery as ListCorehrSubdivisionQuery,
        ListSubregionQuery as ListCorehrSubregionQuery,
        ListWorkingHoursTypeQuery as ListCorehrWorkingHoursTypeQuery,
        SearchOffboardingQuery as SearchCorehrOffboardingQuery,
    },
    corehr::v2::{
        ListApproverV2Query as ListCorehrApproverV2Query, ListBpV2Query as ListCorehrBpV2Query,
        ListByBizIdSignatureFileV2Query as ListCorehrSignatureFileByBizIdV2Query,
        ListByFileIdSignatureNodeV2Query as ListCorehrSignatureNodeByFileIdV2Query,
        ListJobV2Query as ListCorehrJobV2Query, ListProcessV2Query as ListCorehrProcessV2Query,
        ListSignatureFileV2Query as ListCorehrSignatureFileV2Query,
        ListSignatureTemplateInfoWithThumbnailV2Query as ListCorehrSignatureTemplateInfoWithThumbnailV2Query,
        ListWorkforcePlanV2Query as ListCorehrWorkforcePlanV2Query,
        SearchSignatureTemplateV2Query as SearchCorehrSignatureTemplateV2Query,
    },
    directory::{
        ListCollaborationRuleQuery, ListCollaborationTenantQuery, ListDirectoryUserQuery,
        ListShareEntityQuery,
    },
    drive::{
        v1::{ListFileQuery, ListFileVersionQuery, ListFileViewRecordQuery},
        v2::ListFileLikeQuery,
    },
    hire::v1::{
        GetApplicationQuery as GetHireApplicationQuery, GetJobQuery as GetHireJobQuery,
        GetOfferQuery as GetHireOfferQuery, GetTalentQuery as GetHireTalentQuery,
        ListApplicationInterviewQuery as ListHireApplicationInterviewQuery,
        ListApplicationQuery as ListHireApplicationQuery,
        ListEvaluationTaskQuery as ListHireEvaluationTaskQuery,
        ListExamMarkingTaskQuery as ListHireExamMarkingTaskQuery,
        ListInterviewQuery as ListHireInterviewQuery,
        ListInterviewTaskQuery as ListHireInterviewTaskQuery, ListJobQuery as ListHireJobQuery,
        ListJobRequirementQuery as ListHireJobRequirementQuery,
        ListOfferQuery as ListHireOfferQuery, ListTalentQuery as ListHireTalentQuery,
    },
    meeting_room::v1::{
        ListBuildingQuery as ListMeetingRoomBuildingQuery, ListRoomQuery as ListMeetingRoomQuery,
    },
    minutes::v1::{
        GetMinutesQuery as GetMinutesMinutesQuery, GetTranscriptQuery as GetMinutesTranscriptQuery,
        ListParticipantQuery as ListMinutesParticipantQuery,
    },
    performance::{
        v1::ListActivityQuery as ListPerformanceActivityQuery,
        v2::ListMetricTagV2Query as ListPerformanceMetricTagV2Query,
    },
    report::v1::{
        QueryRuleQuery as QueryReportRuleQuery, QueryTaskQuery as QueryReportTaskQuery,
        QueryTaskReqBody as QueryReportTaskReqBody,
    },
    security_and_compliance::{v1::ListOpenapiLogQuery, v2::ListDeviceRecordV2Query},
    task::{
        CreateTaskReqBody, ListTaskQuery,
        v2::{GetTaskV2Query, ListTaskV2Query},
    },
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

// ── Attendance ──

#[tokio::test]
async fn attendance_file_download_smoke() {
    let body = "attendance-file-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"attendance-file.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .attendance()
        .file
        .download("file-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("attendance-file.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/files/file-1/download"));
}

// ── Baike ──

#[tokio::test]
async fn baike_file_download_smoke() {
    let body = "baike-file-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"baike-file.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .baike()
        .file
        .download("file-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("baike-file.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/baike/v1/files/file-token-1/download"));
}

// ── Board ──

#[tokio::test]
async fn board_whiteboard_download_as_image_smoke() {
    let body = "whiteboard-image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"whiteboard.png\"\r\nContent-Type: image/png\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .board()
        .whiteboard
        .download_as_image("whiteboard-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("whiteboard.png"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/board/v1/whiteboards/whiteboard-1/download_as_image"));
}

// ── EHR ──

#[tokio::test]
async fn ehr_attachment_get_download_smoke() {
    let body = "ehr-attachment-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"attachment.pdf\"\r\nContent-Type: application/pdf\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .ehr()
        .attachment
        .get("attachment-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("attachment.pdf"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/ehr/v1/attachments/attachment-token-1"));
}

// ── Lingo ──

#[tokio::test]
async fn lingo_file_download_smoke() {
    let body = "lingo-file-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"lingo-file.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .lingo()
        .file
        .download("file-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("lingo-file.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/lingo/v1/files/file-token-1/download"));
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
async fn corehr_employee_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"employment":{"id":"emp-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetCorehrEmployeeQuery::new("emp-1").user_id_type("open_id");
    let resp = client
        .corehr()
        .employee
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().employment.unwrap().id.as_deref(),
        Some("emp-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/employments/emp-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn corehr_employee_get_positional_adapter_smoke() {
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

#[tokio::test]
async fn corehr_employee_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"emp-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrEmployeeQuery::new()
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .employee
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("emp-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/employments?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_department_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"department":{"id":"dept-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetCorehrDepartmentQuery::new("dept-1").user_id_type("open_id");
    let resp = client
        .corehr()
        .department
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().department.unwrap().id.as_deref(),
        Some("dept-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/departments/dept-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn corehr_department_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"dept-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrDepartmentQuery::new()
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .department
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("dept-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/departments?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_job_level_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"job-level-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrJobLevelQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .job_level
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("job-level-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/job_levels?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_company_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"company-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrCompanyQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .company
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("company-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/companies?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_location_get_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"location":{"id":"loc-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr()
        .location
        .get("loc-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().location.unwrap().id.as_deref(),
        Some("loc-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/locations/loc-1"));
}

#[tokio::test]
async fn corehr_location_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"loc-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrLocationQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .location
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("loc-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/locations?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_currency_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"currency_id":"CNY"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrCurrencyQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .currency
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].currency_id.as_deref(), Some("CNY"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/currencies?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_working_hours_type_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"hours-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrWorkingHoursTypeQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .working_hours_type
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("hours-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/working_hours_types?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_contract_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"contract-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrContractQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .contract
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("contract-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/contracts?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_country_region_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"country-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrCountryRegionQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .country_region
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("country-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/country_regions?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_employee_type_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"type-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrEmployeeTypeQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .employee_type
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("type-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/employee_types?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_job_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"job-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrJobQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .job
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("job-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/jobs?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_job_data_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"job-data-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrJobDataQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .job_data
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("job-data-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/job_datas?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_job_family_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"job-family-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrJobFamilyQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .job_family
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("job-family-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/job_families?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_national_id_type_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"nid-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrNationalIdTypeQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .national_id_type
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("nid-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/national_id_types?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_offboarding_search_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"offboarding_id":"off-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SearchCorehrOffboardingQuery::new()
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let body = serde_json::json!({"employment_id": "emp-1"});
    let resp = client
        .corehr()
        .offboarding
        .search_by_query(&query, body, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.items.unwrap()[0].offboarding_id.as_deref(),
        Some("off-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/corehr/v1/offboardings/search?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""employment_id":"emp-1""#));
}

#[tokio::test]
async fn corehr_pre_hire_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"pre-hire-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrPreHireQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .pre_hire
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("pre-hire-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/pre_hires?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_security_group_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"security-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSecurityGroupQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .security_group
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("security-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/security_groups?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_subdivision_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"subdivision-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSubdivisionQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .subdivision
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("subdivision-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/subdivisions?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_subregion_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"subregion-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSubregionQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .subregion
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("subregion-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/subregions?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_transfer_reason_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"reason-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr()
        .transfer_reason
        .query(&RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("reason-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/transfer_reasons/query"));
}

#[tokio::test]
async fn corehr_transfer_type_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"type-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr()
        .transfer_type
        .query(&RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("type-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/transfer_types/query"));
}

#[tokio::test]
async fn corehr_v2_approver_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"approver-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrApproverV2Query::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .approver
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("approver-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/approvers?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_bp_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"bp-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrBpV2Query::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .bp
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("bp-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/bps?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_job_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"job-v2-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrJobV2Query::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .job
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("job-v2-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/jobs?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_process_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"process-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrProcessV2Query::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .process
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("process-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/processes?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_signature_file_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"signature-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSignatureFileV2Query::new()
        .signature_file_id("signature-1")
        .states("sign_finished")
        .update_time_start("2026-01-01")
        .update_time_end("2026-12-31")
        .user_id_type("people_corehr_id")
        .template_ids("template-1")
        .select_sign_url(true)
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .signature_file
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("signature-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_files?"));
    assert!(request.contains("signature_file_id=signature-1"));
    assert!(request.contains("states=sign_finished"));
    assert!(request.contains("update_time_start=2026-01-01"));
    assert!(request.contains("update_time_end=2026-12-31"));
    assert!(request.contains("user_id_type=people_corehr_id"));
    assert!(request.contains("template_ids=template-1"));
    assert!(request.contains("select_sign_url=true"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_signature_file_list_by_biz_id_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"signature-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSignatureFileByBizIdV2Query::new()
        .biz_process_id("biz-process-1")
        .biz_type("OpenAPI")
        .user_id_type("people_corehr_id")
        .select_sign_url(true);
    let resp = client
        .corehr_v2()
        .signature_file
        .list_by_biz_id_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("signature-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_files/list_by_biz_id?"));
    assert!(request.contains("biz_process_id=biz-process-1"));
    assert!(request.contains("biz_type=OpenAPI"));
    assert!(request.contains("user_id_type=people_corehr_id"));
    assert!(request.contains("select_sign_url=true"));
}

#[tokio::test]
async fn corehr_v2_signature_node_list_by_file_id_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"node-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSignatureNodeByFileIdV2Query::new()
        .file_id("file-1")
        .user_id_type("people_corehr_id");
    let resp = client
        .corehr_v2()
        .signature_node
        .list_by_file_id_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("node-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_nodes/list_by_file_id?"));
    assert!(request.contains("file_id=file-1"));
    assert!(request.contains("user_id_type=people_corehr_id"));
}

#[tokio::test]
async fn corehr_v2_signature_template_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"template-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SearchCorehrSignatureTemplateV2Query::new()
        .template_ids("template-1")
        .select_custom_field(true);
    let resp = client
        .corehr_v2()
        .signature_template
        .search_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("template-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_templates/search?"));
    assert!(request.contains("template_ids=template-1"));
    assert!(request.contains("select_custom_field=true"));
}

#[tokio::test]
async fn corehr_v2_signature_template_info_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"template-info-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSignatureTemplateInfoWithThumbnailV2Query::new()
        .name("Onboarding")
        .category_apiname("contract_agreement")
        .usage_apiname("general")
        .active(false)
        .need_region_info(true)
        .user_id_type("people_corehr_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .signature_template_info_with_thumbnail
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("template-info-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_template_info_with_thumbnails?"));
    assert!(request.contains("name=Onboarding"));
    assert!(request.contains("category_apiname=contract_agreement"));
    assert!(request.contains("usage_apiname=general"));
    assert!(request.contains("active=false"));
    assert!(request.contains("need_region_info=true"));
    assert!(request.contains("user_id_type=people_corehr_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_workforce_plan_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"workforce-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrWorkforcePlanV2Query::new()
        .limit(10)
        .offset(1)
        .get_all_plan(true)
        .active(true)
        .start_date("2026-01-01")
        .end_date("2026-12-31")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .workforce_plan
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("workforce-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/workforce_plans?"));
    assert!(request.contains("limit=10"));
    assert!(request.contains("offset=1"));
    assert!(request.contains("get_all_plan=true"));
    assert!(request.contains("active=true"));
    assert!(request.contains("start_date=2026-01-01"));
    assert!(request.contains("end_date=2026-12-31"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
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

// ── Minutes ──

#[tokio::test]
async fn minutes_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"minutes":{"minutes_token":"min-1","topic":"Weekly review"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetMinutesMinutesQuery::new("min-1").user_id_type("open_id");
    let resp = client
        .minutes()
        .minutes
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let minutes = resp.data.unwrap().minutes.unwrap();
    assert_eq!(minutes.topic.as_deref(), Some("Weekly review"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/minutes/v1/minutes/min-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn minutes_participant_list_positional_adapter_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"participants":[{"user_id":"u-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .minutes()
        .participant
        .list(
            "min-1",
            Some("open_id"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/minutes/v1/minutes/min-1/participants?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn minutes_participant_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"participants":[{"user_id":"u-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListMinutesParticipantQuery::new("min-1")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .minutes()
        .participant
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.participants.len(), 1);
    assert_eq!(data.participants[0].user_id.as_deref(), Some("u-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/minutes/v1/minutes/min-1/participants?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn minutes_transcript_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"transcript":{"phrases":[{"pid":"p1"}]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetMinutesTranscriptQuery::new("min-1")
        .need_speaker(true)
        .need_timestamp(true)
        .file_format("srt");
    let resp = client
        .minutes()
        .transcript
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let transcript = resp.data.unwrap().transcript.unwrap();
    assert_eq!(transcript.phrases.unwrap().len(), 1);
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/minutes/v1/minutes/min-1/transcript?"));
    assert!(request.contains("need_speaker=true"));
    assert!(request.contains("need_timestamp=true"));
    assert!(request.contains("file_format=srt"));
}

// ── Performance ──

#[tokio::test]
async fn performance_activity_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"activity_id":"act-1","name":"Review"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .performance()
        .activity
        .list(Some(20), Some("next-page"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/performance/v1/activities?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn performance_activity_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"activity_id":"act-1","name":"Review"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListPerformanceActivityQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .performance()
        .activity
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].activity_id.as_deref(), Some("act-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/performance/v1/activities?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn performance_metric_tag_v2_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"tag_id":"tag-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .performance_v2()
        .metric_tag
        .list(Some(20), Some("next-page"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/performance/v2/metric_tags?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn performance_metric_tag_v2_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"tag_id":"tag-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let tag_ids = ["tag-1", "tag-2"];
    let query = ListPerformanceMetricTagV2Query::new()
        .tag_ids(tag_ids.as_slice())
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .performance_v2()
        .metric_tag
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0]["tag_id"].as_str(), Some("tag-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/performance/v2/metric_tags?"));
    assert!(request.contains("tag_ids=tag-1"));
    assert!(request.contains("tag_ids=tag-2"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

// ── Security and Compliance ──

#[tokio::test]
async fn security_openapi_log_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"log-1","api_path":"/open-apis/im/v1/messages"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .security_and_compliance()
        .openapi_log
        .list(
            Some("/open-apis/im/v1/messages"),
            Some("1700000000"),
            Some("1700000100"),
            Some(1),
            Some("ou-1"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/security_and_compliance/v1/openapi_logs?"));
    assert!(request.contains("api_path=%2Fopen-apis%2Fim%2Fv1%2Fmessages"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("operator_type=1"));
    assert!(request.contains("operator_value=ou-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn security_openapi_log_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"log-1","api_path":"/open-apis/im/v1/messages"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListOpenapiLogQuery::new()
        .api_path("/open-apis/im/v1/messages")
        .start_time("1700000000")
        .end_time("1700000100")
        .operator_type(1)
        .operator_value("ou-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .security_and_compliance()
        .openapi_log
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].id.as_deref(), Some("log-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/security_and_compliance/v1/openapi_logs?"));
    assert!(request.contains("api_path=%2Fopen-apis%2Fim%2Fv1%2Fmessages"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("operator_type=1"));
    assert!(request.contains("operator_value=ou-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn security_device_record_list_positional_adapter_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"device_id":"device-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .security_and_compliance_v2()
        .device_record
        .list(Some(20), Some("next-page"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/security_and_compliance/v2/device_records?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn security_device_record_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"device_id":"device-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListDeviceRecordV2Query::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .security_and_compliance_v2()
        .device_record
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0]["device_id"].as_str(), Some("device-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/security_and_compliance/v2/device_records?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

// ── Meeting Room ──

#[tokio::test]
async fn meeting_room_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rooms":[{"room_id":"room-1","name":"Boardroom"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .meeting_room()
        .room
        .list(
            Some("building-1"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/meeting_room/v1/rooms?"));
    assert!(request.contains("building_id=building-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn meeting_room_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rooms":[{"room_id":"room-1","name":"Boardroom"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListMeetingRoomQuery::new()
        .building_id("building-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .meeting_room()
        .room
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.rooms.len(), 1);
    assert_eq!(data.rooms[0].room_id.as_deref(), Some("room-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/meeting_room/v1/rooms?"));
    assert!(request.contains("building_id=building-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn meeting_room_building_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"buildings":[{"building_id":"building-1","name":"HQ"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListMeetingRoomBuildingQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .meeting_room()
        .building
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.buildings.len(), 1);
    assert_eq!(data.buildings[0].building_id.as_deref(), Some("building-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/meeting_room/v1/buildings?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

// ── Report ──

#[tokio::test]
async fn report_rule_query_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rules":[{"rule_id":"rule-1","name":"Weekly"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .report()
        .rule
        .query(
            Some("Weekly"),
            Some(1),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/report/v1/rules/query?"));
    assert!(request.contains("rule_name=Weekly"));
    assert!(request.contains("include_deleted=1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn report_rule_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rules":[{"rule_id":"rule-1","name":"Weekly"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = QueryReportRuleQuery::new()
        .rule_name("Weekly")
        .include_deleted(1)
        .user_id_type("open_id");
    let resp = client
        .report()
        .rule
        .query_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.rules.len(), 1);
    assert_eq!(data.rules[0].rule_id.as_deref(), Some("rule-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/report/v1/rules/query?"));
    assert!(request.contains("rule_name=Weekly"));
    assert!(request.contains("include_deleted=1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn report_task_query_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"task_id":"task-1","rule_id":"rule-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let req_body = QueryReportTaskReqBody {
        rule_id: Some("rule-1".into()),
        user_id: Some("u-1".into()),
        from_time: Some("1700000000".into()),
        to_time: Some("1700000100".into()),
        page_size: Some(20),
        page_token: Some("next-page".into()),
    };
    let resp = client
        .report()
        .task
        .query(&req_body, Some("open_id"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/report/v1/tasks/query?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""rule_id":"rule-1""#));
    assert!(request.contains(r#""user_id":"u-1""#));
    assert!(request.contains(r#""page_size":20"#));
    assert!(request.contains(r#""page_token":"next-page""#));
}

#[tokio::test]
async fn report_task_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"task_id":"task-1","rule_id":"rule-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let req_body = QueryReportTaskReqBody {
        rule_id: Some("rule-1".into()),
        user_id: Some("u-1".into()),
        from_time: Some("1700000000".into()),
        to_time: Some("1700000100".into()),
        page_size: Some(20),
        page_token: Some("next-page".into()),
    };
    let query = QueryReportTaskQuery::new(&req_body).user_id_type("open_id");
    let resp = client
        .report()
        .task
        .query_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].task_id.as_deref(), Some("task-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/report/v1/tasks/query?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""rule_id":"rule-1""#));
    assert!(request.contains(r#""user_id":"u-1""#));
    assert!(request.contains(r#""page_size":20"#));
    assert!(request.contains(r#""page_token":"next-page""#));
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

// ── Task v2 ──

#[tokio::test]
async fn task_v2_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"task":{"guid":"task-guid-1","summary":"Fix bug"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetTaskV2Query::new("task-guid-1").user_id_type("open_id");
    let resp = client
        .task_v2()
        .task
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let task = resp.data.unwrap().task.unwrap();
    assert_eq!(task["guid"].as_str(), Some("task-guid-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/tasks/task-guid-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_list_positional_adapter_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"task-guid-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task_v2()
        .task
        .list(
            Some(20),
            Some("next-page"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/tasks?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"task-guid-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListTaskV2Query::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .completed(false)
        .task_type("my_tasks")
        .agent_task_status(1)
        .user_id_type("open_id");
    let resp = client
        .task_v2()
        .task
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0]["guid"].as_str(), Some("task-guid-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/tasks?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("completed=false"));
    assert!(request.contains("type=my_tasks"));
    assert!(request.contains("agent_task_status=1"));
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

#[tokio::test]
async fn helpdesk_ticket_image_download_smoke() {
    let body = "ticket-image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"ticket-image.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket
        .ticket_image("ticket-1", "msg-1", Some(2), &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("ticket-image.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/ticket_images?"));
    assert!(request.contains("ticket_id=ticket-1"));
    assert!(request.contains("msg_id=msg-1"));
    assert!(request.contains("index=2"));
}

#[tokio::test]
async fn helpdesk_ticket_message_image_download_smoke() {
    let body = "ticket-message-image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"ticket-message-image.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket_message
        .image("ticket-1", "msg-1", Some(3), &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("ticket-message-image.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/ticket_images?"));
    assert!(request.contains("ticket_id=ticket-1"));
    assert!(request.contains("msg_id=msg-1"));
    assert!(request.contains("index=3"));
}

#[tokio::test]
async fn helpdesk_faq_image_download_smoke() {
    let body = "faq-image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"faq-image.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .image("faq-1", "image-key-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("faq-image.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs/faq-1/image/image-key-1"));
}

// ── Hire ──

#[tokio::test]
async fn hire_job_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"job":{"id":"job-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetHireJobQuery::new("job-1")
        .user_id_type("open_id")
        .department_id_type("open_department_id")
        .job_level_id_type("job_level_id")
        .job_family_id_type("job_family_id");
    let resp = client
        .hire()
        .job
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.unwrap().job.unwrap().id.as_deref(), Some("job-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/jobs/job-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("job_family_id_type=job_family_id"));
}

#[tokio::test]
async fn hire_job_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"job-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireJobQuery::new()
        .update_start_time("1618500278663")
        .update_end_time("1618500279999")
        .user_id_type("open_id")
        .department_id_type("open_department_id")
        .job_level_id_type("job_level_id")
        .job_family_id_type("job_family_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .job
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("job-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/jobs?"));
    assert!(request.contains("update_start_time=1618500278663"));
    assert!(request.contains("update_end_time=1618500279999"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("job_family_id_type=job_family_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_talent_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"talent":{"id":"talent-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetHireTalentQuery::new("talent-1").user_id_type("open_id");
    let resp = client
        .hire()
        .talent
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().talent.unwrap().id.as_deref(),
        Some("talent-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/talents/talent-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_talent_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"talent-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireTalentQuery::new()
        .keyword("product")
        .update_start_time("1618500278663")
        .update_end_time("1618500279999")
        .sort_by(1)
        .query_option("ignore_empty_error")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .talent
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("talent-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/talents?"));
    assert!(request.contains("keyword=product"));
    assert!(request.contains("update_start_time=1618500278663"));
    assert!(request.contains("update_end_time=1618500279999"));
    assert!(request.contains("sort_by=1"));
    assert!(request.contains("query_option=ignore_empty_error"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_application_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"application":{"id":"app-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetHireApplicationQuery::new("app-1").user_id_type("open_id");
    let resp = client
        .hire()
        .application
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().application.unwrap().id.as_deref(),
        Some("app-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/app-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_application_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"app-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireApplicationQuery::new()
        .process_id("process-1")
        .job_id("job-1")
        .stage_id("stage-1")
        .talent_id("talent-1")
        .active_status(1)
        .user_id_type("open_id")
        .update_start_time("1618500278663")
        .update_end_time("1618500279999")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .application
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("app-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications?"));
    assert!(request.contains("process_id=process-1"));
    assert!(request.contains("job_id=job-1"));
    assert!(request.contains("stage_id=stage-1"));
    assert!(request.contains("talent_id=talent-1"));
    assert!(request.contains("active_status=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("update_start_time=1618500278663"));
    assert!(request.contains("update_end_time=1618500279999"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_application_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"app-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .hire()
        .application
        .list(
            Some(20),
            Some("next-page"),
            Some("job-1"),
            Some("stage-1"),
            Some("talent-1"),
            Some(1),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("app-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications?"));
    assert!(request.contains("job_id=job-1"));
    assert!(request.contains("stage_id=stage-1"));
    assert!(request.contains("talent_id=talent-1"));
    assert!(request.contains("active_status=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_interview_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"interview-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireInterviewQuery::new()
        .application_id("app-1")
        .interview_id("interview-1")
        .start_time("1609489908000")
        .end_time("1610489908000")
        .user_id_type("open_id")
        .job_level_id_type("job_level_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .interview
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("interview-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/interviews?"));
    assert!(request.contains("application_id=app-1"));
    assert!(request.contains("interview_id=interview-1"));
    assert!(request.contains("start_time=1609489908000"));
    assert!(request.contains("end_time=1610489908000"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_offer_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"offer":{"id":"offer-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetHireOfferQuery::new("offer-1")
        .user_id_type("open_id")
        .department_id_type("open_department_id")
        .job_level_id_type("job_level_id")
        .job_family_id_type("job_family_id")
        .employee_type_id_type("employee_type_id");
    let resp = client
        .hire()
        .offer
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().offer.unwrap().id.as_deref(),
        Some("offer-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/offers/offer-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("job_family_id_type=job_family_id"));
    assert!(request.contains("employee_type_id_type=employee_type_id"));
}

#[tokio::test]
async fn hire_offer_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"offer-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireOfferQuery::new()
        .application_id("app-1")
        .talent_id("talent-1")
        .user_id_type("open_id")
        .employee_type_id_type("employee_type_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .offer
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("offer-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/offers?"));
    assert!(request.contains("application_id=app-1"));
    assert!(request.contains("talent_id=talent-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("employee_type_id_type=employee_type_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_job_requirement_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"requirement-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireJobRequirementQuery::new()
        .job_id("job-1")
        .create_time_begin("1658980233000")
        .create_time_end("1658980233999")
        .update_time_begin("1658980234000")
        .update_time_end("1658980234999")
        .user_id_type("open_id")
        .department_id_type("open_department_id")
        .job_level_id_type("job_level_id")
        .job_family_id_type("job_family_id")
        .employee_type_id_type("employee_type_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .job_requirement
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("requirement-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/job_requirements?"));
    assert!(request.contains("job_id=job-1"));
    assert!(request.contains("create_time_begin=1658980233000"));
    assert!(request.contains("create_time_end=1658980233999"));
    assert!(request.contains("update_time_begin=1658980234000"));
    assert!(request.contains("update_time_end=1658980234999"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("job_family_id_type=job_family_id"));
    assert!(request.contains("employee_type_id_type=employee_type_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_job_requirement_list_positional_adapter_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"requirement-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .hire()
        .job_requirement
        .list(
            Some(20),
            Some("next-page"),
            Some("job-1"),
            Some("1658980233000"),
            Some("1658980233999"),
            Some("1658980234000"),
            Some("1658980234999"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("requirement-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/job_requirements?"));
    assert!(request.contains("job_id=job-1"));
    assert!(request.contains("create_time_begin=1658980233000"));
    assert!(request.contains("create_time_end=1658980233999"));
    assert!(request.contains("update_time_begin=1658980234000"));
    assert!(request.contains("update_time_end=1658980234999"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_application_interview_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"app-interview-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireApplicationInterviewQuery::new("app-1")
        .user_id_type("open_id")
        .job_level_id_type("job_level_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .application_interview
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("app-interview-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/app-1/interviews?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_evaluation_task_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"evaluation-task-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireEvaluationTaskQuery::new()
        .user_id("ou_user_1")
        .activity_status(1)
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .evaluation_task
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("evaluation-task-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/evaluation_tasks?"));
    assert!(request.contains("user_id=ou_user_1"));
    assert!(request.contains("activity_status=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_exam_marking_task_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"exam-task-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireExamMarkingTaskQuery::new()
        .user_id("ou_user_1")
        .activity_status(1)
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .exam_marking_task
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("exam-task-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/exam_marking_tasks?"));
    assert!(request.contains("user_id=ou_user_1"));
    assert!(request.contains("activity_status=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_interview_task_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"interview-task-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireInterviewTaskQuery::new()
        .user_id("ou_user_1")
        .activity_status(1)
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .interview_task
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("interview-task-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/interview_tasks?"));
    assert!(request.contains("user_id=ou_user_1"));
    assert!(request.contains("activity_status=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
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
