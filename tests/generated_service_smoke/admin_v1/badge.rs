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
