mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::hire::v1::{
    GetReferralWebsiteJobPostQuery, GetWebsiteJobPostQuery, ListPortalApplySchemaQuery,
    ListReferralWebsiteJobPostQuery, ListWebsiteChannelQuery, ListWebsiteJobPostQuery,
    SearchWebsiteJobPostQuery,
};
use serde_json::json;

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

fn tenant_option() -> RequestOption {
    RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    }
}

#[tokio::test]
async fn hire_website_posting_responses_deserialize_typed_items() {
    let website_get = r#"{"code":0,"msg":"ok","data":{"job_post":{"id":"post-1","title":"Rust Engineer","job_sequence_info":{"id":"seq-1","en_name":"Engineering"},"currency":1,"address":{"id":"addr-1","name":{"en_us":"Shanghai"}},"customized_data_list":[{"object_id":"custom-1","name":{"en_us":"Team"},"object_type":1,"value":{"content":"Platform"}}]}}}"#;
    let website_list = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"post-2","title":"Backend Engineer","target_major_list":[{"id":"major-1","en_name":"CS"}]}],"has_more":false,"page_token":"next-1"}}"#;
    let referral_get = r#"{"code":0,"msg":"ok","data":{"job_post":{"id":"ref-post-1","title":"Referral Engineer","address_list":[{"id":"addr-2","name":{"en_us":"Beijing"}}]}}}"#;
    let portal_schema = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"schema-1","name":"Portal schema"}],"has_more":false,"page_token":"next-2"}}"#;
    let channel_list = r#"{"code":0,"msg":"ok","data":{"website_channel_list":[{"id":"channel-1","name":"Official","link":"https://jobs.example","code":"A1"}],"has_more":false,"page_token":"next-3"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, website_get),
        http_response(200, website_list),
        http_response(200, referral_get),
        http_response(200, portal_schema),
        http_response(200, channel_list),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    let get_resp = hire
        .website_job_post
        .get_by_query(
            &GetWebsiteJobPostQuery::new("site-1", "post-1")
                .user_id_type("open_id")
                .department_id_type("open_department_id")
                .job_level_id_type("job_level_id"),
            &tenant_option(),
        )
        .await
        .unwrap();
    let list_resp = hire
        .website_job_post
        .list_by_query(
            &ListWebsiteJobPostQuery::new("site-1")
                .page(PageQuery::new().page_size(10).page_token("seed-token"))
                .update_start_time("1710000000"),
            &tenant_option(),
        )
        .await
        .unwrap();
    let referral_resp = hire
        .referral_website_job_post
        .get_by_query(
            &GetReferralWebsiteJobPostQuery::new("ref-post-1")
                .department_id_type("open_department_id"),
            &tenant_option(),
        )
        .await
        .unwrap();
    let schema_resp = hire
        .portal_apply_schema
        .list_by_query(
            &ListPortalApplySchemaQuery::new().page_size(20),
            &tenant_option(),
        )
        .await
        .unwrap();
    let channel_resp = hire
        .website_channel
        .list_by_query(
            &ListWebsiteChannelQuery::new("site-1").page_token("channel-token"),
            &tenant_option(),
        )
        .await
        .unwrap();

    let job_post = get_resp.data.unwrap().job_post.unwrap();
    assert_eq!(job_post.id.as_deref(), Some("post-1"));
    assert_eq!(
        job_post
            .address
            .as_ref()
            .and_then(|address| address.name.as_ref())
            .and_then(|name| name.en_us.as_deref()),
        Some("Shanghai")
    );
    assert_eq!(
        job_post
            .customized_data_list
            .as_ref()
            .and_then(|items| items.first())
            .and_then(|item| item.value.as_ref())
            .and_then(|value| value.content.as_deref()),
        Some("Platform")
    );
    assert_eq!(
        list_resp.data.unwrap().items[0]
            .target_major_list
            .as_ref()
            .and_then(|items| items.first())
            .and_then(|item| item.id.as_deref()),
        Some("major-1")
    );
    assert_eq!(
        referral_resp
            .data
            .unwrap()
            .job_post
            .unwrap()
            .title
            .as_deref(),
        Some("Referral Engineer")
    );
    assert_eq!(
        schema_resp.data.unwrap().items[0].id.as_deref(),
        Some("schema-1")
    );
    assert_eq!(
        channel_resp.data.unwrap().website_channel_list[0]
            .name
            .as_deref(),
        Some("Official")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/websites/site-1/job_posts/post-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("GET /open-apis/hire/v1/websites/site-1/job_posts?"));
    assert!(request.contains("page_size=10"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains("update_start_time=1710000000"));
    assert!(request.contains("GET /open-apis/hire/v1/referral_websites/job_posts/ref-post-1?"));
    assert!(request.contains("GET /open-apis/hire/v1/portal_apply_schemas?"));
    assert!(request.contains("GET /open-apis/hire/v1/websites/site-1/channels?"));
    assert!(request.contains("page_token=channel-token"));
}

