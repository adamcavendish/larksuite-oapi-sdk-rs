use super::prelude::*;

// ── Hire ──

#[tokio::test]
async fn hire_job_requirement_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"requirement-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireJobRequirementQuery::new()
        .job_id("job-1")
        .create_time_begin("1658980233000")
        .create_time_end("1658980233999")
        .update_time_begin("1658980234000")
        .update_time_end("1658980234999")
        .user_id_type("open_id")
        .department_id_type("open_department_id")
        .job_level_id_type("job_level_id")
        .job_family_id_type("job_family_id")
        .employee_type_id_type("employee_type_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .job_requirement
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("requirement-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/job_requirements?"));
    assert!(request.contains("job_id=job-1"));
    assert!(request.contains("create_time_begin=1658980233000"));
    assert!(request.contains("create_time_end=1658980233999"));
    assert!(request.contains("update_time_begin=1658980234000"));
    assert!(request.contains("update_time_end=1658980234999"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("job_family_id_type=job_family_id"));
    assert!(request.contains("employee_type_id_type=employee_type_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_job_requirement_list_positional_adapter_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"requirement-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .hire()
        .job_requirement
        .list(
            Some(20),
            Some("next-page"),
            Some("job-1"),
            Some("1658980233000"),
            Some("1658980233999"),
            Some("1658980234000"),
            Some("1658980234999"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("requirement-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/job_requirements?"));
    assert!(request.contains("job_id=job-1"));
    assert!(request.contains("create_time_begin=1658980233000"));
    assert!(request.contains("create_time_end=1658980233999"));
    assert!(request.contains("update_time_begin=1658980234000"));
    assert!(request.contains("update_time_end=1658980234999"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
