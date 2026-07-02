use super::prelude::*;

// ── Admin v1 ──

#[tokio::test]
async fn admin_badge_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"badge":{"id":"badge-1","name":"Mentor"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .badge
        .get_by_query(&GetBadgeQuery::new("badge-1"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.badge.as_ref())
            .and_then(|badge| badge.id.as_deref()),
        Some("badge-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/badges/badge-1"));
}

#[tokio::test]
async fn admin_badge_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"badge-1","name":"Mentor"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .badge
        .list_by_query(
            &ListBadgeQuery::new()
                .page_size(20)
                .page_token("next-page")
                .name("Mentor"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|badge| badge.name.as_deref()),
        Some("Mentor")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/badges?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("name=Mentor"));
}

#[tokio::test]
async fn admin_badge_grant_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"grant":{"id":"grant-1","name":"Grant"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .badge_grant
        .get_by_query(
            &GetBadgeGrantQuery::new("badge-1", "grant-1")
                .user_id_type("open_id")
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.grant.as_ref())
            .and_then(|grant| grant.id.as_deref()),
        Some("grant-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/badges/badge-1/grants/grant-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
}

#[tokio::test]
async fn admin_badge_grant_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"grant-1","name":"Grant"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .badge_grant
        .list_by_query(
            &ListBadgeGrantQuery::new("badge-1")
                .page_size(20)
                .page_token("next-page")
                .user_id_type("open_id")
                .department_id_type("open_department_id")
                .name("Grant"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/badges/badge-1/grants?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("name=Grant"));
}

#[tokio::test]
async fn admin_dept_stat_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"date":"2026-06-01","department_id":"od-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .dept_stat
        .get_by_query(
            &GetAdminDeptStatQuery::new(
                "open_department_id",
                "2026-06-01",
                "2026-06-30",
                "od-1",
                true,
            )
            .page_size(20)
            .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|stat| stat.department_id.as_deref()),
        Some("od-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/dept_stats?"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("start_date=2026-06-01"));
    assert!(request.contains("end_date=2026-06-30"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("contains_child_dept=true"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn admin_dept_stat_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .dept_stat
        .list_by_query(
            &ListAdminDeptStatQuery::new(
                "open_department_id",
                "2026-06-01",
                "2026-06-30",
                "od-1",
                true,
            )
            .page_size(20)
            .page_token("next-page")
            .target_geo("cn")
            .with_product_version(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/admin_dept_stats?"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("start_date=2026-06-01"));
    assert!(request.contains("end_date=2026-06-30"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("contains_child_dept=true"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("target_geo=cn"));
    assert!(request.contains("with_product_version=true"));
}

#[tokio::test]
async fn admin_user_stat_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .user_stat
        .list_by_query(
            &ListAdminUserStatQuery::new("2026-06-01", "2026-06-30")
                .user_id_type("open_id")
                .department_id_type("open_department_id")
                .department_id("od-1")
                .user_id("ou-1")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/admin_user_stats?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("start_date=2026-06-01"));
    assert!(request.contains("end_date=2026-06-30"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn admin_audit_info_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"event_name":"login"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .audit_info
        .list_by_query(
            &ListAuditInfoQuery::new()
                .user_id_type("open_id")
                .latest("2026-06-30")
                .oldest("2026-06-01")
                .event_name("login")
                .operator_type("user")
                .operator_value("ou-1")
                .event_module(1)
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/audit_infos?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("latest=2026-06-30"));
    assert!(request.contains("oldest=2026-06-01"));
    assert!(request.contains("event_name=login"));
    assert!(request.contains("operator_type=user"));
    assert!(request.contains("operator_value=ou-1"));
    assert!(request.contains("event_module=1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

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
