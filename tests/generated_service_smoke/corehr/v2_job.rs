use super::prelude::*;

// CoreHR v2 job smoke tests

#[tokio::test]
async fn corehr_v2_job_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"job-v2-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrJobV2Query::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .job
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("job-v2-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/jobs?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_workforce_plan_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"workforce-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrWorkforcePlanV2Query::new()
        .limit(10)
        .offset(1)
        .get_all_plan(true)
        .active(true)
        .start_date("2026-01-01")
        .end_date("2026-12-31")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .workforce_plan
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("workforce-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/workforce_plans?"));
    assert!(request.contains("limit=10"));
    assert!(request.contains("offset=1"));
    assert!(request.contains("get_all_plan=true"));
    assert!(request.contains("active=true"));
    assert!(request.contains("start_date=2026-01-01"));
    assert!(request.contains("end_date=2026-12-31"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
