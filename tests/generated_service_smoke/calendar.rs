use super::*;

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
