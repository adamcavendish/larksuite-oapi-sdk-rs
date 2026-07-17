use super::prelude::*;

// CoreHR offboarding smoke tests

#[tokio::test]
async fn corehr_offboarding_search_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"offboarding_id":"off-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SearchCorehrOffboardingQuery::new()
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let body = SearchOffboardingReqBody {
        employment_ids: Some(vec!["emp-1".into()]),
        ..Default::default()
    };
    let resp = client
        .corehr()
        .offboarding
        .search_by_query(&query, body, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.items.unwrap()[0].offboarding_id.as_deref(),
        Some("off-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/corehr/v1/offboardings/search?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""employment_ids":["emp-1"]"#));
}
