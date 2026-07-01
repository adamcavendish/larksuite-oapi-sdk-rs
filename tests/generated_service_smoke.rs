mod common;

use common::{http_response, http_response_with_headers, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::{
    acs::v1::{
        GetUserQuery as GetAcsUserQuery, ListAccessRecordQuery as ListAcsAccessRecordQuery,
        ListUserQuery as ListAcsUserQuery,
    },
    admin::v1::{
        GetAdminDeptStatQuery, GetBadgeGrantQuery, GetBadgeQuery, ListAdminDeptStatQuery,
        ListAdminUserStatQuery, ListAuditInfoQuery, ListBadgeGrantQuery, ListBadgeQuery,
    },
    application::v6::{
        ApplicationContactsRangeConfigurationQuery, ContactsRangeSuggestApplicationAppVersionQuery,
        GetApplicationAppVersionQuery as GetApplicationV6AppVersionQuery,
        GetApplicationCollaboratorsQuery, GetApplicationQuery as GetApplicationV6Query,
        ListAppRecommendRuleQuery, ListApplicationAppVersionQuery, ListApplicationFeedbackQuery,
        ListApplicationQuery as ListApplicationV6Query, UnderauditlistApplicationQuery,
    },
    approval::v4::{
        CcSearch, GetApprovalQuery, GetExternalApprovalQuery, GetInstanceQuery, InstanceSearch,
        ListExternalTaskQuery, ListExternalTaskReqBody, ListInstanceCommentQuery,
        ListInstanceQuery, QueryInstanceQuery, QueryTaskQuery as QueryApprovalTaskQuery,
        SearchCcInstanceQuery, SearchTaskQuery, TaskSearch,
    },
    attendance::v1::{
        AttendanceGroup, BatchGetUserSettingQuery, BatchGetUserSettingReqBody,
        CreateGroupQuery as CreateAttendanceGroupQuery, CreateOrUpdateGroupReqBody,
        GetApprovalInfoQuery as GetAttendanceApprovalInfoQuery, GetApprovalInfoReqBody,
        GetGroupQuery as GetAttendanceGroupQuery, ListArchiveRuleQuery,
        ListGroupQuery as ListAttendanceGroupQuery, ListLeaveAccrualQuery,
        ListShiftQuery as ListAttendanceShiftQuery, ListUserGroupQuery,
        PatchLeaveAccrualRecordQuery, PatchLeaveAccrualRecordReqBody,
        QueryRecordQuery as QueryAttendanceRecordQuery, QueryRecordReqBody,
    },
    bitable::v1::{
        ListAppRoleMemberQuery, ListAppRoleQuery, ListAppWorkflowQuery, ListDashboardQuery,
        ListFieldQuery, ListFormFieldQuery, ListRecordQuery, ListTableQuery, ListViewQuery,
        SearchRecordQuery, SearchRecordReqBody,
    },
    calendar::v4::{
        BatchFreeBusyQuery, BatchFreeBusyReqBody, GetCalendarQuery,
        GetEventQuery as GetCalendarEventQuery, InstanceViewEventQuery, InstancesEventQuery,
        ListAclQuery as ListCalendarAclQuery, ListAttendeeQuery, ListCalendarQuery,
        ListChatMemberQuery as ListCalendarChatMemberQuery,
        ListEventQuery as ListCalendarEventQuery, ListFreeBusyQuery, ListFreeBusyReqBody,
        ListTimeZoneQuery, SearchCalendarQuery, SearchCalendarReqBody,
        SearchEventQuery as SearchCalendarEventQuery, SearchEventReqBody,
    },
    common::PageQuery,
    contact::v3::{
        BatchCreateFunctionalRoleMemberQuery, BatchCreateFunctionalRoleMemberReqBody,
        BatchDeleteFunctionalRoleMemberQuery, BatchDeleteFunctionalRoleMemberReqBody,
        BatchDepartmentQuery as BatchContactDepartmentQuery, BatchGetIdUserQuery,
        BatchGetIdUserReqBody, BatchUserQuery as BatchContactUserQuery, ChildrenDepartmentQuery,
        FindUserByDepartmentQuery, GetDepartmentQuery as GetContactDepartmentQuery,
        GetFunctionalRoleMemberQuery as GetContactFunctionalRoleMemberQuery,
        GetGroupQuery as GetContactGroupQuery, GetUserQuery as GetContactUserQuery,
        ListCustomAttrQuery, ListDepartmentQuery, ListDepartmentUnitQuery,
        ListEmployeeTypeEnumQuery,
        ListFunctionalRoleMemberQuery as ListContactFunctionalRoleMemberQuery,
        ListJobFamilyQuery as ListContactJobFamilyQuery,
        ListJobLevelQuery as ListContactJobLevelQuery,
        ListJobTitleQuery as ListContactJobTitleQuery, ListScopeQuery, ListUnitQuery,
        ListUserQuery, ListWorkCityQuery, MemberBelongGroupQuery, ResurrectUserQuery,
        ResurrectUserReqBody, ScopesFunctionalRoleMemberQuery, ScopesFunctionalRoleMemberReqBody,
        SearchDepartmentQuery, SearchDepartmentReqBody, SimplelistGroupMemberQuery,
        SimplelistGroupQuery, UnbindDepartmentChatQuery, UnbindDepartmentChatReqBody,
        UserDepartmentInfo,
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
        CreateDepartmentQuery as CreateDirectoryDepartmentQuery, CreateDirectoryEmployeeQuery,
        ListCollaborationRuleQuery, ListCollaborationTenantQuery, ListDirectoryUserQuery,
        ListShareEntityQuery, MgetEmployeeQuery as MgetDirectoryEmployeeQuery,
        PatchEmployeeQuery as PatchDirectoryEmployeeQuery, SearchDirectoryDepartmentQuery,
    },
    docs::v1::GetContentQuery as GetDocsContentQuery,
    drive::{
        v1::{
            BatchQueryFileCommentQuery, BatchQueryFileCommentReqBody, GetFileCommentQuery,
            GetPermissionPublicQuery as GetDrivePermissionPublicQuery, ListFileCommentQuery,
            ListFileCommentReplyQuery, ListFileQuery, ListFileVersionQuery,
            ListFileViewRecordQuery, ListPermissionMemberQuery,
        },
        v2::{GetPermissionPublicV2Query as GetDrivePermissionPublicV2Query, ListFileLikeQuery},
    },
    ehr::v1::ListEmployeeQuery as ListEhrEmployeeQuery,
    helpdesk::v1::{
        CustomizedFieldsTicketQuery, GetAgentSchedulesQuery, GetCategoryQuery,
        GetFaqQuery as GetHelpdeskFaqQuery, GetTicketQuery, ListAgentQuery, ListAgentScheduleQuery,
        ListCategoryQuery, ListFaqQuery, ListTicketMessageQuery, ListTicketQuery, SearchFaqQuery,
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
    im::v1::{
        GetChatAnnouncementQuery, GetChatMembersQuery, GetChatModerationQuery,
        GetChatQuery as GetImChatQuery, GetMessageQuery as GetImMessageQuery, IsInChatMembersQuery,
        ListChatQuery as ListImChatQuery, ListMessageQuery as ListImMessageQuery,
        ListMessageReactionQuery, ListPinQuery, ReadUsersMessageQuery,
        SearchChatQuery as SearchImChatQuery,
    },
    lingo::v1::{
        GetEntityQuery as GetLingoEntityQuery,
        ListClassificationQuery as ListLingoClassificationQuery,
        ListEntityQuery as ListLingoEntityQuery, SearchEntityQuery as SearchLingoEntityQuery,
        SearchLingoEntityReqBody,
    },
    mail::v1::{
        ListMailgroupMemberQuery as ListMailMailgroupMemberQuery,
        ListMailgroupQuery as ListMailMailgroupQuery,
        ListPublicMailboxMemberQuery as ListMailPublicMailboxMemberQuery,
        ListPublicMailboxQuery as ListMailPublicMailboxQuery,
    },
    meeting_room::v1::{
        ListBuildingQuery as ListMeetingRoomBuildingQuery, ListRoomQuery as ListMeetingRoomQuery,
    },
    minutes::v1::{
        GetMinutesQuery as GetMinutesMinutesQuery, GetTranscriptQuery as GetMinutesTranscriptQuery,
        ListParticipantQuery as ListMinutesParticipantQuery,
    },
    payroll::v1::{
        ListCostAllocationDetailQuery, ListPaymentActivityDetailQuery, ListPaymentActivityQuery,
        ListPayrollRecordQuery, QueryDatasourceRecordQuery, QueryPaymentDetailQuery,
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
        v2::{
            AddCustomFieldV2Query, AddMembersTaskV2Query, AddMembersTasklistV2Query,
            CreateActivitySubscriptionV2Query, CreateCommentV2Query,
            CreateCustomFieldOptionV2Query, CreateCustomFieldV2Query, CreateSectionV2Query,
            CreateTaskSubtaskV2Query, CreateTaskV2Query, CreateTasklistV2Query,
            GetActivitySubscriptionV2Query, GetAttachmentV2Query, GetCommentV2Query,
            GetCustomFieldV2Query, GetSectionV2Query, GetTaskV2Query, GetTasklistV2Query,
            ListActivitySubscriptionV2Query, ListAttachmentV2Query, ListCommentV2Query,
            ListCustomFieldV2Query, ListSectionV2Query, ListTaskSubtaskV2Query, ListTaskV2Query,
            ListTasklistV2Query, PatchActivitySubscriptionV2Query, PatchCommentV2Query,
            PatchCustomFieldOptionV2Query, PatchCustomFieldV2Query, PatchSectionV2Query,
            PatchTaskV2Query, PatchTasklistV2Query, RemoveCustomFieldV2Query,
            RemoveMembersTasklistV2Query, TaskListParams, TasklistsTaskV2Query,
            TasksSectionV2Query, TasksTasklistV2Query, UploadAttachmentV2Query,
        },
    },
    vc::{
        ApplyReserveQuery, CreateRoomQuery as CreateVcRoomQuery, CreateRoomReqBody,
        GetActiveMeetingReserveQuery, GetMeetingListQuery, GetParticipantListQuery,
        GetParticipantQualityListQuery, GetReserveQuery, GetResourceReservationListQuery,
        GetRoomQuery as GetVcRoomQuery, GetTopUserReportQuery, ListAlertQuery,
        ListMeetingByNoQuery, ListRoomQuery as ListVcRoomQuery, MgetRoomQuery, RoomConfigQuery,
        SearchRoomQuery, SetRoomConfigQuery, SetRoomConfigReqBody, UpdateReserveQuery,
        UpdateRoomQuery as UpdateVcRoomQuery, UpdateRoomReqBody,
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
async fn attendance_group_create_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"group":{"group_id":"group-1","group_name":"Engineering"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = CreateOrUpdateGroupReqBody {
        group_id: Some("group-1".to_string()),
        operator_id: Some("u-1".to_string()),
        group: Some(AttendanceGroup {
            group_name: Some("Engineering".to_string()),
            ..Default::default()
        }),
    };
    let resp = client
        .attendance()
        .group
        .create_by_query(
            &CreateAttendanceGroupQuery::new(&body, "employee_id").dept_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.group.as_ref())
            .and_then(|group| group.group_name.as_deref()),
        Some("Engineering")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/attendance/v1/groups?"));
    assert!(request.contains("employee_type=employee_id"));
    assert!(request.contains("dept_type=open_department_id"));
    assert!(request.contains(r#""group_id":"group-1""#));
    assert!(request.contains(r#""group_name":"Engineering""#));
}

#[tokio::test]
async fn attendance_group_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"group":{"group_id":"group-1","group_name":"Engineering"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .attendance()
        .group
        .get_by_query(
            &GetAttendanceGroupQuery::new("group-1", "employee_id").dept_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.group.as_ref())
            .and_then(|group| group.group_id.as_deref()),
        Some("group-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/groups/group-1?"));
    assert!(request.contains("employee_type=employee_id"));
    assert!(request.contains("dept_type=open_department_id"));
}

#[tokio::test]
async fn attendance_group_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"group_list":[{"group_id":"group-1","group_name":"Engineering"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListAttendanceGroupQuery::new()
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .attendance()
        .group
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.group_list.first())
            .and_then(|group| group.get("group_id"))
            .and_then(|group_id| group_id.as_str()),
        Some("group-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/groups?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn attendance_group_list_user_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"u-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListUserGroupQuery::new("group-1")
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .attendance()
        .group
        .list_user_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.get("items"))
            .and_then(|items| items.as_array())
            .and_then(|items| items.first())
            .and_then(|item| item.get("user_id"))
            .and_then(|user_id| user_id.as_str()),
        Some("u-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/groups/group-1/list_user?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn attendance_shift_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"shift_id":"shift-1","shift_name":"Day"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListAttendanceShiftQuery::new()
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .attendance()
        .shift
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.get("items"))
            .and_then(|items| items.as_array())
            .and_then(|items| items.first())
            .and_then(|item| item.get("shift_id"))
            .and_then(|shift_id| shift_id.as_str()),
        Some("shift-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/shifts?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn attendance_user_setting_batch_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"user_settings":[{"user_id":"u-1","face_key":"face-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = BatchGetUserSettingReqBody {
        user_ids: Some(vec!["u-1".to_string()]),
    };
    let resp = client
        .attendance()
        .user_setting
        .batch_get_by_query(
            &BatchGetUserSettingQuery::new(&body, "employee_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.user_settings.first())
            .and_then(|setting| setting.face_key.as_deref()),
        Some("face-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/user_settings/query?"));
    assert!(request.contains("employee_type=employee_id"));
    assert!(request.contains(r#""user_ids":["u-1"]"#));
}

#[tokio::test]
async fn attendance_record_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"user_datas":[{"user_id":"u-1","date":20260702}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = QueryRecordReqBody {
        user_ids: Some(vec!["u-1".to_string()]),
        check_date_from: Some(20260701),
        check_date_to: Some(20260702),
        ..Default::default()
    };
    let resp = client
        .attendance()
        .record
        .query_by_query(
            &QueryAttendanceRecordQuery::new(&body, "employee_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.user_datas.first())
            .and_then(|record| record.user_id.as_deref()),
        Some("u-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/attendance/v1/user_tasks/query?"));
    assert!(request.contains("employee_type=employee_id"));
    assert!(request.contains(r#""check_date_from":20260701"#));
}

#[tokio::test]
async fn attendance_approval_info_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"user_datas":[{"approval_id":"approval-1","user_id":"u-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = GetApprovalInfoReqBody {
        user_ids: Some(vec!["u-1".to_string()]),
        check_date_from: Some("2026-07-01".to_string()),
        check_date_to: Some("2026-07-02".to_string()),
        page_size: Some(20),
        page_token: Some("next-page".to_string()),
        ..Default::default()
    };
    let resp = client
        .attendance()
        .approval_info
        .get_by_query(
            &GetAttendanceApprovalInfoQuery::new(&body, "employee_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.user_datas.first())
            .and_then(|approval| approval.approval_id.as_deref()),
        Some("approval-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/attendance/v1/user_approvals/query?"));
    assert!(request.contains("employee_type=employee_id"));
    assert!(request.contains(r#""page_token":"next-page""#));
}

#[tokio::test]
async fn attendance_leave_accrual_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"records":[{"id":"leave-1","employment_id":"emp-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListLeaveAccrualQuery::new("emp-1")
        .leave_type_id("leave-type-1")
        .accrual_date_from("2026-07-01")
        .accrual_date_to("2026-07-02")
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .attendance()
        .leave_accrual
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.records.first())
            .and_then(|record| record.id.as_deref()),
        Some("leave-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/leave_accrual_record?"));
    assert!(request.contains("employment_id=emp-1"));
    assert!(request.contains("leave_type_id=leave-type-1"));
    assert!(request.contains("accrual_date_from=2026-07-01"));
    assert!(request.contains("accrual_date_to=2026-07-02"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn attendance_archive_rule_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"archive_rule_id":"rule-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListArchiveRuleQuery::new()
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .attendance()
        .archive_rule
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.get("items"))
            .and_then(|items| items.as_array())
            .and_then(|items| items.first())
            .and_then(|item| item.get("archive_rule_id"))
            .and_then(|rule_id| rule_id.as_str()),
        Some("rule-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/archive_rule?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn attendance_leave_accrual_record_patch_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"leave_granting_record_id":"leave-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = PatchLeaveAccrualRecordReqBody {
        leave_granting_record_id: Some("leave-1".to_string()),
        employment_id: Some("emp-1".to_string()),
        quantity: Some("8".to_string()),
        ..Default::default()
    };
    let resp = client
        .attendance()
        .leave_accrual_record
        .patch_by_query(
            &PatchLeaveAccrualRecordQuery::new("leave-1", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/attendance/v1/leave_accrual_record/leave-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""employment_id":"emp-1""#));
    assert!(request.contains(r#""quantity":"8""#));
}

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

#[tokio::test]
async fn ehr_employee_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let statuses = [2, 4];
    let resp = client
        .ehr()
        .employee
        .list_by_query(
            &ListEhrEmployeeQuery::new()
                .user_id_type("open_id")
                .view("basic")
                .status(&statuses[..])
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|employee| employee.user_id.as_deref()),
        Some("ou-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/ehr/v1/employees?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("view=basic"));
    assert!(request.contains("status=2"));
    assert!(request.contains("status=4"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
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

#[tokio::test]
async fn lingo_entity_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"entity":{"id":"entity-1","description":"term"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .lingo()
        .entity
        .get_by_query(
            &GetLingoEntityQuery::new("entity-1")
                .provider("provider-1")
                .outer_id("outer-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.entity.as_ref())
            .and_then(|entity| entity.id.as_deref()),
        Some("entity-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/lingo/v1/entities/entity-1?"));
    assert!(request.contains("provider=provider-1"));
    assert!(request.contains("outer_id=outer-1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn lingo_entity_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"entities":[{"id":"entity-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .lingo()
        .entity
        .list_by_query(
            &ListLingoEntityQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .repo_id("repo-1")
                .provider("provider-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.entities.first())
            .and_then(|entity| entity.id.as_deref()),
        Some("entity-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/lingo/v1/entities?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("repo_id=repo-1"));
    assert!(request.contains("provider=provider-1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn lingo_entity_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"entities":[{"id":"entity-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = SearchLingoEntityReqBody {
        query: Some("term".to_string()),
        ..Default::default()
    };
    let resp = client
        .lingo()
        .entity
        .search_by_query(
            &SearchLingoEntityQuery::new(&body)
                .repo_id("repo-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.entities.first())
            .and_then(|entity| entity.id.as_deref()),
        Some("entity-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/lingo/v1/entities/search?"));
    assert!(request.contains("repo_id=repo-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""query":"term""#));
}

#[tokio::test]
async fn lingo_classification_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"class-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .lingo()
        .classification
        .list_by_query(
            &ListLingoClassificationQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert!(
        resp.data
            .as_ref()
            .and_then(|data| data.get("items"))
            .is_some()
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/lingo/v1/classifications?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

// ── Mail ──

#[tokio::test]
async fn mail_mailgroup_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"mailgroup_id":"mg-1","email":"group@example.com"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .mail()
        .mailgroup
        .list_by_query(
            &ListMailMailgroupQuery::new()
                .manager_user_id("ou-1")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|mailgroup| mailgroup.mailgroup_id.as_deref()),
        Some("mg-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/mail/v1/mailgroups?"));
    assert!(request.contains("manager_user_id=ou-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn mail_mailgroup_member_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id":"member-1","user_id":"ou-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .mail()
        .mailgroup_member
        .list_by_query(
            &ListMailMailgroupMemberQuery::new("mg-1")
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|member| member.member_id.as_deref()),
        Some("member-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/mail/v1/mailgroups/mg-1/members?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn mail_public_mailbox_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"public_mailbox_id":"pm-1","email":"public@example.com"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .mail()
        .public_mailbox
        .list_by_query(
            &ListMailPublicMailboxQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|mailbox| mailbox.public_mailbox_id.as_deref()),
        Some("pm-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/mail/v1/public_mailboxes?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn mail_public_mailbox_member_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id":"member-1","user_id":"ou-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .mail()
        .public_mailbox_member
        .list_by_query(
            &ListMailPublicMailboxMemberQuery::new("pm-1")
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|member| member.member_id.as_deref()),
        Some("member-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/mail/v1/public_mailboxes/pm-1/members?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
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
async fn im_get_message_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"message_id":"om_1","chat_id":"oc_1","body":{"content":"hello"}}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .message
        .get_by_query(
            &GetImMessageQuery::new("om_1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|message| message.body.as_ref())
            .and_then(|body| body.content.as_deref()),
        Some("hello")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/messages/om_1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn im_list_message_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"message_id":"om_1","chat_id":"oc_1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .message
        .list_by_query(
            &ListImMessageQuery::new("chat", "oc_1")
                .start_time("1782910000")
                .end_time("1782913600")
                .sort_type("ByCreateTimeAsc")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/messages?"));
    assert!(request.contains("container_id_type=chat"));
    assert!(request.contains("container_id=oc_1"));
    assert!(request.contains("start_time=1782910000"));
    assert!(request.contains("end_time=1782913600"));
    assert!(request.contains("sort_type=ByCreateTimeAsc"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn im_read_users_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id_type":"open_id","user_id":"ou_1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .message
        .read_users_by_query(
            &ReadUsersMessageQuery::new("om_1", "open_id")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/messages/om_1/read_users?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn im_list_message_reaction_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"reaction_id":"reaction-1","reaction_type":{"emoji_type":"OK"}}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .message_reaction
        .list_by_query(
            &ListMessageReactionQuery::new("om_1")
                .reaction_type("OK")
                .page_size(20)
                .page_token("next-page")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/messages/om_1/reactions?"));
    assert!(request.contains("reaction_type=OK"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn im_list_pin_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"message_id":"om_1","chat_id":"oc_1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .pin
        .list_by_query(
            &ListPinQuery::new("oc_1")
                .start_time("1782910000")
                .end_time("1782913600")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/pins?"));
    assert!(request.contains("chat_id=oc_1"));
    assert!(request.contains("start_time=1782910000"));
    assert!(request.contains("end_time=1782913600"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn im_get_chat_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"chat_id":"oc_1","name":"team"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat
        .get_by_query(
            &GetImChatQuery::new("oc_1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.unwrap().name.as_deref(), Some("team"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/oc_1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn im_list_chat_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"chat_id":"oc_1","name":"team"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat
        .list_by_query(
            &ListImChatQuery::new()
                .user_id_type("open_id")
                .sort_type("ByCreateTimeAsc")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("sort_type=ByCreateTimeAsc"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn im_search_chat_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"chat_id":"oc_1","name":"team"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat
        .search_by_query(
            &SearchImChatQuery::new()
                .user_id_type("open_id")
                .query("team")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/search?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("query=team"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn im_get_chat_members_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id_type":"open_id","member_id":"ou_1","name":"Alice"}],"has_more":false,"member_total":1}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat_members
        .get_by_query(
            &GetChatMembersQuery::new("oc_1")
                .member_id_type("open_id")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/oc_1/members?"));
    assert!(request.contains("member_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn im_is_in_chat_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"is_in_chat":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat_members
        .is_in_chat_by_query(
            &IsInChatMembersQuery::new("oc_1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.unwrap().is_in_chat, Some(true));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/oc_1/members/is_in_chat"));
}

#[tokio::test]
async fn im_get_chat_announcement_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"content":"notice","revision":"1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat_announcement
        .get_by_query(
            &GetChatAnnouncementQuery::new("oc_1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.unwrap().content.as_deref(), Some("notice"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/oc_1/announcement?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn im_get_chat_moderation_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"moderation_setting":"all_members","items":[{"user_id_type":"open_id","user_id":"ou_1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .chat_moderation
        .get_by_query(
            &GetChatModerationQuery::new("oc_1")
                .user_id_type("open_id")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/chats/oc_1/moderation?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
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
async fn contact_department_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"department":{"department_id":"od-1","name":"Engineering"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .department
        .get_by_query(
            &GetContactDepartmentQuery::new("od-1")
                .user_id_type("open_id")
                .department_id_type("department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.department.as_ref())
            .and_then(|department| department.name.as_deref()),
        Some("Engineering")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/departments/od-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
}

#[tokio::test]
async fn contact_department_batch_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-1","name":"Engineering"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let department_ids = ["od-1", "od-2"];
    let resp = client
        .contact()
        .department
        .batch_by_query(
            &BatchContactDepartmentQuery::new(&department_ids)
                .department_id_type("department_id")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/departments/batch?"));
    assert!(request.contains("department_ids=od-1"));
    assert!(request.contains("department_ids=od-2"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn contact_department_unbind_chat_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = UnbindDepartmentChatReqBody {
        department_id: Some("od-1".to_string()),
    };
    let resp = client
        .contact()
        .department
        .unbind_department_chat_by_query(
            &UnbindDepartmentChatQuery::new(&body).department_id_type("department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/contact/v3/departments/unbind_department_chat?"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains(r#""department_id":"od-1""#));
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
async fn contact_user_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"user":{"user_id":"u-1","name":"Ada"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .user
        .get_by_query(
            &GetContactUserQuery::new("u-1")
                .user_id_type("open_id")
                .department_id_type("department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.user.as_ref())
            .and_then(|user| user.name.as_deref()),
        Some("Ada")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/users/u-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
}

#[tokio::test]
async fn contact_user_batch_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"u-1","name":"Ada"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let user_ids = ["u-1", "u-2"];
    let resp = client
        .contact()
        .user
        .batch_by_query(
            &BatchContactUserQuery::new(&user_ids)
                .user_id_type("open_id")
                .department_id_type("department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/users/batch?"));
    assert!(request.contains("user_ids=u-1"));
    assert!(request.contains("user_ids=u-2"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
}

#[tokio::test]
async fn contact_user_batch_get_id_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"user_list":[{"user_id":"u-1","email":"ada@example.com"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = BatchGetIdUserReqBody {
        emails: Some(vec!["ada@example.com".to_string()]),
        mobiles: None,
        include_resigned: Some(true),
    };
    let resp = client
        .contact()
        .user
        .batch_get_id_by_query(
            &BatchGetIdUserQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/contact/v3/users/batch_get_id?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""emails":["ada@example.com"]"#));
    assert!(request.contains(r#""include_resigned":true"#));
}

#[tokio::test]
async fn contact_user_resurrect_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = ResurrectUserReqBody {
        departments: Some(vec![UserDepartmentInfo {
            department_id: Some("od-1".to_string()),
        }]),
        subscription_ids: Some(vec!["sub-1".to_string()]),
    };
    let resp = client
        .contact()
        .user
        .resurrect_by_query(
            &ResurrectUserQuery::new("u-1", &body)
                .user_id_type("open_id")
                .department_id_type("department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/contact/v3/users/u-1/resurrect?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains(r#""department_id":"od-1""#));
    assert!(request.contains(r#""subscription_ids":["sub-1"]"#));
}

#[tokio::test]
async fn contact_unit_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"unitlist":[{"unit_id":"unit-1","name":"Engineering Unit"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListUnitQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .unit
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.unitlist.as_ref())
            .and_then(|items| items.first())
            .and_then(|unit| unit.name.as_deref()),
        Some("Engineering Unit")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/unit?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_unit_list_department_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"departmentlist":[{"unit_id":"unit-1","department_id":"od-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListDepartmentUnitQuery::new("unit-1")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .unit
        .list_department_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.departmentlist.as_ref())
            .and_then(|items| items.first())
            .and_then(|department| department.department_id.as_deref()),
        Some("od-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/unit/list_department?"));
    assert!(request.contains("unit_id=unit-1"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_group_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"group":{"group_id":"group-1","name":"Engineering"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .group
        .get_by_query(
            &GetContactGroupQuery::new("group-1")
                .user_id_type("open_id")
                .department_id_type("department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.group.as_ref())
            .and_then(|group| group.name.as_deref()),
        Some("Engineering")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/group/group-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
}

#[tokio::test]
async fn contact_group_simplelist_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"grouplist":[{"group_id":"group-1","name":"Engineering"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SimplelistGroupQuery::new()
        .group_type(1)
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .group
        .simplelist_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.grouplist.as_ref())
            .and_then(|items| items.first())
            .and_then(|group| group.name.as_deref()),
        Some("Engineering")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/group/simplelist?"));
    assert!(request.contains("type=1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_group_member_belong_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"group_list":["group-1"],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = MemberBelongGroupQuery::new("u-1")
        .member_id_type("open_id")
        .group_type(1)
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .group
        .member_belong_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.group_list.as_ref())
            .and_then(|groups| groups.first())
            .map(String::as_str),
        Some("group-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/group/member_belong?"));
    assert!(request.contains("member_id=u-1"));
    assert!(request.contains("member_id_type=open_id"));
    assert!(request.contains("group_type=1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_group_member_simplelist_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"memberlist":[{"member_id":"u-1","member_type":"user","member_id_type":"open_id"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SimplelistGroupMemberQuery::new("group-1")
        .member_id_type("open_id")
        .member_type("user")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .group_member
        .simplelist_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.memberlist.as_ref())
            .and_then(|members| members.first())
            .and_then(|member| member.member_id.as_deref()),
        Some("u-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/group/group-1/member/simplelist?"));
    assert!(request.contains("member_id_type=open_id"));
    assert!(request.contains("member_type=user"));
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

#[tokio::test]
async fn contact_functional_role_member_batch_create_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"results":[{"user_id":"u-1","reason":0}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = BatchCreateFunctionalRoleMemberReqBody {
        members: Some(vec!["u-1".to_string()]),
    };
    let resp = client
        .contact()
        .functional_role_member
        .batch_create_by_query(
            &BatchCreateFunctionalRoleMemberQuery::new("role-1", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.results.as_ref())
            .and_then(|results| results.first())
            .and_then(|result| result.user_id.as_deref()),
        Some("u-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request
            .contains("POST /open-apis/contact/v3/functional_roles/role-1/members/batch_create?")
    );
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""members":["u-1"]"#));
}

#[tokio::test]
async fn contact_functional_role_member_batch_delete_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"result":[{"user_id":"u-1","reason":0}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = BatchDeleteFunctionalRoleMemberReqBody {
        members: Some(vec!["u-1".to_string()]),
    };
    let resp = client
        .contact()
        .functional_role_member
        .batch_delete_by_query(
            &BatchDeleteFunctionalRoleMemberQuery::new("role-1", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.result.as_ref())
            .and_then(|results| results.first())
            .and_then(|result| result.user_id.as_deref()),
        Some("u-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request
            .contains("PATCH /open-apis/contact/v3/functional_roles/role-1/members/batch_delete?")
    );
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""members":["u-1"]"#));
}

#[tokio::test]
async fn contact_functional_role_member_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"member":{"user_id":"u-1","scope_type":"department","department_ids":["od-1"]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .functional_role_member
        .get_by_query(
            &GetContactFunctionalRoleMemberQuery::new("role-1", "u-1")
                .user_id_type("open_id")
                .department_id_type("department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.member.as_ref())
            .and_then(|member| member.user_id.as_deref()),
        Some("u-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/functional_roles/role-1/members/u-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
}

#[tokio::test]
async fn contact_functional_role_member_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"members":[{"user_id":"u-1","scope_type":"department","department_ids":["od-1"]}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListContactFunctionalRoleMemberQuery::new("role-1")
        .user_id_type("open_id")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .functional_role_member
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.members.as_ref())
            .and_then(|members| members.first())
            .and_then(|member| member.scope_type.as_deref()),
        Some("department")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/functional_roles/role-1/members?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_functional_role_member_scopes_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"results":[{"user_id":"u-1","reason":0}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = ScopesFunctionalRoleMemberReqBody {
        members: Some(vec!["u-1".to_string()]),
        departments: Some(vec!["od-1".to_string()]),
    };
    let resp = client
        .contact()
        .functional_role_member
        .scopes_by_query(
            &ScopesFunctionalRoleMemberQuery::new("role-1", &body)
                .user_id_type("open_id")
                .department_id_type("department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("PATCH /open-apis/contact/v3/functional_roles/role-1/members/scopes?")
    );
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains(r#""members":["u-1"]"#));
    assert!(request.contains(r#""departments":["od-1"]"#));
}

#[tokio::test]
async fn contact_job_level_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"job_level_id":"level-1","name":"L1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListContactJobLevelQuery::new()
        .name("L1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .job_level
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|level| level.job_level_id.as_deref()),
        Some("level-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/job_levels?"));
    assert!(request.contains("name=L1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_job_family_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"job_family_id":"family-1","name":"Engineering"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListContactJobFamilyQuery::new()
        .name("Engineering")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .job_family
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|family| family.job_family_id.as_deref()),
        Some("family-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/job_families?"));
    assert!(request.contains("name=Engineering"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_job_title_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"job_title_id":"title-1","name":"Engineer"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListContactJobTitleQuery::new()
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .contact()
        .job_title
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|title| title.job_title_id.as_deref()),
        Some("title-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/job_titles?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_work_city_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"work_city_id":"city-1","name":"Shanghai"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListWorkCityQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .work_city
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|city| city.work_city_id.as_deref()),
        Some("city-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/work_cities?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_custom_attr_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"attr-1","name":"Nickname"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCustomAttrQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .custom_attr
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|item| item.get("id"))
            .and_then(|id| id.as_str()),
        Some("attr-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/custom_attrs?"));
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
async fn drive_file_comment_batch_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"comment_id":"comment-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = BatchQueryFileCommentReqBody {
        comment_ids: Some(vec!["comment-1".to_string()]),
    };
    let resp = client
        .drive()
        .file_comment
        .batch_query_by_query(
            &BatchQueryFileCommentQuery::new("file-token-1", "doc", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/files/file-token-1/comments/batch_query?"));
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""comment_ids":["comment-1"]"#));
}

#[tokio::test]
async fn drive_file_comment_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"comment_id":"comment-1","user_id":"ou-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file_comment
        .get_by_query(
            &GetFileCommentQuery::new("file-token-1", "comment-1", "doc").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|comment| comment.comment_id.as_deref()),
        Some("comment-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/comments/comment-1?"));
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn drive_file_comment_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"comment_id":"comment-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file_comment
        .list_by_query(
            &ListFileCommentQuery::new("file-token-1", "doc")
                .is_whole(true)
                .is_solved(false)
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/comments?"));
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("is_whole=true"));
    assert!(request.contains("is_solved=false"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn drive_file_comment_reply_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"reply_id":"reply-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file_comment_reply
        .list_by_query(
            &ListFileCommentReplyQuery::new("file-token-1", "comment-1", "doc")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("GET /open-apis/drive/v1/files/file-token-1/comments/comment-1/replies?")
    );
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn drive_permission_member_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id":"ou-1","member_type":"user","perm":"view"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .permission_member
        .list_by_query(
            &ListPermissionMemberQuery::new("file-token-1", "doc")
                .fields("member_id,perm")
                .perm_type("container"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|member| member.member_id.as_deref()),
        Some("ou-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/permissions/file-token-1/members?"));
    assert!(request.contains("type=doc"));
    assert!(request.contains("fields=member_id%2Cperm"));
    assert!(request.contains("perm_type=container"));
}

#[tokio::test]
async fn drive_permission_public_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"permission_public":{"external_access":true,"share_entity":"tenant_readable"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .permission_public
        .get_by_query(
            &GetDrivePermissionPublicQuery::new("file-token-1", "doc"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.permission_public.as_ref())
            .and_then(|permission| permission.external_access),
        Some(true)
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/permissions/file-token-1/public?"));
    assert!(request.contains("type=doc"));
}

#[tokio::test]
async fn drive_v2_permission_public_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"external_access_entity":"anyone","share_entity":"tenant_readable"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive_v2()
        .permission_public
        .get_by_query(
            &GetDrivePermissionPublicV2Query::new("file-token-1", "doc"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|permission| permission.external_access_entity.as_deref()),
        Some("anyone")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v2/permissions/file-token-1/public?"));
    assert!(request.contains("type=doc"));
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

#[tokio::test]
async fn calendar_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"calendar_id":"cal-1","summary":"Team Calendar"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .calendar()
        .calendar
        .get_by_query(&GetCalendarQuery::new("cal-1"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|calendar| calendar.summary.as_deref()),
        Some("Team Calendar")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/calendar/v4/calendars/cal-1"));
}

#[tokio::test]
async fn calendar_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"calendar_list":[{"calendar_id":"cal-1","summary":"cal"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .calendar()
        .calendar
        .list_by_query(
            &ListCalendarQuery::new()
                .page_size(20)
                .page_token("next-page")
                .sync_token("sync-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/calendar/v4/calendars?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("sync_token=sync-1"));
}

#[tokio::test]
async fn calendar_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"calendar_id":"cal-1","summary":"Team Calendar"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = SearchCalendarReqBody {
        query: Some("Team".to_string()),
    };
    let resp = client
        .calendar()
        .calendar
        .search_by_query(
            &SearchCalendarQuery::new(&body)
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/calendar/v4/calendars/search?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""query":"Team""#));
}

#[tokio::test]
async fn calendar_event_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"event":{"event_id":"event-1","summary":"Standup"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .calendar()
        .event
        .get_by_query(
            &GetCalendarEventQuery::new("cal-1", "event-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.event.as_ref())
            .and_then(|event| event.summary.as_deref()),
        Some("Standup")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/calendar/v4/calendars/cal-1/events/event-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn calendar_event_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"event_id":"event-1","summary":"Standup"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .calendar()
        .event
        .list_by_query(
            &ListCalendarEventQuery::new("cal-1")
                .page_size(20)
                .page_token("next-page")
                .anchor_time("1782910000")
                .sync_token("sync-1")
                .start_time("1782910000")
                .end_time("1782913600")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/calendar/v4/calendars/cal-1/events?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("anchor_time=1782910000"));
    assert!(request.contains("sync_token=sync-1"));
    assert!(request.contains("start_time=1782910000"));
    assert!(request.contains("end_time=1782913600"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn calendar_event_instance_view_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"event_id":"event-1","summary":"Standup"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .calendar()
        .event
        .instance_view_by_query(
            &InstanceViewEventQuery::new("cal-1")
                .start_time("1782910000")
                .end_time("1782913600")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/calendar/v4/calendars/cal-1/events/instance_view?"));
    assert!(request.contains("start_time=1782910000"));
    assert!(request.contains("end_time=1782913600"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn calendar_event_instances_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"event_id":"event-1","summary":"Standup"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .calendar()
        .event
        .instances_by_query(
            &InstancesEventQuery::new("cal-1", "event-1")
                .start_time("1782910000")
                .end_time("1782913600")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("GET /open-apis/calendar/v4/calendars/cal-1/events/event-1/instances?")
    );
    assert!(request.contains("start_time=1782910000"));
    assert!(request.contains("end_time=1782913600"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn calendar_event_search_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"event_id":"event-1","summary":"Standup"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = SearchEventReqBody {
        query: Some("Standup".to_string()),
        ..Default::default()
    };
    let resp = client
        .calendar()
        .event
        .search_by_query(
            &SearchCalendarEventQuery::new("cal-1", &body)
                .user_id_type("open_id")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/calendar/v4/calendars/cal-1/events/search?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""query":"Standup""#));
}

#[tokio::test]
async fn calendar_attendee_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"attendee_id":"att-1","user_id":"ou-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .calendar()
        .attendee
        .list_by_query(
            &ListAttendeeQuery::new("cal-1", "event-1")
                .page_size(20)
                .page_token("next-page")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("GET /open-apis/calendar/v4/calendars/cal-1/events/event-1/attendees?")
    );
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn calendar_acl_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"acls":[{"acl_id":"acl-1","role":"reader"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .calendar()
        .acl
        .list_by_query(
            &ListCalendarAclQuery::new("cal-1")
                .user_id_type("open_id")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/calendar/v4/calendars/cal-1/acls?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn calendar_timezone_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"timezone_list":[{"timezone_id":"Asia/Shanghai","name":"Shanghai"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .calendar()
        .timezone
        .list_by_query(
            &ListTimeZoneQuery::new()
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/calendar/v4/timezones?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn calendar_freebusy_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"free_busy":{"ou-1":[{"start_time":"1782910000","end_time":"1782913600"}]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = ListFreeBusyReqBody {
        time_min: Some("1782910000".to_string()),
        time_max: Some("1782913600".to_string()),
        user_id_list: Some(vec!["ou-1".to_string()]),
        ..Default::default()
    };
    let resp = client
        .calendar()
        .freebusy
        .list_by_query(
            &ListFreeBusyQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/calendar/v4/freebusy/list?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""time_min":"1782910000""#));
    assert!(request.contains(r#""time_max":"1782913600""#));
    assert!(request.contains(r#""user_id_list":["ou-1"]"#));
}

#[tokio::test]
async fn calendar_freebusy_batch_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"freebusy_lists":[{"user_id":"ou-1","freebusy_items":[]}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = BatchFreeBusyReqBody {
        time_min: Some("1782910000".to_string()),
        time_max: Some("1782913600".to_string()),
        user_ids: Some(vec!["ou-1".to_string()]),
        only_busy: Some(true),
        ..Default::default()
    };
    let resp = client
        .calendar()
        .freebusy
        .batch_by_query(
            &BatchFreeBusyQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/calendar/v4/freebusy/batch?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""time_min":"1782910000""#));
    assert!(request.contains(r#""time_max":"1782913600""#));
    assert!(request.contains(r#""user_ids":["ou-1"]"#));
    assert!(request.contains(r#""only_busy":true"#));
}

#[tokio::test]
async fn calendar_chat_member_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"open_id":"ou-1","display_name":"Alice"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .calendar()
        .attendee_chat_member
        .list_by_query(
            &ListCalendarChatMemberQuery::new("cal-1", "event-1", "att-1")
                .page_size(20)
                .page_token("next-page")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains(
        "GET /open-apis/calendar/v4/calendars/cal-1/events/event-1/attendees/att-1/chat_members?"
    ));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
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
async fn docs_content_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"content":"hello","revision":3}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .docs()
        .content
        .get_by_query(
            &GetDocsContentQuery::new("doc-token-1")
                .doc_type("docx")
                .content_type("markdown")
                .lang("en"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.as_ref().and_then(|data| data.content.as_deref()),
        Some("hello")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/docs/v1/content?"));
    assert!(request.contains("doc_token=doc-token-1"));
    assert!(request.contains("doc_type=docx"));
    assert!(request.contains("content_type=markdown"));
    assert!(request.contains("lang=en"));
}

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

// ── Payroll ──

#[tokio::test]
async fn payroll_record_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"record_id":"record-1","employee_id":"emp-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .payroll()
        .payroll_record
        .list_by_query(
            &ListPayrollRecordQuery::new()
                .period("2026-06")
                .employee_id("emp-1")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|record| record.record_id.as_deref()),
        Some("record-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/payroll/v1/payroll_records?"));
    assert!(request.contains("period=2026-06"));
    assert!(request.contains("employee_id=emp-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn payroll_cost_allocation_detail_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"detail-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .payroll()
        .cost_allocation_detail
        .list_by_query(
            &ListCostAllocationDetailQuery::new()
                .cost_allocation_plan_id("plan-1")
                .pay_period("2026-06")
                .report_type(2)
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/payroll/v1/cost_allocation_details?"));
    assert!(request.contains("cost_allocation_plan_id=plan-1"));
    assert!(request.contains("pay_period=2026-06"));
    assert!(request.contains("report_type=2"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn payroll_datasource_record_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"record-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"datasource_id":"ds-1"});
    let resp = client
        .payroll()
        .datasource_record
        .query_by_query(
            &QueryDatasourceRecordQuery::new(&body)
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/payroll/v1/datasource_records/query?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""datasource_id":"ds-1""#));
}

#[tokio::test]
async fn payroll_payment_activity_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"activity_id":"act-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let statuses = [1, 3];
    let resp = client
        .payroll()
        .payment_activity
        .list_by_query(
            &ListPaymentActivityQuery::new()
                .pay_period_start_date("2026-06-01")
                .pay_period_end_date("2026-06-30")
                .statuses(&statuses[..])
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/payroll/v1/payment_activitys?"));
    assert!(request.contains("pay_period_start_date=2026-06-01"));
    assert!(request.contains("pay_period_end_date=2026-06-30"));
    assert!(request.contains("statuses=1"));
    assert!(request.contains("statuses=3"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn payroll_payment_activity_detail_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"detail_id":"detail-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let acct_item_ids = ["acct-1", "acct-2"];
    let resp = client
        .payroll()
        .payment_activity_detail
        .list_by_query(
            &ListPaymentActivityDetailQuery::new("act-1")
                .page_index(2)
                .page_size(50)
                .include_segment_data(true)
                .acct_item_ids(&acct_item_ids[..]),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/payroll/v1/payment_activity_details?"));
    assert!(request.contains("activity_id=act-1"));
    assert!(request.contains("page_index=2"));
    assert!(request.contains("page_size=50"));
    assert!(request.contains("include_segment_data=true"));
    assert!(request.contains("acct_item_ids=acct-1"));
    assert!(request.contains("acct_item_ids=acct-2"));
}

#[tokio::test]
async fn payroll_payment_detail_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"payment_id":"pay-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"payment_activity_id":"act-1"});
    let resp = client
        .payroll()
        .payment_detail
        .query_by_query(
            &QueryPaymentDetailQuery::new(&body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/payroll/v1/payment_detail/query"));
    assert!(request.contains(r#""payment_activity_id":"act-1""#));
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

#[tokio::test]
async fn task_v2_attachment_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"attachment":{"guid":"att-1","name":"spec.pdf"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetAttachmentV2Query::new("att-1").user_id_type("open_id");
    let resp = client
        .task_v2()
        .attachment
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let attachment = resp.data.unwrap().attachment.unwrap();
    assert_eq!(attachment["guid"].as_str(), Some("att-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/attachments/att-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_attachment_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"att-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task_v2()
        .attachment
        .list(
            Some("task"),
            Some("task-guid-1"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/attachments?"));
    assert!(request.contains("resource_type=task"));
    assert!(request.contains("resource_id=task-guid-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn task_v2_attachment_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"att-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListAttachmentV2Query::new()
        .resource_type("task")
        .resource_id("task-guid-1")
        .updated_mesc("1700000000000")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .task_v2()
        .attachment
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0]["guid"].as_str(), Some("att-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/attachments?"));
    assert!(request.contains("resource_type=task"));
    assert!(request.contains("resource_id=task-guid-1"));
    assert!(request.contains("updated_mesc=1700000000000"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn task_v2_comment_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"comment":{"id":"comment-1","content":"Looks good"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetCommentV2Query::new("comment-1").user_id_type("open_id");
    let resp = client
        .task_v2()
        .comment
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let comment = resp.data.unwrap().comment.unwrap();
    assert_eq!(comment["id"].as_str(), Some("comment-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/comments/comment-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_comment_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"comment-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task_v2()
        .comment
        .list(
            Some("task"),
            Some("task-guid-1"),
            Some(20),
            Some("next-page"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/comments?"));
    assert!(request.contains("resource_type=task"));
    assert!(request.contains("resource_id=task-guid-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_comment_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"comment-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCommentV2Query::new()
        .resource_type("task")
        .resource_id("task-guid-1")
        .direction("asc")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .task_v2()
        .comment
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0]["id"].as_str(), Some("comment-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/comments?"));
    assert!(request.contains("resource_type=task"));
    assert!(request.contains("resource_id=task-guid-1"));
    assert!(request.contains("direction=asc"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn task_v2_custom_field_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"custom_field":{"guid":"field-1","name":"Priority"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetCustomFieldV2Query::new("field-1").user_id_type("open_id");
    let resp = client
        .task_v2()
        .custom_field
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let custom_field = resp.data.unwrap().custom_field.unwrap();
    assert_eq!(custom_field["guid"].as_str(), Some("field-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/custom_fields/field-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_custom_field_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"field-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task_v2()
        .custom_field
        .list(
            Some("tasklist"),
            Some("tasklist-guid-1"),
            Some(20),
            Some("next-page"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/custom_fields?"));
    assert!(request.contains("resource_type=tasklist"));
    assert!(request.contains("resource_id=tasklist-guid-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_custom_field_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"field-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCustomFieldV2Query::new()
        .resource_type("tasklist")
        .resource_id("tasklist-guid-1")
        .update_msec("1700000000000")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .task_v2()
        .custom_field
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0]["guid"].as_str(), Some("field-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/custom_fields?"));
    assert!(request.contains("resource_type=tasklist"));
    assert!(request.contains("resource_id=tasklist-guid-1"));
    assert!(request.contains("update_msec=1700000000000"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn task_v2_task_write_by_query_smoke() {
    let task_body =
        r#"{"code":0,"msg":"ok","data":{"task":{"guid":"task-guid-1","summary":"Fix bug"}}}"#;
    let list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"task-guid-1"}],"has_more":false}}"#;
    let tasklist_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"tasklist-guid-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, task_body),
        http_response(200, task_body),
        http_response(200, task_body),
        http_response(200, tasklist_body),
        http_response(200, task_body),
        http_response(200, list_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"summary":"Fix bug"});
    let patch_body = serde_json::json!({"summary":"Fix bug updated"});
    let members_body = serde_json::json!({"members":["u-1"]});
    let subtask_body = serde_json::json!({"summary":"Child task"});

    client
        .task_v2()
        .task
        .create_by_query(
            &CreateTaskV2Query::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .task
        .patch_by_query(
            &PatchTaskV2Query::new("task-guid-1", &patch_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .task
        .add_members_by_query(
            &AddMembersTaskV2Query::new("task-guid-1", &members_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .task
        .tasklists_by_query(
            &TasklistsTaskV2Query::new("task-guid-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .task
        .create_subtask_by_query(
            &CreateTaskSubtaskV2Query::new("task-guid-1", &subtask_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .task
        .list_subtasks_by_query(
            &ListTaskSubtaskV2Query::new("task-guid-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/tasks?"));
    assert!(request.contains("PATCH /open-apis/task/v2/tasks/task-guid-1?"));
    assert!(request.contains("POST /open-apis/task/v2/tasks/task-guid-1/add_members "));
    assert!(request.contains("GET /open-apis/task/v2/tasks/task-guid-1/tasklists?"));
    assert!(request.contains("POST /open-apis/task/v2/tasks/task-guid-1/subtasks?"));
    assert!(request.contains("GET /open-apis/task/v2/tasks/task-guid-1/subtasks?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""summary":"Fix bug""#));
    assert!(request.contains(r#""summary":"Fix bug updated""#));
    assert!(request.contains(r#""members":["u-1"]"#));
    assert!(request.contains(r#""summary":"Child task""#));
}

#[tokio::test]
async fn task_v2_attachment_upload_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"attachment":{"guid":"att-1","name":"spec.pdf"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let upload_body = serde_json::json!({"file_token":"file-1"});
    let resp = client
        .task_v2()
        .attachment
        .upload_by_query(
            &UploadAttachmentV2Query::new(&upload_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/attachments/upload "));
    assert!(request.contains(r#""file_token":"file-1""#));
}

#[tokio::test]
async fn task_v2_comment_write_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"comment":{"id":"comment-1","content":"Looks good"}}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"content":"Looks good"});
    let patch_body = serde_json::json!({"content":"Ship it"});
    client
        .task_v2()
        .comment
        .create_by_query(
            &CreateCommentV2Query::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .comment
        .patch_by_query(
            &PatchCommentV2Query::new("comment-1", &patch_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/comments?"));
    assert!(request.contains("PATCH /open-apis/task/v2/comments/comment-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""content":"Looks good""#));
    assert!(request.contains(r#""content":"Ship it""#));
}

#[tokio::test]
async fn task_v2_custom_field_write_by_query_smoke() {
    let field_body =
        r#"{"code":0,"msg":"ok","data":{"custom_field":{"guid":"field-1","name":"Priority"}}}"#;
    let option_body = r#"{"code":0,"msg":"ok","data":{"option":{"guid":"option-1"}}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, field_body),
        http_response(200, field_body),
        http_response(200, field_body),
        http_response(200, empty_body),
        http_response(200, option_body),
        http_response(200, option_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"name":"Priority"});
    let patch_body = serde_json::json!({"name":"Priority updated"});
    let bind_body = serde_json::json!({"resource_type":"tasklist","resource_id":"tasklist-1"});
    let option_body = serde_json::json!({"name":"High"});
    let option_patch_body = serde_json::json!({"name":"Urgent"});

    client
        .task_v2()
        .custom_field
        .create_by_query(
            &CreateCustomFieldV2Query::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .custom_field
        .patch_by_query(
            &PatchCustomFieldV2Query::new("field-1", &patch_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .custom_field
        .add_by_query(
            &AddCustomFieldV2Query::new("field-1", &bind_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .custom_field
        .remove_by_query(
            &RemoveCustomFieldV2Query::new("field-1", &bind_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .custom_field
        .create_option_by_query(
            &CreateCustomFieldOptionV2Query::new("field-1", &option_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .custom_field
        .patch_option_by_query(
            &PatchCustomFieldOptionV2Query::new("field-1", "option-1", &option_patch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/custom_fields?"));
    assert!(request.contains("PATCH /open-apis/task/v2/custom_fields/field-1?"));
    assert!(request.contains("POST /open-apis/task/v2/custom_fields/field-1/add "));
    assert!(request.contains("POST /open-apis/task/v2/custom_fields/field-1/remove "));
    assert!(request.contains("POST /open-apis/task/v2/custom_fields/field-1/options "));
    assert!(request.contains("PATCH /open-apis/task/v2/custom_fields/field-1/options/option-1 "));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""name":"Priority""#));
    assert!(request.contains(r#""name":"Priority updated""#));
    assert!(request.contains(r#""resource_type":"tasklist""#));
    assert!(request.contains(r#""name":"High""#));
    assert!(request.contains(r#""name":"Urgent""#));
}

#[tokio::test]
async fn task_v2_section_by_query_smoke() {
    let section_body =
        r#"{"code":0,"msg":"ok","data":{"section":{"guid":"section-1","name":"Backlog"}}}"#;
    let section_list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"section-1"}],"has_more":false}}"#;
    let task_list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"task-guid-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, section_body),
        http_response(200, section_body),
        http_response(200, section_body),
        http_response(200, section_list_body),
        http_response(200, task_list_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"name":"Backlog"});
    let patch_body = serde_json::json!({"name":"Doing"});
    let task_params = TaskListParams::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .completed(false)
        .created_from("2026-01-01")
        .created_to("2026-01-31")
        .user_id_type("open_id");

    client
        .task_v2()
        .section
        .create_by_query(
            &CreateSectionV2Query::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .section
        .get_by_query(
            &GetSectionV2Query::new("section-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .section
        .patch_by_query(
            &PatchSectionV2Query::new("section-1", &patch_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .section
        .list_by_query(
            &ListSectionV2Query::new()
                .resource_type("tasklist")
                .resource_id("tasklist-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .section
        .tasks_by_query(
            &TasksSectionV2Query::new("section-1").params(task_params),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/sections?"));
    assert!(request.contains("GET /open-apis/task/v2/sections/section-1?"));
    assert!(request.contains("PATCH /open-apis/task/v2/sections/section-1?"));
    assert!(request.contains("GET /open-apis/task/v2/sections?"));
    assert!(request.contains("GET /open-apis/task/v2/sections/section-1/tasks?"));
    assert!(request.contains("resource_type=tasklist"));
    assert!(request.contains("resource_id=tasklist-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("completed=false"));
    assert!(request.contains("created_from=2026-01-01"));
    assert!(request.contains("created_to=2026-01-31"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""name":"Backlog""#));
    assert!(request.contains(r#""name":"Doing""#));
}

#[tokio::test]
async fn task_v2_tasklist_by_query_smoke() {
    let tasklist_body =
        r#"{"code":0,"msg":"ok","data":{"tasklist":{"guid":"tasklist-1","name":"Roadmap"}}}"#;
    let tasklist_list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"tasklist-1"}],"has_more":false}}"#;
    let task_list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"task-guid-1"}],"has_more":false}}"#;
    let activity_body = r#"{"code":0,"msg":"ok","data":{"activity_subscription":{"guid":"activity-1","name":"Changes"}}}"#;
    let activity_list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"activity-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, tasklist_body),
        http_response(200, tasklist_body),
        http_response(200, tasklist_body),
        http_response(200, tasklist_list_body),
        http_response(200, tasklist_body),
        http_response(200, tasklist_body),
        http_response(200, task_list_body),
        http_response(200, activity_body),
        http_response(200, activity_body),
        http_response(200, activity_body),
        http_response(200, activity_list_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"name":"Roadmap"});
    let patch_body = serde_json::json!({"name":"Roadmap updated"});
    let members_body = serde_json::json!({"members":["u-1"]});
    let activity_body = serde_json::json!({"event":"task_changed"});
    let activity_patch_body = serde_json::json!({"event":"comment_changed"});
    let task_params = TaskListParams::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .completed(false)
        .created_from("2026-01-01")
        .created_to("2026-01-31")
        .user_id_type("open_id");

    client
        .task_v2()
        .tasklist
        .create_by_query(
            &CreateTasklistV2Query::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .get_by_query(
            &GetTasklistV2Query::new("tasklist-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .patch_by_query(
            &PatchTasklistV2Query::new("tasklist-1", &patch_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .list_by_query(
            &ListTasklistV2Query::new()
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .add_members_by_query(
            &AddMembersTasklistV2Query::new("tasklist-1", &members_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .remove_members_by_query(
            &RemoveMembersTasklistV2Query::new("tasklist-1", &members_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .tasks_by_query(
            &TasksTasklistV2Query::new("tasklist-1").params(task_params),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .create_activity_subscription_by_query(
            &CreateActivitySubscriptionV2Query::new("tasklist-1", &activity_body)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .get_activity_subscription_by_query(
            &GetActivitySubscriptionV2Query::new("tasklist-1", "activity-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .patch_activity_subscription_by_query(
            &PatchActivitySubscriptionV2Query::new(
                "tasklist-1",
                "activity-1",
                &activity_patch_body,
            )
            .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .tasklist
        .list_activity_subscriptions_by_query(
            &ListActivitySubscriptionV2Query::new("tasklist-1")
                .limit(50)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/tasklists?"));
    assert!(request.contains("GET /open-apis/task/v2/tasklists/tasklist-1?"));
    assert!(request.contains("PATCH /open-apis/task/v2/tasklists/tasklist-1?"));
    assert!(request.contains("GET /open-apis/task/v2/tasklists?"));
    assert!(request.contains("POST /open-apis/task/v2/tasklists/tasklist-1/add_members?"));
    assert!(request.contains("POST /open-apis/task/v2/tasklists/tasklist-1/remove_members?"));
    assert!(request.contains("GET /open-apis/task/v2/tasklists/tasklist-1/tasks?"));
    assert!(
        request.contains("POST /open-apis/task/v2/tasklists/tasklist-1/activity_subscriptions?")
    );
    assert!(request.contains(
        "GET /open-apis/task/v2/tasklists/tasklist-1/activity_subscriptions/activity-1?"
    ));
    assert!(request.contains(
        "PATCH /open-apis/task/v2/tasklists/tasklist-1/activity_subscriptions/activity-1?"
    ));
    assert!(
        request.contains("GET /open-apis/task/v2/tasklists/tasklist-1/activity_subscriptions?")
    );
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("completed=false"));
    assert!(request.contains("created_from=2026-01-01"));
    assert!(request.contains("created_to=2026-01-31"));
    assert!(request.contains("limit=50"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""name":"Roadmap""#));
    assert!(request.contains(r#""name":"Roadmap updated""#));
    assert!(request.contains(r#""members":["u-1"]"#));
    assert!(request.contains(r#""event":"task_changed""#));
    assert!(request.contains(r#""event":"comment_changed""#));
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

#[tokio::test]
async fn approval_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"approval_code":"apv-1","approval_name":"Leave Request"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .approval
        .get_by_query(
            &GetApprovalQuery::new("apv-1")
                .locale("en-US")
                .user_id("ou-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.approval_name.as_deref()),
        Some("Leave Request")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/approvals/apv-1?"));
    assert!(request.contains("locale=en-US"));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn approval_instance_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"instance_code":"inst-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .instance
        .get_by_query(
            &GetInstanceQuery::new("inst-1")
                .locale("en-US")
                .user_id("ou-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.instance_code.as_deref()),
        Some("inst-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/instances/inst-1?"));
    assert!(request.contains("locale=en-US"));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn approval_instance_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"instance_code_list":["inst-1"],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .instance
        .list_by_query(
            &ListInstanceQuery::new("apv-1", "1700000000", "1700003600")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.instance_code_list.first())
            .map(String::as_str),
        Some("inst-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/instances?"));
    assert!(request.contains("approval_code=apv-1"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700003600"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn approval_instance_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"instance_code":"inst-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = InstanceSearch {
        approval_code: Some("apv-1".to_string()),
        user_id: Some("ou-1".to_string()),
        ..Default::default()
    };
    let resp = client
        .approval()
        .instance
        .query_by_query(
            &QueryInstanceQuery::new(&body)
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/approval/v4/instances/query?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""approval_code":"apv-1""#));
    assert!(request.contains(r#""user_id":"ou-1""#));
}

#[tokio::test]
async fn approval_instance_search_cc_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"cc_title":"Review"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = CcSearch {
        cc_title: Some("Review".to_string()),
        user_id: Some("ou-1".to_string()),
        ..Default::default()
    };
    let resp = client
        .approval()
        .instance
        .search_cc_by_query(
            &SearchCcInstanceQuery::new(&body)
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/approval/v4/instances/search_cc?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""cc_title":"Review""#));
    assert!(request.contains(r#""user_id":"ou-1""#));
}

#[tokio::test]
async fn approval_task_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"task_id":"task-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .task
        .query_by_query(
            &QueryApprovalTaskQuery::new()
                .user_id("ou-1")
                .topic("assigned")
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/tasks/query?"));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("topic=assigned"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn approval_task_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"task_id":"task-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = TaskSearch {
        task_title: Some("Task".to_string()),
        user_id: Some("ou-1".to_string()),
        ..Default::default()
    };
    let resp = client
        .approval()
        .task
        .search_by_query(
            &SearchTaskQuery::new(&body)
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/approval/v4/tasks/search?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""task_title":"Task""#));
    assert!(request.contains(r#""user_id":"ou-1""#));
}

#[tokio::test]
async fn approval_external_approval_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"approval_code":"apv-1","approval_name":"External"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .external_approval
        .get_by_query(
            &GetExternalApprovalQuery::new("apv-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.approval_name.as_deref()),
        Some("External")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/external_approvals/apv-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn approval_instance_comment_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"comment_id":"comment-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .instance_comment
        .list_by_query(
            &ListInstanceCommentQuery::new("inst-1")
                .user_id("ou-1")
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/instances/inst-1/comments?"));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn approval_external_task_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"task_id":"task-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = ListExternalTaskReqBody {
        approval_codes: vec!["apv-1".to_string()],
        user_ids: vec!["ou-1".to_string()],
        ..Default::default()
    };
    let resp = client
        .approval()
        .external_task
        .list_by_query(
            &ListExternalTaskQuery::new(&body)
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/external_tasks?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""approval_codes":["apv-1"]"#));
    assert!(request.contains(r#""user_ids":["ou-1"]"#));
}

// ── Admin v1 ──

#[tokio::test]
async fn admin_badge_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"badge":{"id":"badge-1","name":"Mentor"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .badge
        .get_by_query(&GetBadgeQuery::new("badge-1"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.badge.as_ref())
            .and_then(|badge| badge.id.as_deref()),
        Some("badge-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/badges/badge-1"));
}

#[tokio::test]
async fn admin_badge_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"badge-1","name":"Mentor"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .badge
        .list_by_query(
            &ListBadgeQuery::new()
                .page_size(20)
                .page_token("next-page")
                .name("Mentor"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|badge| badge.name.as_deref()),
        Some("Mentor")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/badges?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("name=Mentor"));
}

#[tokio::test]
async fn admin_badge_grant_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"grant":{"id":"grant-1","name":"Grant"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .badge_grant
        .get_by_query(
            &GetBadgeGrantQuery::new("badge-1", "grant-1")
                .user_id_type("open_id")
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.grant.as_ref())
            .and_then(|grant| grant.id.as_deref()),
        Some("grant-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/badges/badge-1/grants/grant-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
}

#[tokio::test]
async fn admin_badge_grant_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"grant-1","name":"Grant"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .badge_grant
        .list_by_query(
            &ListBadgeGrantQuery::new("badge-1")
                .page_size(20)
                .page_token("next-page")
                .user_id_type("open_id")
                .department_id_type("open_department_id")
                .name("Grant"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/badges/badge-1/grants?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("name=Grant"));
}

#[tokio::test]
async fn admin_dept_stat_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"date":"2026-06-01","department_id":"od-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .dept_stat
        .get_by_query(
            &GetAdminDeptStatQuery::new(
                "open_department_id",
                "2026-06-01",
                "2026-06-30",
                "od-1",
                true,
            )
            .page_size(20)
            .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|stat| stat.department_id.as_deref()),
        Some("od-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/dept_stats?"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("start_date=2026-06-01"));
    assert!(request.contains("end_date=2026-06-30"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("contains_child_dept=true"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn admin_dept_stat_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .dept_stat
        .list_by_query(
            &ListAdminDeptStatQuery::new(
                "open_department_id",
                "2026-06-01",
                "2026-06-30",
                "od-1",
                true,
            )
            .page_size(20)
            .page_token("next-page")
            .target_geo("cn")
            .with_product_version(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/admin_dept_stats?"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("start_date=2026-06-01"));
    assert!(request.contains("end_date=2026-06-30"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("contains_child_dept=true"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("target_geo=cn"));
    assert!(request.contains("with_product_version=true"));
}

#[tokio::test]
async fn admin_user_stat_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .user_stat
        .list_by_query(
            &ListAdminUserStatQuery::new("2026-06-01", "2026-06-30")
                .user_id_type("open_id")
                .department_id_type("open_department_id")
                .department_id("od-1")
                .user_id("ou-1")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/admin_user_stats?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("start_date=2026-06-01"));
    assert!(request.contains("end_date=2026-06-30"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn admin_audit_info_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"event_name":"login"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .audit_info
        .list_by_query(
            &ListAuditInfoQuery::new()
                .user_id_type("open_id")
                .latest("2026-06-30")
                .oldest("2026-06-01")
                .event_name("login")
                .operator_type("user")
                .operator_value("ou-1")
                .event_module(1)
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/audit_infos?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("latest=2026-06-30"));
    assert!(request.contains("oldest=2026-06-01"));
    assert!(request.contains("event_name=login"));
    assert!(request.contains("operator_type=user"));
    assert!(request.contains("operator_value=ou-1"));
    assert!(request.contains("event_module=1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

// ── Application v6 ──

#[tokio::test]
async fn application_v6_app_recommend_rule_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"rule-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .app_recommend_rule
        .list_by_query(
            &ListAppRecommendRuleQuery::new()
                .page_size(20)
                .page_token("next-page")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.get("items"))
            .and_then(|items| items.as_array())
            .map(Vec::len),
        Some(1)
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/app_recommend_rules?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_contacts_range_configuration_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"scope-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application
        .contacts_range_configuration_by_query(
            &ApplicationContactsRangeConfigurationQuery::new("cli_a")
                .page_size(50)
                .page_token("next-page")
                .department_id_type("open_department_id")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains(
        "GET /open-apis/application/v6/applications/cli_a/contacts_range_configuration?"
    ));
    assert!(request.contains("page_size=50"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"app":{"app_id":"cli_a"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application
        .get_by_query(
            &GetApplicationV6Query::new("cli_a")
                .lang("zh_cn")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.get("app"))
            .and_then(|app| app.get("app_id"))
            .and_then(|app_id| app_id.as_str()),
        Some("cli_a")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications/cli_a?"));
    assert!(request.contains("lang=zh_cn"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"app_id":"cli_a"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application
        .list_by_query(
            &ListApplicationV6Query::new()
                .page_size(20)
                .page_token("next-page")
                .user_id_type("open_id")
                .lang("zh_cn")
                .status(1)
                .payment_type(2)
                .owner_type(3),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("lang=zh_cn"));
    assert!(request.contains("status=1"));
    assert!(request.contains("payment_type=2"));
    assert!(request.contains("owner_type=3"));
}

#[tokio::test]
async fn application_v6_underauditlist_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"app_id":"cli_a"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application
        .underauditlist_by_query(
            &UnderauditlistApplicationQuery::new()
                .lang("zh_cn")
                .page_size(20)
                .page_token("next-page")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications/underauditlist?"));
    assert!(request.contains("lang=zh_cn"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_app_version_contacts_range_suggest_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"scope-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_app_version
        .contacts_range_suggest_by_query(
            &ContactsRangeSuggestApplicationAppVersionQuery::new("cli_a", "ver-1")
                .department_id_type("open_department_id")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains(
        "GET /open-apis/application/v6/applications/cli_a/app_versions/ver-1/contacts_range_suggest?"
    ));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_app_version_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"app_version":{"version_id":"ver-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_app_version
        .get_by_query(
            &GetApplicationV6AppVersionQuery::new("cli_a", "ver-1")
                .lang("zh_cn")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("GET /open-apis/application/v6/applications/cli_a/app_versions/ver-1?")
    );
    assert!(request.contains("lang=zh_cn"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_app_version_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"version_id":"ver-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_app_version
        .list_by_query(
            &ListApplicationAppVersionQuery::new("cli_a")
                .lang("zh_cn")
                .page_size(20)
                .page_token("next-page")
                .order(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications/cli_a/app_versions?"));
    assert!(request.contains("lang=zh_cn"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("order=1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_collaborators_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_collaborators
        .get_by_query(
            &GetApplicationCollaboratorsQuery::new("cli_a").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications/cli_a/collaborators?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_feedback_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"feedback_id":"fb-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_feedback
        .list_by_query(
            &ListApplicationFeedbackQuery::new("cli_a")
                .from_date("2026-06-01")
                .to_date("2026-06-30")
                .feedback_type(1)
                .status(2)
                .user_id_type("open_id")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications/cli_a/feedbacks?"));
    assert!(request.contains("from_date=2026-06-01"));
    assert!(request.contains("to_date=2026-06-30"));
    assert!(request.contains("feedback_type=1"));
    assert!(request.contains("status=2"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

// ── ACS ──

#[tokio::test]
async fn acs_user_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"user":{"user_id":"ou-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .acs()
        .user
        .get_by_query(
            &GetAcsUserQuery::new("ou-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.user.as_ref())
            .and_then(|user| user.user_id.as_deref()),
        Some("ou-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/users/ou-1?user_id_type=open_id"));
}

#[tokio::test]
async fn acs_user_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .acs()
        .user
        .list_by_query(
            &ListAcsUserQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|user| user.user_id.as_deref()),
        Some("ou-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/users?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn acs_access_record_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"access_record_id":"rec-1","device_id":"dev-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .acs()
        .access_record
        .list_by_query(
            &ListAcsAccessRecordQuery::new()
                .page(PageQuery::new().page_size(50).page_token("next-page"))
                .from(1_700_000_000)
                .to(1_700_000_100)
                .device_id("dev-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|record| record.access_record_id.as_deref()),
        Some("rec-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/access_records?"));
    assert!(request.contains("page_size=50"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("from=1700000000"));
    assert!(request.contains("to=1700000100"));
    assert!(request.contains("device_id=dev-1"));
    assert!(request.contains("user_id_type=open_id"));
}

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
async fn helpdesk_ticket_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ticket":{"ticket_id":"ticket-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket
        .get_by_query(&GetTicketQuery::new("ticket-1"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.ticket.as_ref())
            .and_then(|ticket| ticket.ticket_id.as_deref()),
        Some("ticket-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/tickets/ticket-1"));
}

#[tokio::test]
async fn helpdesk_ticket_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"tickets":[{"ticket_id":"ticket-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket
        .list_by_query(
            &ListTicketQuery::new()
                .ticket_id("ticket-1")
                .agent_id("agent-1")
                .closed_by_id("agent-2")
                .status(3)
                .guest_id("guest-1")
                .keyword("refund")
                .create_time_start(1782910000)
                .create_time_end(1782913600)
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.tickets.first())
            .and_then(|ticket| ticket.ticket_id.as_deref()),
        Some("ticket-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/tickets?"));
    assert!(request.contains("ticket_id=ticket-1"));
    assert!(request.contains("agent_id=agent-1"));
    assert!(request.contains("closed_by_id=agent-2"));
    assert!(request.contains("status=3"));
    assert!(request.contains("guest_id=guest-1"));
    assert!(request.contains("keyword=refund"));
    assert!(request.contains("create_time_start=1782910000"));
    assert!(request.contains("create_time_end=1782913600"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn helpdesk_ticket_customized_fields_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ticket_customized_fields":[{"ticket_customized_field_id":"field-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket
        .customized_fields_by_query(
            &CustomizedFieldsTicketQuery::new().visible_only(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/customized_fields?"));
    assert!(request.contains("visible_only=true"));
}

#[tokio::test]
async fn helpdesk_ticket_message_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"messages":[{"message_id":"msg-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket_message
        .list_by_query(
            &ListTicketMessageQuery::new("ticket-1")
                .time_start(1782910000)
                .time_end(1782913600)
                .page_token(1782910100)
                .page_size(20),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.messages.first())
            .and_then(|message| message.message_id.as_deref()),
        Some("msg-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/tickets/ticket-1/messages?"));
    assert!(request.contains("time_start=1782910000"));
    assert!(request.contains("time_end=1782913600"));
    assert!(request.contains("page_token=1782910100"));
    assert!(request.contains("page_size=20"));
}

#[tokio::test]
async fn helpdesk_agent_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"agents":[{"agent_id":"agent-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .agent
        .list_by_query(&ListAgentQuery::new().status(1), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.agents.first())
            .and_then(|agent| agent.agent_id.as_deref()),
        Some("agent-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/agents?"));
    assert!(request.contains("status=1"));
}

#[tokio::test]
async fn helpdesk_agent_schedules_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"schedules":[{"schedule_id":"schedule-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .agent_schedules
        .get_by_query(
            &GetAgentSchedulesQuery::new("agent-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/agents/agent-1/schedules"));
}

#[tokio::test]
async fn helpdesk_agent_schedule_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"schedule_id":"schedule-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let statuses = [1, 2];
    let resp = client
        .helpdesk()
        .agent_schedule
        .list_by_query(
            &ListAgentScheduleQuery::new().status(statuses.as_slice()),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/agent_schedules?"));
    assert!(request.contains("status=1"));
    assert!(request.contains("status=2"));
}

#[tokio::test]
async fn helpdesk_category_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"category":{"id":"cat-1","name":"Billing"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .category
        .get_by_query(&GetCategoryQuery::new("cat-1"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.category.as_ref())
            .and_then(|category| category.name.as_deref()),
        Some("Billing")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/categories/cat-1"));
}

#[tokio::test]
async fn helpdesk_category_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"categories":[{"id":"cat-1","name":"Billing"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .category
        .list_by_query(
            &ListCategoryQuery::new().language("en-US"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/categories?"));
    assert!(request.contains("language=en-US"));
}

#[tokio::test]
async fn helpdesk_faq_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"faq":{"id":"faq-1","answer":"Try restarting"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .get_by_query(
            &GetHelpdeskFaqQuery::new("faq-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.faq.as_ref())
            .and_then(|faq| faq.id.as_deref()),
        Some("faq-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs/faq-1"));
}

#[tokio::test]
async fn helpdesk_faq_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"faq-1","question":"How?"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .list_by_query(
            &ListFaqQuery::new()
                .category_id("cat-1")
                .keyword("restart")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs?"));
    assert!(request.contains("category_id=cat-1"));
    assert!(request.contains("keyword=restart"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn helpdesk_faq_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"faq-1","question":"How?"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .search_by_query(
            &SearchFaqQuery::new()
                .query("restart")
                .base64("cmVzdGFydA==")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs/search?"));
    assert!(request.contains("query=restart"));
    assert!(request.contains("base64=cmVzdGFydA%3D%3D"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
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
async fn vc_room_create_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"room":{"room_id":"room-1","name":"Boardroom"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = CreateRoomReqBody {
        name: Some("Boardroom".to_string()),
        capacity: Some(8),
        room_level_id: Some("level-1".to_string()),
        ..Default::default()
    };
    let resp = client
        .vc()
        .room
        .create_by_query(
            &CreateVcRoomQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.room.as_ref())
            .and_then(|room| room.room_id.as_deref()),
        Some("room-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/vc/v1/rooms?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""name":"Boardroom""#));
    assert!(request.contains(r#""capacity":8"#));
}

#[tokio::test]
async fn vc_room_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"room":{"room_id":"room-1","name":"Boardroom"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .room
        .get_by_query(
            &GetVcRoomQuery::new("room-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.room.as_ref())
            .and_then(|room| room.name.as_deref()),
        Some("Boardroom")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/rooms/room-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_room_update_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = UpdateRoomReqBody {
        name: Some("Boardroom Updated".to_string()),
        capacity: Some(10),
        ..Default::default()
    };
    let resp = client
        .vc()
        .room
        .update_by_query(
            &UpdateVcRoomQuery::new("room-1", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/vc/v1/rooms/room-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""name":"Boardroom Updated""#));
    assert!(request.contains(r#""capacity":10"#));
}

#[tokio::test]
async fn vc_room_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rooms":[{"room_id":"room-1","name":"Boardroom"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListVcRoomQuery::new()
        .room_level_id("level-1")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .vc()
        .room
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.as_ref().unwrap().rooms.len(), 1);
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/rooms?"));
    assert!(request.contains("room_level_id=level-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn vc_room_mget_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rooms":[{"room_id":"room-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"room_ids":["room-1"]});
    let resp = client
        .vc()
        .room
        .mget_by_query(
            &MgetRoomQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/vc/v1/rooms/mget?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""room_ids":["room-1"]"#));
}

#[tokio::test]
async fn vc_room_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rooms":[{"room_id":"room-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"query":"Boardroom"});
    let resp = client
        .vc()
        .room
        .search_by_query(
            &SearchRoomQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/vc/v1/rooms/search?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""query":"Boardroom""#));
}

#[tokio::test]
async fn vc_room_config_set_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = SetRoomConfigReqBody {
        set_room_background: Some(true),
        room_background: Some("blue".to_string()),
        ..Default::default()
    };
    let resp = client
        .vc()
        .room_config
        .set_by_query(
            &SetRoomConfigQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/vc/v1/room_configs/set?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""set_room_background":true"#));
    assert!(request.contains(r#""room_background":"blue""#));
}

#[tokio::test]
async fn vc_reserve_apply_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"reserve_id":"reserve-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"room_id":"room-1","topic":"Planning"});
    let resp = client
        .vc()
        .reserve
        .apply_by_query(
            &ApplyReserveQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/vc/v1/reserves/apply?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""room_id":"room-1""#));
    assert!(request.contains(r#""topic":"Planning""#));
}

#[tokio::test]
async fn vc_reserve_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"reserve_id":"reserve-1","room_id":"room-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .reserve
        .get_by_query(
            &GetReserveQuery::new("reserve-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/reserves/reserve-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_reserve_active_meeting_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"meeting_id":"meeting-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .reserve
        .get_active_meeting_by_query(
            &GetActiveMeetingReserveQuery::new("reserve-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/reserves/reserve-1/get_active_meeting?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_reserve_update_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"reserve_id":"reserve-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"topic":"Updated Planning"});
    let resp = client
        .vc()
        .reserve
        .update_by_query(
            &UpdateReserveQuery::new("reserve-1", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PUT /open-apis/vc/v1/reserves/reserve-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""topic":"Updated Planning""#));
}

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
