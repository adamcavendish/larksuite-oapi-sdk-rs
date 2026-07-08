mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use serde_json::json;

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn hire_job_existing_model_write_responses() {
    let manager = r#"{"code":0,"msg":"ok","data":{"job_manager":{"id":"manager-1","recruiter_id":"ou_recruiter","hiring_manager_id_list":["ou_hiring"],"assistant_id_list":["ou_assistant"]}}}"#;
    let requirement = r#"{"code":0,"msg":"ok","data":{"job_requirement":{"id":"jr-1","short_code":"JR001","name":"Backend Engineer","head_count":2,"recruiter_list":[{"id":"ou_recruiter","name":{"zh_cn":"招聘负责人"}}]}}}"#;
    let tripartite = r#"{"code":0,"msg":"ok","data":{"id":"agreement-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, manager),
        http_response(200, requirement),
        http_response(200, tripartite),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    let manager = hire
        .job_manager
        .batch_update(
            "job-1",
            json!({"recruiter_id":"ou_recruiter"}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .job_manager
        .unwrap();
    let requirement = hire
        .job_requirement
        .create(
            json!({"name":"Backend Engineer"}),
            Some("open_id"),
            Some("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .job_requirement
        .unwrap();
    let agreement_id = hire
        .tripartite_agreement
        .create(
            json!({"application_id":"application-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .id;

    assert_eq!(manager.id.as_deref(), Some("manager-1"));
    assert_eq!(manager.recruiter_id.as_deref(), Some("ou_recruiter"));
    assert_eq!(
        manager.hiring_manager_id_list.as_ref().unwrap()[0],
        "ou_hiring"
    );
    assert_eq!(requirement.id.as_deref(), Some("jr-1"));
    assert_eq!(requirement.head_count, Some(2));
    assert_eq!(
        requirement.recruiter_list.as_ref().unwrap()[0]
            .name
            .as_ref()
            .unwrap()
            .get("zh_cn")
            .and_then(|value| value.as_str()),
        Some("招聘负责人")
    );
    assert_eq!(agreement_id.as_deref(), Some("agreement-1"));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/jobs/job-1/managers/batch_update "));
    assert!(request.contains("POST /open-apis/hire/v1/job_requirements?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("POST /open-apis/hire/v1/tripartite_agreements "));
    assert!(request.contains(r#""recruiter_id":"ou_recruiter""#));
    assert!(request.contains(r#""application_id":"application-1""#));
}

#[tokio::test]
async fn hire_referral_account_existing_model_write_responses() {
    let created = r#"{"code":0,"msg":"ok","data":{"account":{"account_id":"account-1","status":1,"assets":{"confirmed_bonus":{"bonus_type":1,"point_bonus":100}},"referrer":{"id":"ou_referrer","email":"referrer@example.test"}}}}"#;
    let deactivated =
        r#"{"code":0,"msg":"ok","data":{"account":{"account_id":"account-1","status":2}}}"#;
    let enabled =
        r#"{"code":0,"msg":"ok","data":{"account":{"account_id":"account-1","status":1}}}"#;
    let reconciliation = r#"{"code":0,"msg":"ok","data":{"check_failed_list":[{"account_id":"account-2","total_withdraw_reward_info":{"bonus_type":1,"point_bonus":50},"total_recharge_reward_info":{"bonus_type":1,"point_bonus":40}}]}}"#;
    let withdrawn = r#"{"code":0,"msg":"ok","data":{"external_order_id":"withdraw-1","trans_time":"1710000000000","withdrawal_details":{"bonus_type":1,"point_bonus":75,"cash":{"currency_type":"CNY","amount":12.5}}}}"#;
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

    let created = hire
        .referral_account
        .create(
            json!({"referrer_id":"ou_referrer"}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .account
        .unwrap();
    let deactivated = hire
        .referral_account
        .deactivate("account-1", &RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap()
        .account
        .unwrap();
    let enabled = hire
        .referral_account
        .enable(
            json!({"referral_account_id":"account-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .account
        .unwrap();
    let failed = hire
        .referral_account
        .reconciliation(
            json!({"account_id_list":["account-2"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .check_failed_list
        .into_iter()
        .next()
        .unwrap();
    let withdrawn = hire
        .referral_account
        .withdraw(
            "account-1",
            json!({"withdraw_bonus_type":[1],"external_order_id":"withdraw-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap();

    assert_eq!(created.account_id.as_deref(), Some("account-1"));
    assert_eq!(
        created
            .assets
            .as_ref()
            .unwrap()
            .confirmed_bonus
            .as_ref()
            .unwrap()
            .point_bonus,
        Some(100)
    );
    assert_eq!(deactivated.status, Some(2));
    assert_eq!(enabled.status, Some(1));
    assert_eq!(failed.account_id.as_deref(), Some("account-2"));
    assert_eq!(
        failed
            .total_withdraw_reward_info
            .as_ref()
            .unwrap()
            .point_bonus,
        Some(50)
    );
    assert_eq!(withdrawn.external_order_id.as_deref(), Some("withdraw-1"));
    assert_eq!(
        withdrawn
            .withdrawal_details
            .as_ref()
            .unwrap()
            .cash
            .as_ref()
            .unwrap()
            .amount,
        Some(12.5)
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/referral_account "));
    assert!(request.contains("POST /open-apis/hire/v1/referral_account/account-1/deactivate "));
    assert!(request.contains("POST /open-apis/hire/v1/referral_account/enable "));
    assert!(request.contains("POST /open-apis/hire/v1/referral_account/reconciliation "));
    assert!(request.contains("POST /open-apis/hire/v1/referral_account/account-1/withdraw "));
    assert!(request.contains(r#""referrer_id":"ou_referrer""#));
    assert!(request.contains(r#""external_order_id":"withdraw-1""#));
}
