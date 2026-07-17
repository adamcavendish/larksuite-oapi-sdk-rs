use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_talent_and_job_requirement_code_only_response_smoke() {
    let empty = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, empty),
        http_response(200, empty),
        http_response(200, empty),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.talent.onboard_status(
        "talent-1",
        OnboardStatusTalentReqBody {
            operation: Some(1),
            ..Default::default()
        },
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.talent.tag(
        "talent-1",
        TagTalentReqBody {
            tag_id_list: Some(vec!["tag-1".into()]),
            ..Default::default()
        },
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.job_requirement.update(
        "requirement-1",
        json!({"name": "Platform"}),
        Some("open_id"),
        Some("open_department_id"),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/talents/talent-1/onboard_status "));
    assert!(request.contains("POST /open-apis/hire/v1/talents/talent-1/tag "));
    assert!(request.contains("PUT /open-apis/hire/v1/job_requirements/requirement-1?"));
}

#[tokio::test]
async fn hire_agreement_advertisement_and_agency_code_only_response_smoke() {
    let empty = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, empty),
        http_response(200, empty),
        http_response(200, empty),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.tripartite_agreement.update(
        "agreement-1",
        json!({"application_id": "application-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.advertisement.publish(
        "advertisement-1",
        json!({"status": 1}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(
        hire.agency
            .protect(json!({"agency_id": "agency-1"}), &RequestOption::default()),
    )
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PUT /open-apis/hire/v1/tripartite_agreements/agreement-1 "));
    assert!(request.contains("POST /open-apis/hire/v1/advertisements/advertisement-1/publish "));
    assert!(request.contains("POST /open-apis/hire/v1/agencies/protect "));
}

#[tokio::test]
async fn hire_ehr_and_offer_field_code_only_response_smoke() {
    let empty = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, empty), http_response(200, empty)]).await;

    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.ehr_import_task.patch(
        "ehr-task-1",
        json!({"status": 1}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.offer_custom_field.update(
        "field-1",
        json!({"name": "Start date"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/hire/v1/ehr_import_tasks/ehr-task-1 "));
    assert!(request.contains("PUT /open-apis/hire/v1/offer_custom_fields/field-1 "));
}

#[tokio::test]
async fn hire_agency_and_talent_pool_code_only_response_smoke() {
    let responses = (0..3)
        .map(|_| http_response(200, r#"{"code":0,"msg":"ok"}"#))
        .collect();
    let (addr, _handle, requests) = mock_server_with_requests(responses).await;

    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.agency.operate_agency_account(
        json!({"account_id": "account-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(
        hire.talent_blocklist
            .change_talent_block(json!({"talent_id": "talent-1"}), &RequestOption::default()),
    )
    .await
    .unwrap();
    Box::pin(hire.talent_pool.batch_change_talent_pool(
        "pool-1",
        json!({"talent_id_list": ["talent-1"]}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/agencies/operate_agency_account "));
    assert!(request.contains("POST /open-apis/hire/v1/talent_blocklist/change_talent_block "));
    assert!(
        request.contains("POST /open-apis/hire/v1/talent_pools/pool-1/batch_change_talent_pool ")
    );
}
