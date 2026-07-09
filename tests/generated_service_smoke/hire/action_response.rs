use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_action_response_smoke() {
    let transferred = r#"{"code":0,"msg":"ok","data":{"employee":{"id":"employee-1","application_id":"application-1"}}}"#;
    let recruiter =
        r#"{"code":0,"msg":"ok","data":{"info":{"id":"job-1","recruiter_id":"ou_recruiter"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, transferred),
        http_response(200, recruiter),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.application.transfer_onboard(
        "application-1",
        json!({"onboard_city_code":"CN_1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.job.recruiter("job-1", &RequestOption::default()))
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("POST /open-apis/hire/v1/applications/application-1/transfer_onboard ")
    );
    assert!(request.contains("GET /open-apis/hire/v1/jobs/job-1/recruiter "));
}
