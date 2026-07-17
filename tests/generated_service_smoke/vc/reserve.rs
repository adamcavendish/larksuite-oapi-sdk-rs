use super::prelude::*;

// VC reservation smoke tests

#[tokio::test]
async fn vc_reserve_apply_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"reserve_id":"reserve-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = ApplyReserveReqBody {
        end_time: Some("1700000100".into()),
        owner_id: Some("ou_owner".into()),
        ..Default::default()
    };
    let resp = client
        .vc()
        .reserve
        .apply_by_query(
            &ApplyReserveQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/vc/v1/reserves/apply?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""end_time":"1700000100""#));
    assert!(request.contains(r#""owner_id":"ou_owner""#));
}

#[tokio::test]
async fn vc_reserve_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"reserve_id":"reserve-1","room_id":"room-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .reserve
        .get_by_query(
            &GetReserveQuery::new("reserve-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/reserves/reserve-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_reserve_active_meeting_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"meeting_id":"meeting-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .reserve
        .get_active_meeting_by_query(
            &GetActiveMeetingReserveQuery::new("reserve-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/reserves/reserve-1/get_active_meeting?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_reserve_update_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"reserve_id":"reserve-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = UpdateReserveReqBody {
        end_time: Some("1700000200".into()),
        ..Default::default()
    };
    let resp = client
        .vc()
        .reserve
        .update_by_query(
            &UpdateReserveQuery::new("reserve-1", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PUT /open-apis/vc/v1/reserves/reserve-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""end_time":"1700000200""#));
}

#[tokio::test]
async fn vc_resource_reservation_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .resource_reservation_list
        .get(
            "level-1",
            Some(true),
            "1700000000",
            "1700000100",
            Some("room-1,room-2"),
            Some(false),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/resource_reservation_list?"));
    assert!(request.contains("room_level_id=level-1"));
    assert!(request.contains("has_video=true"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("room_ids=room-1%2Croom-2"));
    assert!(request.contains("is_exclude=false"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn vc_resource_reservation_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetResourceReservationListQuery::new("level-1", "1700000000", "1700000100")
        .has_video(true)
        .room_ids("room-1,room-2")
        .is_exclude(false)
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .vc()
        .resource_reservation_list
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/resource_reservation_list?"));
    assert!(request.contains("room_level_id=level-1"));
    assert!(request.contains("has_video=true"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("room_ids=room-1%2Croom-2"));
    assert!(request.contains("is_exclude=false"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
