use super::*;

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
