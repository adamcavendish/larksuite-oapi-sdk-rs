use super::prelude::*;

// ── Hire ──

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
    assert_eq!(data.items[0].id.as_deref(), Some("app-interview-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/app-1/interviews?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_interview_support_list_by_query_smoke() {
    let interviewer_body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou_user_1","verify_status":1}],"page_token":"next-page","has_more":false}}"#;
    let feedback_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"form-1","version":1,"name":{"en_us":"Default"},"type":1}],"page_token":"next-page","has_more":false}}"#;
    let schema_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"schema-1","name":"登记表","is_used_as_interview":true}],"page_token":"next-page","has_more":false}}"#;
    let round_body = r#"{"code":0,"msg":"ok","data":{"active_status":1,"items":[{"id":"round-1","name":{"en_us":"Technical"},"process_type":1}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, interviewer_body),
        http_response(200, feedback_body),
        http_response(200, schema_body),
        http_response(200, round_body),
    ])
    .await;

    let client = client_for(addr);
    let user_ids = ["ou_user_1", "ou_user_2"];
    let form_ids = ["form-1"];
    let page = PageQuery::new().page_size(20).page_token("next-page");

    client
        .hire()
        .interviewer
        .list_by_query(
            &ListHireInterviewerQuery::new()
                .page(page)
                .user_ids(Some(user_ids.as_slice()))
                .verify_status(1)
                .earliest_update_time("1710000000")
                .latest_update_time("1710009999")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire()
        .interview_feedback_form
        .list_by_query(
            &ListHireInterviewFeedbackFormQuery::new()
                .page(page)
                .interview_feedback_form_ids(Some(form_ids.as_slice()))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire()
        .interview_registration_schema
        .list_by_query(
            &ListHireInterviewRegistrationSchemaQuery::new()
                .page(page)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire()
        .interview_round_type
        .list_by_query(
            &ListHireInterviewRoundTypeQuery::new().process_type(1),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/interviewers?"));
    assert!(request.contains("GET /open-apis/hire/v1/interview_feedback_forms?"));
    assert!(request.contains("GET /open-apis/hire/v1/interview_registration_schemas?"));
    assert!(request.contains("GET /open-apis/hire/v1/interview_round_types?"));
    assert_eq!(request.matches("page_size=20").count(), 3);
    assert_eq!(request.matches("page_token=next-page").count(), 3);
    assert!(request.contains("user_ids=ou_user_1"));
    assert!(request.contains("user_ids=ou_user_2"));
    assert!(request.contains("verify_status=1"));
    assert!(request.contains("interview_feedback_form_ids=form-1"));
    assert!(request.contains("process_type=1"));
}
