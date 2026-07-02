use super::prelude::*;

// ── Attendance ──

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
