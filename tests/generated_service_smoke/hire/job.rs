use super::prelude::*;

// ── Hire ──

#[tokio::test]
async fn hire_job_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"job":{"id":"job-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetHireJobQuery::new("job-1")
        .user_id_type("open_id")
        .department_id_type("open_department_id")
        .job_level_id_type("job_level_id")
        .job_family_id_type("job_family_id");
    let resp = client
        .hire()
        .job
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data.unwrap().job.unwrap().id.as_deref(), Some("job-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/jobs/job-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("job_family_id_type=job_family_id"));
}

#[tokio::test]
async fn hire_job_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"job-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireJobQuery::new()
        .update_start_time("1618500278663")
        .update_end_time("1618500279999")
        .user_id_type("open_id")
        .department_id_type("open_department_id")
        .job_level_id_type("job_level_id")
        .job_family_id_type("job_family_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .job
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("job-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/jobs?"));
    assert!(request.contains("update_start_time=1618500278663"));
    assert!(request.contains("update_end_time=1618500279999"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("job_family_id_type=job_family_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_job_get_detail_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"job_detail":{"basic_info":{"id":"job-1","title":"Engineer"},"recruiter":{"id":"ou_recruiter","name":{"en_us":"Recruiter"}},"job_config":{"id":"job-1","internship_offer_apply_schema":{"id":"intern-schema-1"}},"stage_count_list":[{"count":2,"stage":{"id":"stage-1"}}]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .hire()
        .job
        .get_detail("job-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let detail = resp.data.unwrap().job_detail.unwrap();
    assert_eq!(
        detail.basic_info.unwrap().title.as_deref(),
        Some("Engineer")
    );
    assert_eq!(
        detail.recruiter.unwrap().id.as_deref(),
        Some("ou_recruiter")
    );
    assert_eq!(
        detail
            .job_config
            .unwrap()
            .internship_offer_apply_schema
            .unwrap()
            .id
            .as_deref(),
        Some("intern-schema-1")
    );
    assert_eq!(detail.stage_count_list.unwrap()[0].count, Some(2));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/jobs/job-1/get_detail "));
}
