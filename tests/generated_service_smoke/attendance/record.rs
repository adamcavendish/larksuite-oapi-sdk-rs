use super::prelude::*;

// ── Attendance ──

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
