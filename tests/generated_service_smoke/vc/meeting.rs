use super::prelude::*;

// VC meeting smoke tests

#[tokio::test]
async fn vc_meeting_list_by_no_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"meeting_briefs":[{"id":"m-1","topic":"Sprint Review"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListMeetingByNoQuery::new("meeting-no", "1700000000", "1700000100")
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .user_id_type("open_id");
    let resp = client
        .vc()
        .meeting
        .list_by_no_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.unwrap().meeting_briefs.len(), 1);
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/meetings/list_by_no?"));
    assert!(request.contains("meeting_no=meeting-no"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_get_meeting_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"meeting":{"id":"m-1","topic":"Sprint Review"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .meeting_list
        .get(
            "m-1",
            "",
            Some(1),
            Some("meeting-no"),
            Some("user-1"),
            Some("room-1"),
            Some(20),
            Some("next-page"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/meeting_list?"));
    assert!(request.contains("start_time=m-1"));
    assert!(request.contains("end_time="));
    assert!(request.contains("meeting_status=1"));
    assert!(request.contains("meeting_no=meeting-no"));
    assert!(request.contains("user_id=user-1"));
    assert!(request.contains("room_id=room-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_get_meeting_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"meeting":{"id":"m-1","topic":"Sprint Review"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetMeetingListQuery::new("m-1", "")
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .user_id_type("open_id");
    let resp = client
        .vc()
        .meeting_list
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert!(resp.data.is_some());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/meeting_list?"));
    assert!(request.contains("start_time=m-1"));
    assert!(request.contains("end_time="));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}
