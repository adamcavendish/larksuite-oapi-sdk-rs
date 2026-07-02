use super::prelude::*;

// ── Admin v1 ──

#[tokio::test]
async fn admin_v1_write_by_query_smoke() {
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let badge_body = r#"{"code":0,"msg":"ok","data":{"badge":{"id":"badge-1","name":"Mentor"}}}"#;
    let grant_body = r#"{"code":0,"msg":"ok","data":{"grant":{"id":"grant-1","name":"Grant"}}}"#;
    let image_body = r#"{"code":0,"msg":"ok","data":{"image_key":"img-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, empty_body),
        http_response(200, badge_body),
        http_response(200, badge_body),
        http_response(200, grant_body),
        http_response(200, empty_body),
        http_response(200, grant_body),
        http_response(200, image_body),
    ])
    .await;

    let client = client_for(addr);
    let reset_body = ResetPasswordReqBody {
        password: Some(serde_json::json!({"value": "secret-1"})),
        user_id: Some("ou-1".to_string()),
        user_id_type: Some("open_id".to_string()),
    };
    let badge_create_body = CreateBadgeReqBody {
        name: Some("Mentor".to_string()),
        explanation: Some("Recognizes mentors".to_string()),
        detail_image: Some("detail-img".to_string()),
        show_image: Some("show-img".to_string()),
    };
    let badge_update_body = CreateBadgeReqBody {
        name: Some("Mentor updated".to_string()),
        ..Default::default()
    };
    let grant_create_body = CreateBadgeGrantReqBody {
        name: Some("Grant".to_string()),
        grant_type: Some(1),
        user_ids: Some(vec!["ou-1".to_string()]),
        ..Default::default()
    };
    let grant_update_body = CreateBadgeGrantReqBody {
        name: Some("Grant updated".to_string()),
        department_ids: Some(vec!["od-1".to_string()]),
        ..Default::default()
    };
    let image_create_body = serde_json::json!({"image": "base64-image"});

    let reset_resp = client
        .admin()
        .password
        .reset_by_query(
            &ResetPasswordQuery::new(&reset_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let create_badge_resp = client
        .admin()
        .badge
        .create_by_query(
            &CreateBadgeQuery::new(&badge_create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_badge_resp = client
        .admin()
        .badge
        .update_by_query(
            &UpdateBadgeQuery::new("badge-1", &badge_update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let create_grant_resp = client
        .admin()
        .badge_grant
        .create_by_query(
            &CreateBadgeGrantQuery::new("badge-1", &grant_create_body)
                .user_id_type("open_id")
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete_grant_resp = client
        .admin()
        .badge_grant
        .delete_by_query(
            &DeleteBadgeGrantQuery::new("badge-1", "grant-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_grant_resp = client
        .admin()
        .badge_grant
        .update_by_query(
            &UpdateBadgeGrantQuery::new("badge-1", "grant-1", &grant_update_body)
                .user_id_type("open_id")
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let image_resp = client
        .admin()
        .badge_image
        .create_by_query(
            &CreateBadgeImageQuery::new(&image_create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(reset_resp.success());
    assert!(create_badge_resp.success());
    assert!(update_badge_resp.success());
    assert!(create_grant_resp.success());
    assert!(delete_grant_resp.success());
    assert!(update_grant_resp.success());
    assert!(image_resp.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/admin/v1/password/reset"));
    assert!(request.contains(r#""user_id":"ou-1""#));
    assert!(request.contains(r#""user_id_type":"open_id""#));
    assert!(request.contains("POST /open-apis/admin/v1/badges"));
    assert!(request.contains(r#""name":"Mentor""#));
    assert!(request.contains("PUT /open-apis/admin/v1/badges/badge-1"));
    assert!(request.contains(r#""name":"Mentor updated""#));
    assert!(request.contains("POST /open-apis/admin/v1/badges/badge-1/grants?"));
    assert!(request.contains("DELETE /open-apis/admin/v1/badges/badge-1/grants/grant-1"));
    assert!(request.contains("PUT /open-apis/admin/v1/badges/badge-1/grants/grant-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains(r#""user_ids":["ou-1"]"#));
    assert!(request.contains(r#""department_ids":["od-1"]"#));
    assert!(request.contains("POST /open-apis/admin/v1/badge_images"));
    assert!(request.contains(r#""image":"base64-image""#));
}
