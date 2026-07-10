use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_job_response_smoke() {
    let config = r#"{"code":0,"msg":"ok","data":{"job_config":{"id":"job-1","interview_registration":{"schema_id":"schema-1","name":"Interview"}}}}"#;
    let combined_create = r#"{"code":0,"msg":"ok","data":{"default_job_post":{"id":"post-1"},"job":{"id":"job-1"},"target_major_list":[{"id":"major-1","zh_name":"Computer Science"}]}}"#;
    let combined_update = r#"{"code":0,"msg":"ok","data":{"job":{"id":"job-1"},"onboard_registration_schema_info":{"schema_id":"schema-2","name":"Onboard"}}}"#;
    let update_config =
        r#"{"code":0,"msg":"ok","data":{"job_config":{"id":"job-1","job_attribute":2}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, config),
        http_response(200, combined_create),
        http_response(200, combined_update),
        http_response(200, update_config),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.job.config("job-1", &RequestOption::default()))
        .await
        .unwrap();
    Box::pin(hire.job.combined_create(
        json!({"job":{"title":"Engineer"}}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.job.combined_update(
        "job-1",
        json!({"job":{"title":"Staff Engineer"}}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.job.update_config(
        "job-1",
        json!({"job_attribute":2}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/jobs/job-1/config "));
    assert!(request.contains("POST /open-apis/hire/v1/jobs/combined_create "));
    assert!(request.contains("POST /open-apis/hire/v1/jobs/job-1/combined_update "));
    assert!(request.contains("POST /open-apis/hire/v1/jobs/job-1/update_config "));
}
