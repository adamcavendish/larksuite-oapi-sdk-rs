mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::hire::v1::{
    GetReferralQuery, SearchReferralQuery, SearchTalentOperationLogQuery, SearchTalentPoolQuery,
};
use serde_json::json;

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn hire_referral_talent_responses_deserialize_typed_items() {
    let referral_get = r#"{"code":0,"msg":"ok","data":{"referral":{"id":"ref-1","application_id":"app-1","create_time":1710000000,"referral_user":{"id":"ou_1","name":{"en_us":"Ada"}}}}}"#;
    let referral_search = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"ref-info-1","application_ids":["app-1","app-2"],"referral_user":{"id":"ou_2","name":{"zh_cn":"小林"}}}]}}"#;
    let talent_objects = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"object-1","name":{"en_us":"Portfolio"},"is_customized":true}]}}"#;
    let operation_logs = r#"{"code":0,"msg":"ok","data":{"items":[{"application_id":"app-1","talent_id":"talent-1","operator":{"id":"ou_3","name":{"en_us":"Recruiter"}},"operation_type":3001,"operation_time":"1710000000","operator_type":1}],"page_token":"op-next","has_more":false}}"#;
    let talent_pools = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"pool-1","i18n_name":{"en_us":"Backend Pool"},"i18n_description":{"zh_cn":"后端"},"is_private":1}],"page_token":"pool-next","has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, referral_get),
        http_response(200, referral_search),
        http_response(200, talent_objects),
        http_response(200, operation_logs),
        http_response(200, talent_pools),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();
    let page = PageQuery::new().page_size(20).page_token("seed-token");
    let pool_ids = ["pool-1", "pool-2"];

    let referral = hire
        .referral
        .get_by_application_query(
            &GetReferralQuery::new()
                .application_id("app-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .referral
        .unwrap();
    let referral_info = hire
        .referral
        .search_by_query(
            &SearchReferralQuery::new().user_id_type("open_id"),
            json!({"talent_id":"talent-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let talent_object = hire
        .talent_object
        .query(&RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let operation_log = hire
        .talent_operation_log
        .search_by_query(
            &SearchTalentOperationLogQuery::new()
                .page(page)
                .user_id_type("open_id"),
            json!({"job_id_list":["job-1"],"operation_list":[3001]}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let talent_pool = hire
        .talent_pool
        .search_by_query(
            &SearchTalentPoolQuery::new()
                .page(page)
                .id_list(Some(&pool_ids[..])),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);

    assert_eq!(
        referral
            .referral_user
            .as_ref()
            .and_then(|user| user.name.as_ref())
            .and_then(|name| name.en_us.as_deref()),
        Some("Ada")
    );
    assert_eq!(
        referral_info.application_ids.as_ref().unwrap()[1].as_str(),
        "app-2"
    );
    assert_eq!(talent_object.is_customized, Some(true));
    assert_eq!(
        operation_log
            .operator
            .as_ref()
            .and_then(|operator| operator.name.as_ref())
            .and_then(|name| name.en_us.as_deref()),
        Some("Recruiter")
    );
    assert_eq!(
        talent_pool
            .i18n_name
            .as_ref()
            .and_then(|name| name.en_us.as_deref()),
        Some("Backend Pool")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/referrals/get_by_application?"));
    assert!(request.contains("application_id=app-1"));
    assert!(request.contains("POST /open-apis/hire/v1/referrals/search?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""talent_id":"talent-1""#));
    assert!(request.contains("GET /open-apis/hire/v1/talent_objects/query "));
    assert!(request.contains("POST /open-apis/hire/v1/talent_operation_logs/search?"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains(r#""operation_list":[3001]"#));
    assert!(request.contains("GET /open-apis/hire/v1/talent_pools?"));
    assert!(request.contains("id_list=pool-1"));
    assert!(request.contains("id_list=pool-2"));
}

#[tokio::test]
async fn hire_talent_pool_iterator_pages_and_limits() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"pool-1"},{"id":"pool-2"}],"page_token":"pool-next","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"pool-3"}],"page_token":"pool-done","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let pool_ids = ["pool-1", "pool-2"];
    let query = SearchTalentPoolQuery::new()
        .page_size(2)
        .page_token("seed-token")
        .id_list(Some(&pool_ids[..]));
    let hire = client.hire();
    let mut iter = hire.talent_pool.search_iterator_by_query(&query).limit(2);

    let first = iter.next(&RequestOption::default()).await.unwrap().unwrap();
    let second = iter.next(&RequestOption::default()).await.unwrap().unwrap();
    let third = iter.next(&RequestOption::default()).await.unwrap();

    assert_eq!(first.id.as_deref(), Some("pool-1"));
    assert_eq!(second.id.as_deref(), Some("pool-2"));
    assert!(third.is_none());
    assert_eq!(iter.next_page_token(), Some("pool-next"));

    let reqs = requests.lock().unwrap();
    assert_eq!(reqs.len(), 1);
    assert!(reqs[0].contains("GET /open-apis/hire/v1/talent_pools?"));
    assert!(reqs[0].contains("page_size=2"));
    assert!(reqs[0].contains("page_token=seed-token"));
    assert!(reqs[0].contains("id_list=pool-1"));
    assert!(reqs[0].contains("id_list=pool-2"));
}

#[tokio::test]
async fn hire_talent_pool_iterator_limit_zero_is_unlimited() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"pool-1"}],"page_token":"pool-next","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"pool-2"}],"page_token":"pool-done","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let query = SearchTalentPoolQuery::new().page_size(1);
    let hire = client.hire();
    let mut iter = hire.talent_pool.search_iterator_by_query(&query).limit(0);

    let first = iter.next(&RequestOption::default()).await.unwrap().unwrap();
    let second = iter.next(&RequestOption::default()).await.unwrap().unwrap();
    let third = iter.next(&RequestOption::default()).await.unwrap();

    assert_eq!(first.id.as_deref(), Some("pool-1"));
    assert_eq!(second.id.as_deref(), Some("pool-2"));
    assert!(third.is_none());
    assert_eq!(requests.lock().unwrap().len(), 2);
}
