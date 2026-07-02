use super::prelude::*;

// ── Calendar ──

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
