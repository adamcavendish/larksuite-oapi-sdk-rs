use super::prelude::*;

// ── Application v6 ──

#[tokio::test]
async fn application_v6_app_recommend_rule_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"rule-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .app_recommend_rule
        .list_by_query(
            &ListAppRecommendRuleQuery::new()
                .page_size(20)
                .page_token("next-page")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.get("items"))
            .and_then(|items| items.as_array())
            .map(Vec::len),
        Some(1)
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/app_recommend_rules?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}
