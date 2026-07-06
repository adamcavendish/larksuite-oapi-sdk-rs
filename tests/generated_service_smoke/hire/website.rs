use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_website_posting_list_by_query_smoke() {
    let website_post_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"post-1","title":"Engineer"}],"page_token":"next-page","has_more":false}}"#;
    let referral_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"ref-post-1","title":"Referral"}],"page_token":"next-page","has_more":false}}"#;
    let portal_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"schema-1","name":"Portal"}],"page_token":"next-page","has_more":false}}"#;
    let channel_body = r#"{"code":0,"msg":"ok","data":{"website_channel_list":[{"id":"channel-1","name":"Official"}],"page_token":"next-page","has_more":false}}"#;
    let search_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"post-2","title":"Search"}],"page_token":"next-page","has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, website_post_body),
        http_response(200, referral_body),
        http_response(200, portal_body),
        http_response(200, channel_body),
        http_response(200, search_body),
    ])
    .await;

    let client = client_for(addr);
    let page = PageQuery::new().page_size(20).page_token("seed-token");

    client
        .hire()
        .website_job_post
        .list_by_query(
            &ListHireWebsiteJobPostQuery::new("site-1")
                .page(page)
                .user_id_type("open_id")
                .department_id_type("open_department_id")
                .job_level_id_type("job_level_id")
                .update_start_time("1710000000")
                .create_end_time("1710009999"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire()
        .referral_website_job_post
        .list_by_query(
            &ListHireReferralWebsiteJobPostQuery::new()
                .page(page)
                .process_type(1)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire()
        .portal_apply_schema
        .list_by_query(
            &ListHirePortalApplySchemaQuery::new().page(page),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire()
        .website_channel
        .list_by_query(
            &ListHireWebsiteChannelQuery::new("site-1").page(page),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire()
        .website_job_post
        .search_by_query(
            &SearchHireWebsiteJobPostQuery::new("site-1")
                .page(page)
                .user_id_type("open_id"),
            json!({"keyword":"engineer"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/websites/site-1/job_posts?"));
    assert!(request.contains("GET /open-apis/hire/v1/referral_websites/job_posts?"));
    assert!(request.contains("GET /open-apis/hire/v1/portal_apply_schemas?"));
    assert!(request.contains("GET /open-apis/hire/v1/websites/site-1/channels?"));
    assert!(request.contains("POST /open-apis/hire/v1/websites/site-1/job_posts/search?"));
    assert_eq!(request.matches("page_size=20").count(), 5);
    assert_eq!(request.matches("page_token=seed-token").count(), 5);
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("process_type=1"));
    assert!(request.contains(r#""keyword":"engineer""#));
}
