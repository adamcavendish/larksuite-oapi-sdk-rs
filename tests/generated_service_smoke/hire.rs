use super::*;

// ── Hire ──

#[tokio::test]
async fn hire_v2_by_query_smoke() {
    let record_body = r#"{"code":0,"msg":"ok","data":{"interview_record":{"id":"record-1"}}}"#;
    let list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"record-1"}],"has_more":false}}"#;
    let talent_body = r#"{"code":0,"msg":"ok","data":{"talent":{"id":"talent-v2-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, record_body),
        http_response(200, list_body),
        http_response(200, talent_body),
    ])
    .await;

    let client = client_for(addr);

    client
        .hire_v2()
        .interview_record
        .get_by_query(
            &GetHireV2InterviewRecordQuery::new("record-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire_v2()
        .interview_record
        .list_by_query(
            &ListHireV2InterviewRecordQuery::new()
                .interview_id("interview-1")
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire_v2()
        .talent
        .get_by_query(
            &GetHireV2TalentQuery::new("talent-v2-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v2/interview_records/record-1?"));
    assert!(request.contains("GET /open-apis/hire/v2/interview_records?"));
    assert!(request.contains("GET /open-apis/hire/v2/talents/talent-v2-1?"));
    assert!(request.contains("interview_id=interview-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

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
async fn hire_talent_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"talent":{"id":"talent-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetHireTalentQuery::new("talent-1").user_id_type("open_id");
    let resp = client
        .hire()
        .talent
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().talent.unwrap().id.as_deref(),
        Some("talent-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/talents/talent-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_talent_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"talent-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireTalentQuery::new()
        .keyword("product")
        .update_start_time("1618500278663")
        .update_end_time("1618500279999")
        .sort_by(1)
        .query_option("ignore_empty_error")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .talent
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("talent-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/talents?"));
    assert!(request.contains("keyword=product"));
    assert!(request.contains("update_start_time=1618500278663"));
    assert!(request.contains("update_end_time=1618500279999"));
    assert!(request.contains("sort_by=1"));
    assert!(request.contains("query_option=ignore_empty_error"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

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
async fn hire_interview_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"interview-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireInterviewQuery::new()
        .application_id("app-1")
        .interview_id("interview-1")
        .start_time("1609489908000")
        .end_time("1610489908000")
        .user_id_type("open_id")
        .job_level_id_type("job_level_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .interview
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("interview-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/interviews?"));
    assert!(request.contains("application_id=app-1"));
    assert!(request.contains("interview_id=interview-1"));
    assert!(request.contains("start_time=1609489908000"));
    assert!(request.contains("end_time=1610489908000"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_offer_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"offer":{"id":"offer-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetHireOfferQuery::new("offer-1")
        .user_id_type("open_id")
        .department_id_type("open_department_id")
        .job_level_id_type("job_level_id")
        .job_family_id_type("job_family_id")
        .employee_type_id_type("employee_type_id");
    let resp = client
        .hire()
        .offer
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().offer.unwrap().id.as_deref(),
        Some("offer-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/offers/offer-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("job_family_id_type=job_family_id"));
    assert!(request.contains("employee_type_id_type=employee_type_id"));
}

#[tokio::test]
async fn hire_offer_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"offer-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireOfferQuery::new()
        .application_id("app-1")
        .talent_id("talent-1")
        .user_id_type("open_id")
        .employee_type_id_type("employee_type_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .offer
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("offer-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/offers?"));
    assert!(request.contains("application_id=app-1"));
    assert!(request.contains("talent_id=talent-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("employee_type_id_type=employee_type_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

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

#[tokio::test]
async fn hire_application_interview_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"app-interview-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireApplicationInterviewQuery::new("app-1")
        .user_id_type("open_id")
        .job_level_id_type("job_level_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .application_interview
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("app-interview-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/app-1/interviews?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_evaluation_task_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"evaluation-task-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireEvaluationTaskQuery::new()
        .user_id("ou_user_1")
        .activity_status(1)
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .evaluation_task
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("evaluation-task-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/evaluation_tasks?"));
    assert!(request.contains("user_id=ou_user_1"));
    assert!(request.contains("activity_status=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_exam_marking_task_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"exam-task-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireExamMarkingTaskQuery::new()
        .user_id("ou_user_1")
        .activity_status(1)
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .exam_marking_task
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("exam-task-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/exam_marking_tasks?"));
    assert!(request.contains("user_id=ou_user_1"));
    assert!(request.contains("activity_status=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_interview_task_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"interview-task-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireInterviewTaskQuery::new()
        .user_id("ou_user_1")
        .activity_status(1)
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .interview_task
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("interview-task-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/interview_tasks?"));
    assert!(request.contains("user_id=ou_user_1"));
    assert!(request.contains("activity_status=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
