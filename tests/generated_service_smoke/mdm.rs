use super::prelude::*;

// ── MDM ──

#[tokio::test]
async fn mdm_v1_user_device_and_auth_relation_by_query_smoke() {
    let list_body = r#"{"code":0,"msg":"ok","data":{"items":[{"device_id":"device-1","user_id":"ou-1"}],"has_more":false}}"#;
    let ok_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, list_body),
        http_response(200, ok_body),
        http_response(200, ok_body),
        http_response(200, ok_body),
    ])
    .await;

    let client = client_for(addr);
    let update_body = UpdateDeviceReqBody { status: Some(2) };
    let bind_body = json_value!({"user_id":"ou-1","device_id":"device-1"});
    let unbind_body = json_value!({"user_id":"ou-1","device_id":"device-1"});

    let resp = client
        .mdm()
        .user_device
        .list_by_query(
            &ListUserDeviceQuery::new("ou-1")
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mdm()
        .user_device
        .update_by_query(
            &UpdateUserDeviceQuery::new("device-1", &update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mdm()
        .user_auth_data_relation
        .bind_by_query(
            &BindUserAuthDataRelationQuery::new(&bind_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mdm()
        .user_auth_data_relation
        .unbind_by_query(
            &UnbindUserAuthDataRelationQuery::new(&unbind_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/mdm/v1/user_devices?"));
    assert!(request.contains("PATCH /open-apis/mdm/v1/user_devices/device-1 "));
    assert!(request.contains("POST /open-apis/mdm/v1/user_auth_data_relations/bind?"));
    assert!(request.contains("POST /open-apis/mdm/v1/user_auth_data_relations/unbind?"));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""status":2"#));
    assert!(request.contains(r#""device_id":"device-1""#));
}

#[tokio::test]
async fn mdm_country_region_v3_by_query_smoke() {
    let batch_body = r#"{"code":0,"msg":"ok","data":{"data":[{"id":"CN","alpha_2_code":"CN","name":{"value":"China"}}]}}"#;
    let list_body = r#"{"code":0,"msg":"ok","data":{"data":[{"id":"CN","alpha_2_code":"CN","name":{"multilingual_value":{"en-US":"China"}},"continents":{"value":"asia"},"md_time_zone":[{"time_zone_id":"Asia/Shanghai","name":{"languages":["zh-CN"]}}]}],"total":"1","next_page_token":"next-2"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, batch_body),
        http_response(200, list_body),
    ])
    .await;

    let client = client_for(addr);
    let batch_resp = client
        .mdm_v3()
        .batch_country_region
        .get_by_query(
            &GetBatchCountryRegionV3Query::new(),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let list_resp = client
        .mdm_v3()
        .country_region
        .list_by_query(
            &ListCountryRegionV3Query::new()
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(
        batch_resp.data.unwrap().data[0].alpha_2_code.as_deref(),
        Some("CN")
    );
    let list_data = list_resp.data.unwrap();
    assert_eq!(list_data.data[0].id.as_deref(), Some("CN"));
    assert_eq!(
        list_data.data[0]
            .name
            .as_ref()
            .unwrap()
            .multilingual_value
            .as_ref()
            .unwrap()["en-US"],
        "China"
    );
    assert_eq!(
        list_data.data[0]
            .continents
            .as_ref()
            .unwrap()
            .value
            .as_deref(),
        Some("asia")
    );
    assert_eq!(
        list_data.data[0].md_time_zone.as_ref().unwrap()[0]
            .name
            .as_ref()
            .unwrap()
            .languages
            .as_ref()
            .unwrap()[0],
        "zh-CN"
    );
    assert_eq!(list_data.next_page_token.as_deref(), Some("next-2"));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/mdm/v3/batch_country_region "));
    assert!(request.contains("GET /open-apis/mdm/v3/country_regions?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn meeting_room_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rooms":[{"room_id":"room-1","name":"Boardroom"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .meeting_room()
        .room
        .list(
            Some("building-1"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/meeting_room/v1/rooms?"));
    assert!(request.contains("building_id=building-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn meeting_room_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"rooms":[{"room_id":"room-1","name":"Boardroom"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListMeetingRoomQuery::new()
        .building_id("building-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .meeting_room()
        .room
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.rooms.len(), 1);
    assert_eq!(data.rooms[0].room_id.as_deref(), Some("room-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/meeting_room/v1/rooms?"));
    assert!(request.contains("building_id=building-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn meeting_room_building_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"buildings":[{"building_id":"building-1","name":"HQ"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListMeetingRoomBuildingQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .meeting_room()
        .building
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.buildings.len(), 1);
    assert_eq!(data.buildings[0].building_id.as_deref(), Some("building-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/meeting_room/v1/buildings?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
