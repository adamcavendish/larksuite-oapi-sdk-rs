use super::prelude::*;

// VC room smoke tests

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
