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
async fn hire_external_application_and_background_write_responses() {
    let created_application = r#"{"code":0,"msg":"ok","data":{"external_application":{"id":"external-application-1","external_id":"ext-app-1","talent_id":"talent-1","job_title":"Backend Engineer"}}}"#;
    let updated_application = r#"{"code":0,"msg":"ok","data":{"external_application":{"id":"external-application-1","external_id":"ext-app-1-updated","stage":"interview"}}}"#;
    let created_background_check = r#"{"code":0,"msg":"ok","data":{"external_background_check":{"id":"background-1","external_id":"ext-bg-1","external_application_id":"external-application-1","name":"Vendor Check","attachment_list":[{"id":"file-1","name":"report.pdf","size":42}]}}}"#;
    let updated_background_check = r#"{"code":0,"msg":"ok","data":{"external_background_check":{"id":"background-1","result":"pass"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, created_application),
        http_response(200, updated_application),
        http_response(200, created_background_check),
        http_response(200, updated_background_check),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    let created_application = Box::pin(hire.external_application.create(
        json!({"external_id":"ext-app-1","talent_id":"talent-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .external_application
    .unwrap();
    let updated_application = Box::pin(hire.external_application.update(
        "external-application-1",
        json!({"stage":"interview"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .external_application
    .unwrap();
    let created_background_check = Box::pin(hire.external_background_check.create(
        json!({"external_id":"ext-bg-1","external_application_id":"external-application-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .external_background_check
    .unwrap();
    let updated_background_check = Box::pin(hire.external_background_check.update(
        "background-1",
        json!({"result":"pass"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .external_background_check
    .unwrap();

    assert_eq!(
        created_application.id.as_deref(),
        Some("external-application-1")
    );
    assert_eq!(updated_application.stage.as_deref(), Some("interview"));
    assert_eq!(
        created_background_check.attachment_list.as_ref().unwrap()[0]
            .name
            .as_deref(),
        Some("report.pdf")
    );
    assert_eq!(updated_background_check.result.as_deref(), Some("pass"));

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
async fn hire_external_interview_and_offer_write_responses() {
    let created_interview = r#"{"code":0,"msg":"ok","data":{"external_interview":{"id":"interview-1","external_id":"ext-interview-1","external_application_id":"external-application-1","participate_status":1}}}"#;
    let updated_interview = r#"{"code":0,"msg":"ok","data":{"external_interview":{"id":"interview-1","participate_status":2}}}"#;
    let created_offer = r#"{"code":0,"msg":"ok","data":{"external_offer":{"id":"offer-1","external_id":"ext-offer-1","external_application_id":"external-application-1","owner":"ou_owner"}}}"#;
    let updated_offer = r#"{"code":0,"msg":"ok","data":{"external_offer":{"id":"offer-1","creator":"ou_creator"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, created_interview),
        http_response(200, updated_interview),
        http_response(200, created_offer),
        http_response(200, updated_offer),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    let created_interview = Box::pin(hire.external_interview.create(
        json!({"external_id":"ext-interview-1","external_application_id":"external-application-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .external_interview
    .unwrap();
    let updated_interview = Box::pin(hire.external_interview.update(
        "interview-1",
        json!({"participate_status":2}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .external_interview
    .unwrap();
    let created_offer = Box::pin(hire.external_offer.create(
        json!({"external_id":"ext-offer-1","external_application_id":"external-application-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .external_offer
    .unwrap();
    let updated_offer = Box::pin(hire.external_offer.update(
        "offer-1",
        json!({"creator":"ou_creator"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .external_offer
    .unwrap();

    assert_eq!(created_interview.participate_status, Some(1));
    assert_eq!(updated_interview.participate_status, Some(2));
    assert_eq!(created_offer.owner.as_deref(), Some("ou_owner"));
    assert_eq!(updated_offer.creator.as_deref(), Some("ou_creator"));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/external_interviews "));
    assert!(request.contains("PUT /open-apis/hire/v1/external_interviews/interview-1 "));
    assert!(request.contains("POST /open-apis/hire/v1/external_offers "));
    assert!(request.contains("PUT /open-apis/hire/v1/external_offers/offer-1 "));
    assert!(request.contains(r#""participate_status":2"#));
    assert!(request.contains(r#""creator":"ou_creator""#));
}

#[tokio::test]
async fn hire_external_assessment_and_reward_responses() {
    let created_assessment = r#"{"code":0,"msg":"ok","data":{"external_interview_assessment":{"id":"assessment-1","external_id":"ext-assessment-1","username":"Interviewer","assessment_dimension_list":[{"score":5,"title":"Rust"}]}}}"#;
    let patched_assessment = r#"{"code":0,"msg":"ok","data":{"external_interview_assessment":{"id":"assessment-1","conclusion":1,"content":"strong"}}}"#;
    let reward = r#"{"code":0,"msg":"ok","data":{"id":"reward-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, created_assessment),
        http_response(200, patched_assessment),
        http_response(200, reward),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    let created_assessment = Box::pin(hire.external_interview_assessment.create(
        json!({"external_id":"ext-assessment-1","external_interview_id":"interview-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .external_interview_assessment
    .unwrap();
    let patched_assessment = Box::pin(hire.external_interview_assessment.patch(
        "assessment-1",
        json!({"content":"strong"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .external_interview_assessment
    .unwrap();
    let reward = Box::pin(hire.external_referral_reward.create(
        json!({"external_id":"reward-ext-1","application_id":"application-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap();

    assert_eq!(
        created_assessment
            .assessment_dimension_list
            .as_ref()
            .unwrap()[0]
            .title
            .as_deref(),
        Some("Rust")
    );
    assert_eq!(patched_assessment.conclusion, Some(1));
    assert_eq!(reward.id.as_deref(), Some("reward-1"));

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
async fn hire_talent_external_info_and_site_user_responses() {
    let created_external_info = r#"{"code":0,"msg":"ok","data":{"external_info":{"talent_id":"talent-1","external_create_time":"1710000000000"}}}"#;
    let updated_external_info = r#"{"code":0,"msg":"ok","data":{"external_info":{"talent_id":"talent-1","external_create_time":"1710003600000"}}}"#;
    let site_user = r#"{"code":0,"msg":"ok","data":{"site_user":{"user_id":"site-user-1","name":"Candidate","email":"candidate@example.test","external_id":"external-user-1","mobile":"13800000000","mobile_country_code":"86"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, created_external_info),
        http_response(200, updated_external_info),
        http_response(200, site_user),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    let created_external_info = Box::pin(hire.talent_external_info.create(
        "talent-1",
        json!({"external_create_time":"1710000000000"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .external_info
    .unwrap();
    let updated_external_info = Box::pin(hire.talent_external_info.update(
        "talent-1",
        json!({"external_create_time":"1710003600000"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .external_info
    .unwrap();
    let site_user = Box::pin(hire.website_site_user.create(
        "website-1",
        json!({"external_id":"external-user-1","email":"candidate@example.test"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .site_user
    .unwrap();

    assert_eq!(
        created_external_info.external_create_time.as_deref(),
        Some("1710000000000")
    );
    assert_eq!(
        updated_external_info.external_create_time.as_deref(),
        Some("1710003600000")
    );
    assert_eq!(site_user.email.as_deref(), Some("candidate@example.test"));
    assert_eq!(site_user.mobile_country_code.as_deref(), Some("86"));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/talents/talent-1/external_info "));
    assert!(request.contains("PUT /open-apis/hire/v1/talents/talent-1/external_info "));
    assert!(request.contains("POST /open-apis/hire/v1/websites/website-1/site_users "));
    assert!(request.contains(r#""external_create_time":"1710003600000""#));
    assert!(request.contains(r#""email":"candidate@example.test""#));
}
