use super::prelude::*;

// ── Calendar ──

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
