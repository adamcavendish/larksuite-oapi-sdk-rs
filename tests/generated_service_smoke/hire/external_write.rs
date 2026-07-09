use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_external_application_and_background_write_smoke() {
    let created_application =
        r#"{"code":0,"msg":"ok","data":{"external_application":{"id":"external-application-1"}}}"#;
    let updated_application =
        r#"{"code":0,"msg":"ok","data":{"external_application":{"id":"external-application-1"}}}"#;
    let created_background_check =
        r#"{"code":0,"msg":"ok","data":{"external_background_check":{"id":"background-1"}}}"#;
    let updated_background_check =
        r#"{"code":0,"msg":"ok","data":{"external_background_check":{"id":"background-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, created_application),
        http_response(200, updated_application),
        http_response(200, created_background_check),
        http_response(200, updated_background_check),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.external_application.create(
        json!({"external_id":"ext-app-1","talent_id":"talent-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.external_application.update(
        "external-application-1",
        json!({"stage":"interview"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.external_background_check.create(
        json!({"external_id":"ext-bg-1","external_application_id":"external-application-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.external_background_check.update(
        "background-1",
        json!({"result":"pass"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/external_applications "));
    assert!(
        request.contains("PUT /open-apis/hire/v1/external_applications/external-application-1 ")
    );
    assert!(request.contains("POST /open-apis/hire/v1/external_background_checks "));
    assert!(request.contains("PUT /open-apis/hire/v1/external_background_checks/background-1 "));
    assert!(request.contains(r#""external_id":"ext-app-1""#));
    assert!(request.contains(r#""result":"pass""#));
}

#[tokio::test]
async fn hire_external_interview_and_offer_write_smoke() {
    let created_interview =
        r#"{"code":0,"msg":"ok","data":{"external_interview":{"id":"interview-1"}}}"#;
    let updated_interview =
        r#"{"code":0,"msg":"ok","data":{"external_interview":{"id":"interview-1"}}}"#;
    let created_offer = r#"{"code":0,"msg":"ok","data":{"external_offer":{"id":"offer-1"}}}"#;
    let updated_offer = r#"{"code":0,"msg":"ok","data":{"external_offer":{"id":"offer-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, created_interview),
        http_response(200, updated_interview),
        http_response(200, created_offer),
        http_response(200, updated_offer),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.external_interview.create(
        json!({"external_id":"ext-interview-1","external_application_id":"external-application-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.external_interview.update(
        "interview-1",
        json!({"participate_status":2}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.external_offer.create(
        json!({"external_id":"ext-offer-1","external_application_id":"external-application-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.external_offer.update(
        "offer-1",
        json!({"creator":"ou_creator"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/external_interviews "));
    assert!(request.contains("PUT /open-apis/hire/v1/external_interviews/interview-1 "));
    assert!(request.contains("POST /open-apis/hire/v1/external_offers "));
    assert!(request.contains("PUT /open-apis/hire/v1/external_offers/offer-1 "));
    assert!(request.contains(r#""participate_status":2"#));
    assert!(request.contains(r#""creator":"ou_creator""#));
}

#[tokio::test]
async fn hire_external_assessment_and_reward_smoke() {
    let created_assessment =
        r#"{"code":0,"msg":"ok","data":{"external_interview_assessment":{"id":"assessment-1"}}}"#;
    let patched_assessment =
        r#"{"code":0,"msg":"ok","data":{"external_interview_assessment":{"id":"assessment-1"}}}"#;
    let reward = r#"{"code":0,"msg":"ok","data":{"id":"reward-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, created_assessment),
        http_response(200, patched_assessment),
        http_response(200, reward),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.external_interview_assessment.create(
        json!({"external_id":"ext-assessment-1","external_interview_id":"interview-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.external_interview_assessment.patch(
        "assessment-1",
        json!({"content":"strong"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.external_referral_reward.create(
        json!({"external_id":"reward-ext-1","application_id":"application-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/external_interview_assessments "));
    assert!(
        request.contains("PATCH /open-apis/hire/v1/external_interview_assessments/assessment-1 ")
    );
    assert!(request.contains("POST /open-apis/hire/v1/external_referral_rewards "));
    assert!(request.contains(r#""content":"strong""#));
    assert!(request.contains(r#""application_id":"application-1""#));
}

#[tokio::test]
async fn hire_talent_external_info_and_site_user_smoke() {
    let created_external_info =
        r#"{"code":0,"msg":"ok","data":{"external_info":{"talent_id":"talent-1"}}}"#;
    let updated_external_info =
        r#"{"code":0,"msg":"ok","data":{"external_info":{"talent_id":"talent-1"}}}"#;
    let site_user = r#"{"code":0,"msg":"ok","data":{"site_user":{"user_id":"site-user-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, created_external_info),
        http_response(200, updated_external_info),
        http_response(200, site_user),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.talent_external_info.create(
        "talent-1",
        json!({"external_create_time":"1710000000000"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.talent_external_info.update(
        "talent-1",
        json!({"external_create_time":"1710003600000"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.website_site_user.create(
        "website-1",
        json!({"external_id":"external-user-1","email":"candidate@example.test"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/talents/talent-1/external_info "));
    assert!(request.contains("PUT /open-apis/hire/v1/talents/talent-1/external_info "));
    assert!(request.contains("POST /open-apis/hire/v1/websites/website-1/site_users "));
    assert!(request.contains(r#""external_create_time":"1710003600000""#));
    assert!(request.contains(r#""email":"candidate@example.test""#));
}
