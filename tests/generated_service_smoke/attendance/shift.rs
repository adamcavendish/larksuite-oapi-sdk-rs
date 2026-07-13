use super::prelude::*;

// ── Attendance ──

#[tokio::test]
async fn attendance_shift_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"shift_list":[{"shift_id":"shift-1","shift_name":"Day"}],"has_more":false}}"#;
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
            .and_then(|data| data.shift_list.first())
            .and_then(|item| item.shift_id.as_deref()),
        Some("shift-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/shifts?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