#[tokio::test]
async fn hire_website_job_post_iterator_pages_and_filters() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"post-1"},{"id":"post-2"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"post-3"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let hire = client.hire();
    let mut iter = hire
        .website_job_post
        .list_by_iterator(
            "site-1",
            Some(2),
            Some("open_id"),
            Some("open_department_id"),
            Some("job_level_id"),
            Some("1710000000"),
            Some("1710009999"),
            Some("1700000000"),
            Some("1700009999"),
        )
        .limit(2);

    let first = iter.next(&tenant_option()).await.unwrap().unwrap();
    let second = iter.next(&tenant_option()).await.unwrap().unwrap();
    let third = iter.next(&tenant_option()).await.unwrap();

    assert_eq!(first.id.as_deref(), Some("post-1"));
    assert_eq!(second.id.as_deref(), Some("post-2"));
    assert!(third.is_none());
    assert_eq!(iter.next_page_token(), Some("next-1"));

    let reqs = requests.lock().unwrap();
    assert_eq!(reqs.len(), 1);
    assert!(reqs[0].contains("GET /open-apis/hire/v1/websites/site-1/job_posts?"));
    assert!(reqs[0].contains("page_size=2"));
    assert!(reqs[0].contains("user_id_type=open_id"));
    assert!(reqs[0].contains("department_id_type=open_department_id"));
    assert!(reqs[0].contains("job_level_id_type=job_level_id"));
    assert!(reqs[0].contains("update_start_time=1710000000"));
    assert!(reqs[0].contains("create_end_time=1700009999"));
}

#[tokio::test]
async fn hire_website_job_post_search_iterator_sends_body_and_resume_token() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"post-1","title":"Backend"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"post-2","title":"Frontend"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let body = json!({"keyword":"engineer","job_type_id_list":["type-1"]});
    let query = SearchWebsiteJobPostQuery::new("site-1")
        .page_size(1)
        .page_token("seed-token")
        .user_id_type("open_id");
    let hire = client.hire();
    let mut iter = hire.website_job_post.search_iterator_by_query(&query, body);

    let first = iter.next(&tenant_option()).await.unwrap().unwrap();
    let second = iter.next(&tenant_option()).await.unwrap().unwrap();
    let third = iter.next(&tenant_option()).await.unwrap();

    assert_eq!(first.title.as_deref(), Some("Backend"));
    assert_eq!(second.id.as_deref(), Some("post-2"));
    assert!(third.is_none());

    let reqs = requests.lock().unwrap();
    assert!(reqs[0].contains("POST /open-apis/hire/v1/websites/site-1/job_posts/search?"));
    assert!(reqs[0].contains("page_token=seed-token"));
    assert!(reqs[0].contains(r#""keyword":"engineer""#));
    assert!(reqs[1].contains("page_token=next-1"));
    assert!(reqs[1].contains(r#""job_type_id_list":["type-1"]"#));
}

#[tokio::test]
async fn hire_referral_and_portal_iterators_use_page_state() {
    let referral_page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"ref-1"}],"page_token":"ref-next","has_more":true}}"#;
    let referral_page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"ref-2"}],"page_token":"ref-done","has_more":false}}"#;
    let portal_page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"schema-1"}],"page_token":"portal-next","has_more":true}}"#;
    let portal_page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"schema-2"}],"page_token":"portal-done","has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, referral_page1),
        http_response(200, referral_page2),
        http_response(200, portal_page1),
        http_response(200, portal_page2),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();
    let referral_query = ListReferralWebsiteJobPostQuery::new()
        .page_size(1)
        .page_token("ref-seed")
        .process_type(1)
        .user_id_type("open_id");
    let mut referral_iter = hire
        .referral_website_job_post
        .list_iterator_by_query(&referral_query);

    let first_ref = referral_iter.next(&tenant_option()).await.unwrap().unwrap();
    let second_ref = referral_iter.next(&tenant_option()).await.unwrap().unwrap();
    assert_eq!(first_ref.id.as_deref(), Some("ref-1"));
    assert_eq!(second_ref.id.as_deref(), Some("ref-2"));

    let portal_query = ListPortalApplySchemaQuery::new()
        .page_size(Some(1))
        .page_token(Some("portal-seed"));
    let mut portal_iter = hire
        .portal_apply_schema
        .list_iterator_by_query(&portal_query)
        .limit(0);

    let first_schema = portal_iter.next(&tenant_option()).await.unwrap().unwrap();
    let second_schema = portal_iter.next(&tenant_option()).await.unwrap().unwrap();
    let third_schema = portal_iter.next(&tenant_option()).await.unwrap();

    assert_eq!(first_schema.id.as_deref(), Some("schema-1"));
    assert_eq!(second_schema.id.as_deref(), Some("schema-2"));
    assert!(third_schema.is_none());

    let reqs = requests.lock().unwrap();
    assert!(reqs[0].contains("GET /open-apis/hire/v1/referral_websites/job_posts?"));
    assert!(reqs[0].contains("page_token=ref-seed"));
    assert!(reqs[0].contains("process_type=1"));
    assert!(reqs[1].contains("page_token=ref-next"));
    assert!(reqs[2].contains("GET /open-apis/hire/v1/portal_apply_schemas?"));
    assert!(reqs[2].contains("page_token=portal-seed"));
    assert!(reqs[3].contains("page_token=portal-next"));
}
