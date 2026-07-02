use super::prelude::*;

// ── Attendance ──

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
