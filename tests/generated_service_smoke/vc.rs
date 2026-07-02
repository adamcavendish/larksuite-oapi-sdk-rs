use super::*;

// ── VC ──

#[tokio::test]
async fn vc_room_create_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"room":{"room_id":"room-1","name":"Boardroom"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = CreateRoomReqBody {
        name: Some("Boardroom".to_string()),
        capacity: Some(8),
        room_level_id: Some("level-1".to_string()),
        ..Default::default()
    };
    let resp = client
        .vc()
        .room
        .create_by_query(
            &CreateVcRoomQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.room.as_ref())
            .and_then(|room| room.room_id.as_deref()),
        Some("room-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/vc/v1/rooms?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""name":"Boardroom""#));
    assert!(request.contains(r#""capacity":8"#));
}

#[tokio::test]
async fn vc_room_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"room":{"room_id":"room-1","name":"Boardroom"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .room
        .get_by_query(
            &GetVcRoomQuery::new("room-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.room.as_ref())
            .and_then(|room| room.name.as_deref()),
        Some("Boardroom")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/rooms/room-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_room_update_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = UpdateRoomReqBody {
        name: Some("Boardroom Updated".to_string()),
        capacity: Some(10),
        ..Default::default()
    };
    let resp = client
        .vc()
        .room
        .update_by_query(
            &UpdateVcRoomQuery::new("room-1", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/vc/v1/rooms/room-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""name":"Boardroom Updated""#));
    assert!(request.contains(r#""capacity":10"#));
}

#[tokio::test]
async fn vc_room_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rooms":[{"room_id":"room-1","name":"Boardroom"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListVcRoomQuery::new()
        .room_level_id("level-1")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .vc()
        .room
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.as_ref().unwrap().rooms.len(), 1);
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/rooms?"));
    assert!(request.contains("room_level_id=level-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn vc_room_mget_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rooms":[{"room_id":"room-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"room_ids":["room-1"]});
    let resp = client
        .vc()
        .room
        .mget_by_query(
            &MgetRoomQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/vc/v1/rooms/mget?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""room_ids":["room-1"]"#));
}

#[tokio::test]
async fn vc_room_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rooms":[{"room_id":"room-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"query":"Boardroom"});
    let resp = client
        .vc()
        .room
        .search_by_query(
            &SearchRoomQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/vc/v1/rooms/search?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""query":"Boardroom""#));
}

#[tokio::test]
async fn vc_room_config_set_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = SetRoomConfigReqBody {
        set_room_background: Some(true),
        room_background: Some("blue".to_string()),
        ..Default::default()
    };
    let resp = client
        .vc()
        .room_config
        .set_by_query(
            &SetRoomConfigQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/vc/v1/room_configs/set?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""set_room_background":true"#));
    assert!(request.contains(r#""room_background":"blue""#));
}

#[tokio::test]
async fn vc_reserve_apply_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"reserve_id":"reserve-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"room_id":"room-1","topic":"Planning"});
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
    assert!(request.contains(r#""room_id":"room-1""#));
    assert!(request.contains(r#""topic":"Planning""#));
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
    let body = serde_json::json!({"topic":"Updated Planning"});
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
    assert!(request.contains(r#""topic":"Updated Planning""#));
}

#[tokio::test]
async fn vc_room_config_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"room_config":{"room_background":"blue"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = RoomConfigQuery::new(1)
        .country_id("country-1")
        .district_id("district-1")
        .building_id("building-1")
        .floor_name("3F")
        .room_id("room-1")
        .user_id_type("open_id");
    let resp = client
        .vc()
        .room_config
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.room_config.unwrap().room_background.as_deref(),
        Some("blue")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/room_configs/query?"));
    assert!(request.contains("scope=1"));
    assert!(request.contains("country_id=country-1"));
    assert!(request.contains("district_id=district-1"));
    assert!(request.contains("building_id=building-1"));
    assert!(request.contains("floor_name=3F"));
    assert!(request.contains("room_id=room-1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_room_config_query_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .room_config
        .query(
            1,
            Some("country-1"),
            Some("district-1"),
            Some("building-1"),
            Some("3F"),
            Some("room-1"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/room_configs/query?"));
    assert!(request.contains("scope=1"));
    assert!(request.contains("country_id=country-1"));
    assert!(request.contains("district_id=district-1"));
    assert!(request.contains("building_id=building-1"));
    assert!(request.contains("floor_name=3F"));
    assert!(request.contains("room_id=room-1"));
    assert!(request.contains("user_id_type=open_id"));
}

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
async fn vc_report_top_user_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .report
        .get_top_user(
            "1700000000",
            "1700000100",
            50,
            1,
            Some(2),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/reports/get_top_user?"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("limit=50"));
    assert!(request.contains("order_by=1"));
    assert!(request.contains("meeting_type=2"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_report_top_user_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetTopUserReportQuery::new("1700000000", "1700000100", 50, 1)
        .meeting_type(2)
        .user_id_type("open_id");
    let resp = client
        .vc()
        .report
        .get_top_user_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/reports/get_top_user?"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("limit=50"));
    assert!(request.contains("order_by=1"));
    assert!(request.contains("meeting_type=2"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_alert_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListAlertQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .start_time("1700000000")
        .end_time("1700000100")
        .query_type(1)
        .query_value("room-1");
    let resp = client
        .vc()
        .alert
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/alerts?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("query_type=1"));
    assert!(request.contains("query_value=room-1"));
}

#[tokio::test]
async fn vc_participant_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .participant_list
        .get(
            "1700000000",
            "1700000100",
            Some(1),
            "meeting-no",
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
    assert!(request.contains("GET /open-apis/vc/v1/participant_list?"));
    assert!(request.contains("meeting_start_time=1700000000"));
    assert!(request.contains("meeting_end_time=1700000100"));
    assert!(request.contains("meeting_status=1"));
    assert!(request.contains("meeting_no=meeting-no"));
    assert!(request.contains("user_id=user-1"));
    assert!(request.contains("room_id=room-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_participant_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetParticipantListQuery::new("1700000000", "1700000100", "meeting-no")
        .meeting_status(1)
        .user_id("user-1")
        .room_id("room-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"))
        .user_id_type("open_id");
    let resp = client
        .vc()
        .participant_list
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/participant_list?"));
    assert!(request.contains("meeting_start_time=1700000000"));
    assert!(request.contains("meeting_end_time=1700000100"));
    assert!(request.contains("meeting_status=1"));
    assert!(request.contains("meeting_no=meeting-no"));
    assert!(request.contains("user_id=user-1"));
    assert!(request.contains("room_id=room-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn vc_participant_quality_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        GetParticipantQualityListQuery::new("1700000000", "1700000100", "meeting-no", "1700000050")
            .user_id("user-1")
            .room_id("room-1")
            .page(PageQuery::new().page_size(20).page_token("next-page"))
            .user_id_type("open_id");
    let resp = client
        .vc()
        .participant_quality_list
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/participant_quality_list?"));
    assert!(request.contains("meeting_start_time=1700000000"));
    assert!(request.contains("meeting_end_time=1700000100"));
    assert!(request.contains("meeting_no=meeting-no"));
    assert!(request.contains("join_time=1700000050"));
    assert!(request.contains("user_id=user-1"));
    assert!(request.contains("room_id=room-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
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

#[tokio::test]
async fn vc_export_download_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"file_token":"download-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .vc()
        .export
        .download("file-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap()["file_token"].as_str(),
        Some("download-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/vc/v1/exports/download?"));
    assert!(request.contains("file_token=file-token-1"));
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
