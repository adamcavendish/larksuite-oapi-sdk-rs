use super::prelude::*;

// ── Admin v1 ──

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
