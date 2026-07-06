use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_referral_talent_query_smoke() {
    let referral_get = r#"{"code":0,"msg":"ok","data":{"referral":{"id":"ref-1"}}}"#;
    let referral_search = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"ref-info-1"}]}}"#;
    let talent_objects = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"object-1"}]}}"#;
    let operation_logs =
        r#"{"code":0,"msg":"ok","data":{"items":[{"talent_id":"talent-1"}],"has_more":false}}"#;
    let talent_pools =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"pool-1"}],"has_more":false}}"#;
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

    hire.referral
        .get_by_application_query(
            &GetHireReferralQuery::new()
                .application_id("app-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.referral
        .search_by_query(
            &SearchHireReferralQuery::new().user_id_type("open_id"),
            json!({"talent_id":"talent-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.talent_object
        .query(&RequestOption::default())
        .await
        .unwrap();
    hire.talent_operation_log
        .search_by_query(
            &SearchHireTalentOperationLogQuery::new()
                .page(page)
                .user_id_type("open_id"),
            json!({"operation_list":[3001]}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.talent_pool
        .search_by_query(
            &SearchHireTalentPoolQuery::new()
                .page(page)
                .id_list(Some(&pool_ids[..])),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/referrals/get_by_application?"));
    assert!(request.contains("application_id=app-1"));
    assert!(request.contains("POST /open-apis/hire/v1/referrals/search?"));
    assert!(request.contains("GET /open-apis/hire/v1/talent_objects/query "));
    assert!(request.contains("POST /open-apis/hire/v1/talent_operation_logs/search?"));
    assert!(request.contains("GET /open-apis/hire/v1/talent_pools?"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains("id_list=pool-1"));
    assert!(request.contains("id_list=pool-2"));
    assert!(request.contains(r#""operation_list":[3001]"#));
}
