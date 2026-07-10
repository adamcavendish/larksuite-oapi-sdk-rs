use super::prelude::*;

// ── Hire ──

#[tokio::test]
async fn hire_application_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"application":{"id":"app-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetHireApplicationQuery::new("app-1").user_id_type("open_id");
    let resp = client
        .hire()
        .application
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().application.unwrap().id.as_deref(),
        Some("app-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/app-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_application_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"app-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireApplicationQuery::new()
        .process_id("process-1")
        .job_id("job-1")
        .stage_id("stage-1")
        .talent_id("talent-1")
        .active_status(1)
        .user_id_type("open_id")
        .update_start_time("1618500278663")
        .update_end_time("1618500279999")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .application
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("app-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications?"));
    assert!(request.contains("process_id=process-1"));
    assert!(request.contains("job_id=job-1"));
    assert!(request.contains("stage_id=stage-1"));
    assert!(request.contains("talent_id=talent-1"));
    assert!(request.contains("active_status=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("update_start_time=1618500278663"));
    assert!(request.contains("update_end_time=1618500279999"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_application_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"app-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .hire()
        .application
        .list(
            Some(20),
            Some("next-page"),
            Some("job-1"),
            Some("stage-1"),
            Some("talent-1"),
            Some(1),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("app-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications?"));
    assert!(request.contains("job_id=job-1"));
    assert!(request.contains("stage_id=stage-1"));
    assert!(request.contains("talent_id=talent-1"));
    assert!(request.contains("active_status=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_application_offer_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"offer":{"id":"offer-1","application_id":"app-1","job_info":{"job_id":"job-1","job_name":"Engineer"}}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .hire()
        .application
        .offer("app-1", Some("open_id"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .unwrap()
            .offer
            .unwrap()
            .job_info
            .unwrap()
            .job_name
            .as_deref(),
        Some("Engineer")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/app-1/offer?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_application_lifecycle_smoke() {
    let detail = r#"{"code":0,"msg":"ok","data":{"application_detail":{"basic_info":{"id":"app-1","stage":{"id":"stage-1","type":2}},"job":{"id":"job-1","name":"Engineer"},"talent":{"id":"talent-1","name":"Taylor"}}}}"#;
    let empty = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, detail),
        http_response(200, empty),
        http_response(200, empty),
    ])
    .await;

    let client = client_for(addr);
    let detail = client
        .hire()
        .application
        .get_detail("app-1", &RequestOption::default())
        .await
        .unwrap();
    let cancel_onboard = client
        .hire()
        .application
        .cancel_onboard(
            "app-1",
            serde_json::json!({"termination_type": 1}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let recover = client
        .hire()
        .application
        .recover("app-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(
        detail
            .data
            .unwrap()
            .application_detail
            .unwrap()
            .job
            .unwrap()
            .name
            .as_deref(),
        Some("Engineer")
    );
    assert!(cancel_onboard.success());
    assert!(recover.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/app-1/get_detail "));
    assert!(request.contains("POST /open-apis/hire/v1/applications/app-1/cancel_onboard "));
    assert!(request.contains("POST /open-apis/hire/v1/applications/app-1/recover "));
}
