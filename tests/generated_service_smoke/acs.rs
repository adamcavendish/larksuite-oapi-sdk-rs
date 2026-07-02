use super::*;

// ── ACS ──

#[tokio::test]
async fn acs_user_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"user":{"user_id":"ou-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .acs()
        .user
        .get_by_query(
            &GetAcsUserQuery::new("ou-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.user.as_ref())
            .and_then(|user| user.user_id.as_deref()),
        Some("ou-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/users/ou-1?user_id_type=open_id"));
}

#[tokio::test]
async fn acs_user_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .acs()
        .user
        .list_by_query(
            &ListAcsUserQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|user| user.user_id.as_deref()),
        Some("ou-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/users?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn acs_access_record_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"access_record_id":"rec-1","device_id":"dev-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .acs()
        .access_record
        .list_by_query(
            &ListAcsAccessRecordQuery::new()
                .page(PageQuery::new().page_size(50).page_token("next-page"))
                .from(1_700_000_000)
                .to(1_700_000_100)
                .device_id("dev-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|record| record.access_record_id.as_deref()),
        Some("rec-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/access_records?"));
    assert!(request.contains("page_size=50"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("from=1700000000"));
    assert!(request.contains("to=1700000100"));
    assert!(request.contains("device_id=dev-1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn acs_list_devices_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"device_id":"dev-1","device_name":"Front Gate"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .acs()
        .device
        .list_by_query(&ListDeviceQuery::new(), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert!(resp.data.is_some());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/devices"));
}

#[tokio::test]
async fn acs_user_patch_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"card_list":[{"card_id":"card-1"}]});
    let resp = client
        .acs()
        .user
        .patch_by_query(
            &PatchAcsUserQuery::new("ou-1", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/acs/v1/users/ou-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""card_id":"card-1""#));
}

#[tokio::test]
async fn acs_rule_external_by_query_smoke() {
    let rule_body = r#"{"code":0,"msg":"ok","data":{"rule":{"rule_id":"rule-1"}}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let bind_body = r#"{"code":0,"msg":"ok","data":{"device_id":"dev-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, rule_body),
        http_response(200, empty_body),
        http_response(200, bind_body),
        http_response(200, rule_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"device_id":"dev-1","user_ids":["ou-1"]});
    let bind_body = serde_json::json!({"device_ids":["dev-1"],"rule_id":"rule-1"});
    let create_resp = client
        .acs()
        .rule_external
        .create_by_query(
            &CreateAcsRuleExternalQuery::new(&create_body)
                .rule_id("rule-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete_resp = client
        .acs()
        .rule_external
        .delete_by_query(
            &DeleteAcsRuleExternalQuery::new("rule-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let bind_resp = client
        .acs()
        .rule_external
        .device_bind_by_query(
            &DeviceBindRuleExternalQuery::new(&bind_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let get_resp = client
        .acs()
        .rule_external
        .get_by_query(
            &GetRuleExternalQuery::new()
                .device_id("dev-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(delete_resp.success());
    assert!(bind_resp.success());
    assert!(get_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/acs/v1/rule_external?"));
    assert!(request.contains("DELETE /open-apis/acs/v1/rule_external?"));
    assert!(request.contains("POST /open-apis/acs/v1/rule_external/device_bind"));
    assert!(request.contains("GET /open-apis/acs/v1/rule_external?"));
    assert!(request.contains("rule_id=rule-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("device_id=dev-1"));
    assert!(request.contains(r#""user_ids":["ou-1"]"#));
    assert!(request.contains(r#""device_ids":["dev-1"]"#));
}

#[tokio::test]
async fn acs_face_visitor_by_query_smoke() {
    let photo_body = r#"{"code":0,"msg":"ok","data":{"file_content":"photo-bytes"}}"#;
    let face_body = r#"{"code":0,"msg":"ok","data":{"file_content":"face-bytes"}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let visitor_body = r#"{"code":0,"msg":"ok","data":{"visitor_id":"visitor-1"}}"#;
    let delete_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, photo_body),
        http_response(200, face_body),
        http_response(200, empty_body),
        http_response(200, visitor_body),
        http_response(200, delete_body),
    ])
    .await;

    let client = client_for(addr);
    let face_update_body = UpdateUserFaceReqBody {
        files: Some(serde_json::json!(["face-bytes"])),
        file_type: Some("jpg".to_string()),
        file_name: Some("face.jpg".to_string()),
    };
    let visitor_create_body = serde_json::json!({"name":"Visitor","user_id":"ou-1"});
    let photo_resp = client
        .acs()
        .access_record_access_photo
        .get_by_query(
            &GetAccessRecordAccessPhotoQuery::new("rec-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let face_resp = client
        .acs()
        .user_face
        .get_by_query(
            &GetUserFaceQuery::new("ou-1")
                .is_cropped(true)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_face_resp = client
        .acs()
        .user_face
        .update_by_query(
            &UpdateUserFaceQuery::new("ou-1", &face_update_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let create_visitor_resp = client
        .acs()
        .visitor
        .create_by_query(
            &CreateVisitorQuery::new(&visitor_create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete_visitor_resp = client
        .acs()
        .visitor
        .delete_by_query(
            &DeleteVisitorQuery::new("visitor-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(photo_resp.success());
    assert!(face_resp.success());
    assert!(update_face_resp.success());
    assert!(create_visitor_resp.success());
    assert!(delete_visitor_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/access_records/rec-1/access_photo"));
    assert!(request.contains("GET /open-apis/acs/v1/users/ou-1/face?"));
    assert!(request.contains("PUT /open-apis/acs/v1/users/ou-1/face?"));
    assert!(request.contains("POST /open-apis/acs/v1/visitors?"));
    assert!(request.contains("DELETE /open-apis/acs/v1/visitors/visitor-1?"));
    assert!(request.contains("is_cropped=true"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""file_type":"jpg""#));
    assert!(request.contains(r#""file_name":"face.jpg""#));
    assert!(request.contains(r#""name":"Visitor""#));
}
