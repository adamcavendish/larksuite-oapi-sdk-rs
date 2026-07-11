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
async fn hire_talent_and_job_requirement_code_only_responses_preserve_requests() {
    let empty = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, empty),
        http_response(200, empty),
        http_response(200, empty),
    ])
    .await;

    let client = client_for(addr);
    let onboard = client
        .hire()
        .talent
        .onboard_status("talent-1", json!({"status": 1}), &RequestOption::default())
        .await
        .unwrap();
    let tag = client
        .hire()
        .talent
        .tag(
            "talent-1",
            json!({"tag_id_list": ["tag-1"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update = client
        .hire()
        .job_requirement
        .update(
            "requirement-1",
            json!({"name": "Platform"}),
            Some("open_id"),
            Some("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(onboard.success());
    assert!(tag.success());
    assert!(update.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/talents/talent-1/onboard_status "));
    assert!(request.contains("POST /open-apis/hire/v1/talents/talent-1/tag "));
    assert!(request.contains("PUT /open-apis/hire/v1/job_requirements/requirement-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("tag_id_list"));
}

#[tokio::test]
async fn hire_agreement_advertisement_and_agency_code_only_responses_preserve_requests() {
    let empty = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, empty),
        http_response(200, empty),
        http_response(200, empty),
    ])
    .await;

    let client = client_for(addr);
    let agreement = client
        .hire()
        .tripartite_agreement
        .update(
            "agreement-1",
            json!({"application_id": "application-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let advertisement = client
        .hire()
        .advertisement
        .publish(
            "advertisement-1",
            json!({"status": 1}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let protect = client
        .hire()
        .agency
        .protect(json!({"agency_id": "agency-1"}), &RequestOption::default())
        .await
        .unwrap();

    assert!(agreement.success());
    assert!(advertisement.success());
    assert!(protect.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PUT /open-apis/hire/v1/tripartite_agreements/agreement-1 "));
    assert!(request.contains("POST /open-apis/hire/v1/advertisements/advertisement-1/publish "));
    assert!(request.contains("POST /open-apis/hire/v1/agencies/protect "));
    assert!(request.contains("application_id"));
    assert!(request.contains("agency_id"));
}

#[tokio::test]
async fn hire_ehr_and_offer_field_code_only_responses_preserve_requests() {
    let empty = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, empty), http_response(200, empty)]).await;

    let client = client_for(addr);
    let ehr = client
        .hire()
        .ehr_import_task
        .patch(
            "ehr-task-1",
            json!({"status": 1}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let field = client
        .hire()
        .offer_custom_field
        .update(
            "field-1",
            json!({"name": "Start date"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(ehr.success());
    assert!(field.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/hire/v1/ehr_import_tasks/ehr-task-1 "));
    assert!(request.contains("PUT /open-apis/hire/v1/offer_custom_fields/field-1 "));
    assert!(request.contains("Start date"));
}
