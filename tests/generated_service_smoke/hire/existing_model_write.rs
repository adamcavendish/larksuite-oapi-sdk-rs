use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_job_existing_model_write_smoke() {
    let manager = r#"{"code":0,"msg":"ok","data":{"job_manager":{"id":"manager-1"}}}"#;
    let requirement = r#"{"code":0,"msg":"ok","data":{"job_requirement":{"id":"jr-1"}}}"#;
    let tripartite = r#"{"code":0,"msg":"ok","data":{"id":"agreement-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, manager),
        http_response(200, requirement),
        http_response(200, tripartite),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.job_manager.batch_update(
        "job-1",
        json!({"recruiter_id":"ou_recruiter"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.job_requirement.create(
        json!({"name":"Backend Engineer"}),
        Some("open_id"),
        Some("open_department_id"),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.tripartite_agreement.create(
        json!({"application_id":"application-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/jobs/job-1/managers/batch_update "));
    assert!(request.contains("POST /open-apis/hire/v1/job_requirements?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("POST /open-apis/hire/v1/tripartite_agreements "));
}

#[tokio::test]
async fn hire_referral_account_existing_model_write_smoke() {
    let created = r#"{"code":0,"msg":"ok","data":{"account":{"account_id":"account-1"}}}"#;
    let deactivated = r#"{"code":0,"msg":"ok","data":{"account":{"account_id":"account-1"}}}"#;
    let enabled = r#"{"code":0,"msg":"ok","data":{"account":{"account_id":"account-1"}}}"#;
    let reconciliation = r#"{"code":0,"msg":"ok","data":{"check_failed_list":[]}}"#;
    let withdrawn = r#"{"code":0,"msg":"ok","data":{"external_order_id":"withdraw-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, created),
        http_response(200, deactivated),
        http_response(200, enabled),
        http_response(200, reconciliation),
        http_response(200, withdrawn),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.referral_account.create(
        json!({"referrer_id":"ou_referrer"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(
        hire.referral_account
            .deactivate("account-1", &RequestOption::default()),
    )
    .await
    .unwrap();
    Box::pin(hire.referral_account.enable(
        json!({"referral_account_id":"account-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.referral_account.reconciliation(
        json!({"account_id_list":["account-2"]}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.referral_account.withdraw(
        "account-1",
        json!({"withdraw_bonus_type":[1],"external_order_id":"withdraw-1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/referral_account "));
    assert!(request.contains("POST /open-apis/hire/v1/referral_account/account-1/deactivate "));
    assert!(request.contains("POST /open-apis/hire/v1/referral_account/enable "));
    assert!(request.contains("POST /open-apis/hire/v1/referral_account/reconciliation "));
    assert!(request.contains("POST /open-apis/hire/v1/referral_account/account-1/withdraw "));
}
