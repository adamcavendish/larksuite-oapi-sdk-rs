use super::prelude::*;

// ── Calendar ──

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
