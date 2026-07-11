mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::req::RequestOption;
use serde_json::json;

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn hire_application_transfer_onboard_typed_response() {
    let transferred = r#"{"code":0,"msg":"ok","data":{"employee":{"id":"employee-1","application_id":"application-1","onboard_status":1,"department":"dept-1","leader":"ou_leader"}}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, transferred)]).await;

    let client = client_for(addr);
    let hire = client.hire();

    let employee = Box::pin(hire.application.transfer_onboard(
        "application-1",
        json!({"onboard_city_code":"CN_1","department":"dept-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .employee
    .unwrap();

    assert_eq!(employee.id.as_deref(), Some("employee-1"));
    assert_eq!(employee.application_id.as_deref(), Some("application-1"));
    assert_eq!(employee.onboard_status, Some(1));
    assert_eq!(employee.department.as_deref(), Some("dept-1"));
    assert_eq!(employee.leader.as_deref(), Some("ou_leader"));

    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("POST /open-apis/hire/v1/applications/application-1/transfer_onboard ")
    );
    assert!(request.contains(r#""onboard_city_code":"CN_1""#));
    assert!(request.contains(r#""department":"dept-1""#));
}

#[tokio::test]
async fn hire_job_recruiter_typed_response() {
    let recruiter = r#"{"code":0,"msg":"ok","data":{"info":{"id":"job-1","recruiter_id":"ou_recruiter","hiring_manager_id_list":["ou_hiring"],"assistant_id_list":["ou_assistant"]}}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, recruiter)]).await;

    let client = client_for(addr);
    let hire = client.hire();

    let info = Box::pin(hire.job.recruiter("job-1", &RequestOption::default()))
        .await
        .unwrap()
        .data
        .unwrap()
        .info
        .unwrap();

    assert_eq!(info.id.as_deref(), Some("job-1"));
    assert_eq!(info.recruiter_id.as_deref(), Some("ou_recruiter"));
    assert_eq!(
        info.hiring_manager_id_list.as_ref().unwrap()[0],
        "ou_hiring"
    );
    assert_eq!(info.assistant_id_list.as_ref().unwrap()[0], "ou_assistant");

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/jobs/job-1/recruiter "));
}
