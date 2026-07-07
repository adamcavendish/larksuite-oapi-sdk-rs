mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::hire::v1::SearchTestQuery;
use serde_json::json;

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn hire_utility_responses_deserialize_and_send_filters() {
    let job_manager = r#"{"code":0,"msg":"ok","data":{"info":{"id":"job-1","recruiter_id":"ou_recruiter","hiring_manager_id_list":["ou_hm"],"assistant_id_list":["ou_assistant"]}}}"#;
    let account_assets = r#"{"code":0,"msg":"ok","data":{"account":{"account_id":"acct-1","status":1,"assets":{"confirmed_bonus":{"bonus_type":1,"point_bonus":100,"cash":{"currency_type":"CNY","amount":12.5},"cash_bonus":[{"currency_type":"USD","amount":3.5}]},"paid_bonus":{"bonus_type":2}},"referrer":{"id":"ou_ref","name":{"en_us":"Referrer"},"email":"ref@example.com","mobile":"123"}}}}"#;
    let talent_batch = r#"{"code":0,"msg":"ok","data":{"talent_list":[{"talent_id":"talent-1","mobile_code":"86","mobile_number":"13800000000","email":"talent@example.com","identification_type":1,"identification_number":"id-1","is_onboarded":true}]}}"#;
    let tests = r#"{"code":0,"msg":"ok","data":{"items":[{"test_id":"test-1","application_id":"app-1","talent_id":"talent-1","job_id":"job-1","test_paper_name":"Backend Exam","test_paper_source_name":{"zh_cn":"平台"},"reply_status":1,"test_status":2,"test_schedule":{"start_time":"1710000000000","end_time":"1710003600000"},"report_url_list":["https://example.test/report"],"result_detail_list":[{"subject":"Rust","result":"Pass"}],"score":"95","reviewer":"ou_reviewer"}],"has_more":false,"page_token":"test-next"}}"#;
    let delivery_task = r#"{"code":0,"msg":"ok","data":{"task_id":"task-1"}}"#;
    let delivery = r#"{"code":0,"msg":"ok","data":{"delivery":{"application_id":"app-2","id":"delivery-1","job_id":"job-2","job_post_id":"post-1","portal_resume_id":"resume-1","user_id":"user-1","talent_id":"talent-2"}}}"#;
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

    let manager = hire
        .job_manager
        .get(
            "job-1",
            "manager-1",
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .info
        .unwrap();
    let account = hire
        .referral_account
        .get_account_assets(&RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap()
        .account
        .unwrap();
    let talent = hire
        .talent
        .batch_get_id(
            json!({"mobile_list":["13800000000"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .talent_list
        .remove(0);
    let test = hire
        .test
        .search_by_query(
            &SearchTestQuery::new().page(page).user_id_type("open_id"),
            json!({"application_id_list":["app-1"],"test_start_time_min":"1710000000000"}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let delivery_task = hire
        .website_delivery
        .create_by_attachment(
            "website-1",
            json!({"attachment_id":"attachment-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap();
    let delivery = hire
        .website_delivery
        .create_by_resume(
            "website-1",
            json!({"resume_id":"resume-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap();

    assert_eq!(manager.recruiter_id.as_deref(), Some("ou_recruiter"));
    assert_eq!(manager.hiring_manager_id_list.as_ref().unwrap()[0], "ou_hm");
    assert_eq!(
        account
            .assets
            .as_ref()
            .and_then(|assets| assets.confirmed_bonus.as_ref())
            .and_then(|bonus| bonus.cash.as_ref())
            .and_then(|cash| cash.currency_type.as_deref()),
        Some("CNY")
    );
    assert_eq!(
        account
            .referrer
            .as_ref()
            .and_then(|referrer| referrer.name.as_ref())
            .and_then(|name| name.en_us.as_deref()),
        Some("Referrer")
    );
    assert_eq!(talent.is_onboarded, Some(true));
    assert_eq!(
        test.test_schedule
            .as_ref()
            .and_then(|schedule| schedule.start_time.as_deref()),
        Some("1710000000000")
    );
    assert_eq!(
        test.result_detail_list.as_ref().unwrap()[0]
            .result
            .as_deref(),
        Some("Pass")
    );
    assert_eq!(delivery_task.task_id.as_deref(), Some("task-1"));
    assert_eq!(
        delivery
            .delivery
            .as_ref()
            .and_then(|delivery| delivery.job_post_id.as_deref()),
        Some("post-1")
    );

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
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains(r#""mobile_list":["13800000000"]"#));
    assert!(request.contains(r#""application_id_list":["app-1"]"#));
    assert!(request.contains(r#""attachment_id":"attachment-1""#));
    assert!(request.contains(r#""resume_id":"resume-1""#));
}

#[tokio::test]
async fn hire_test_iterator_pages_and_uses_server_cursor() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"test_id":"test-1"},{"test_id":"test-2"}],"page_token":"test-next","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"test_id":"test-3"}],"page_token":"test-done","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let hire = client.hire();
    let query = SearchTestQuery::new()
        .page_size(2)
        .page_token("seed-token")
        .user_id_type("open_id");
    let mut iter = hire
        .test
        .search_iterator_by_query(&query, json!({"application_id_list":["app-1"]}))
        .limit(3);

    let first = iter.next(&RequestOption::default()).await.unwrap().unwrap();
    let second = iter.next(&RequestOption::default()).await.unwrap().unwrap();
    let third = iter.next(&RequestOption::default()).await.unwrap().unwrap();
    let done = iter.next(&RequestOption::default()).await.unwrap();

    assert_eq!(first.test_id.as_deref(), Some("test-1"));
    assert_eq!(second.test_id.as_deref(), Some("test-2"));
    assert_eq!(third.test_id.as_deref(), Some("test-3"));
    assert!(done.is_none());
    assert_eq!(iter.next_page_token(), Some("test-done"));

    let reqs = requests.lock().unwrap();
    assert_eq!(reqs.len(), 2);
    assert!(reqs[0].contains("page_token=seed-token"));
    assert!(reqs[1].contains("page_token=test-next"));
    assert!(reqs[0].contains("user_id_type=open_id"));
    assert!(reqs[1].contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_test_iterator_limit_zero_is_unlimited() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"test_id":"test-1"}],"page_token":"test-next","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"test_id":"test-2"}],"has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let hire = client.hire();
    let mut iter = hire
        .test
        .search_by_iterator(Some(1), json!({"application_id_list":["app-1"]}))
        .limit(0);

    let first = iter.next(&RequestOption::default()).await.unwrap().unwrap();
    let second = iter.next(&RequestOption::default()).await.unwrap().unwrap();
    let done = iter.next(&RequestOption::default()).await.unwrap();

    assert_eq!(first.test_id.as_deref(), Some("test-1"));
    assert_eq!(second.test_id.as_deref(), Some("test-2"));
    assert!(done.is_none());
    assert_eq!(requests.lock().unwrap().len(), 2);
}
