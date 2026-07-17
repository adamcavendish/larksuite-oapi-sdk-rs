use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_utility_test_query_smoke() {
    let job_manager = r#"{"code":0,"msg":"ok","data":{"info":{"id":"job-1"}}}"#;
    let account_assets = r#"{"code":0,"msg":"ok","data":{"account":{"account_id":"acct-1"}}}"#;
    let talent_batch = r#"{"code":0,"msg":"ok","data":{"talent_list":[{"talent_id":"talent-1"}]}}"#;
    let tests = r#"{"code":0,"msg":"ok","data":{"items":[{"test_id":"test-1"}],"has_more":false}}"#;
    let delivery_task = r#"{"code":0,"msg":"ok","data":{"task_id":"task-1"}}"#;
    let delivery = r#"{"code":0,"msg":"ok","data":{"delivery":{"id":"delivery-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, job_manager),
        http_response(200, account_assets),
        http_response(200, talent_batch),
        http_response(200, tests),
        http_response(200, delivery_task),
        http_response(200, delivery),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();
    let page = PageQuery::new().page_size(20).page_token("seed-token");

    hire.job_manager
        .get(
            "job-1",
            "manager-1",
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.referral_account
        .get_account_assets(&RequestOption::default())
        .await
        .unwrap();
    hire.talent
        .batch_get_id(
            BatchGetIdTalentReqBody {
                mobile_number_list: Some(vec!["13800000000".into()]),
                ..Default::default()
            },
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.test
        .search_by_query(
            &SearchHireTestQuery::new()
                .page(page)
                .user_id_type("open_id"),
            json!({"application_id_list":["app-1"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.website_delivery
        .create_by_attachment(
            "website-1",
            json!({"attachment_id":"attachment-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.website_delivery
        .create_by_resume(
            "website-1",
            json!({"resume_id":"resume-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/jobs/job-1/managers/manager-1?"));
    assert!(request.contains("GET /open-apis/hire/v1/referral_account/get_account_assets "));
    assert!(request.contains("POST /open-apis/hire/v1/talents/batch_get_id "));
    assert!(request.contains("POST /open-apis/hire/v1/tests/search?"));
    assert!(
        request.contains(
            "POST /open-apis/hire/v1/websites/website-1/deliveries/create_by_attachment "
        )
    );
    assert!(
        request.contains("POST /open-apis/hire/v1/websites/website-1/deliveries/create_by_resume ")
    );
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains(r#""mobile_number_list":["13800000000"]"#));
    assert!(request.contains(r#""application_id_list":["app-1"]"#));
    assert!(request.contains(r#""attachment_id":"attachment-1""#));
    assert!(request.contains(r#""resume_id":"resume-1""#));
}
