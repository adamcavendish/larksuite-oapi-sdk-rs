use super::prelude::*;

// ── Calendar ──

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
