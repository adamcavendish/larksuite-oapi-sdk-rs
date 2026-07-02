use super::prelude::*;

// ── Calendar ──

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
