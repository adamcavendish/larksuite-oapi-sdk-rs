use super::prelude::*;

// ── Application v6 ──

#[tokio::test]
async fn application_v6_utility_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let badge_body = serde_json::json!({"user_id":"ou-1","badge_count":3});
    client
        .application_v6()
        .app_badge
        .set_by_query(
            &SetAppBadgeQuery::new(&badge_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .application_v6()
        .scope
        .apply_by_query(&ApplyScopeQuery::new(), &RequestOption::default())
        .await
        .unwrap();
    client
        .application_v6()
        .scope
        .list_by_query(
            &ListApplicationV6ScopeQuery::new(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/application/v6/app_badge/set?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""badge_count":3"#));
    assert!(request.contains("POST /open-apis/application/v6/scopes/apply "));
    assert!(request.contains("GET /open-apis/application/v6/scopes "));
}
