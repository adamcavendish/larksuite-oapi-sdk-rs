mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::hire::v1::{
    ListJobFunctionQuery, ListRoleQuery, ListWebsiteQuery,
};

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn hire_catalog_lists_deserialize_typed_items() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"role-1","name":{"zh_cn":"管理员","en_us":"Admin"},"scope_of_application":1,"role_status":1}],"page_token":"next-1","has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListRoleQuery::new().page(PageQuery::new().page_size(20).page_token("seed-token"));
    let resp = client
        .hire()
        .role
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    let data = resp.data.unwrap();
    let role = &data.items[0];
    assert_eq!(role.id.as_deref(), Some("role-1"));
    assert_eq!(
        role.name.as_ref().and_then(|name| name.en_us.as_deref()),
        Some("Admin")
    );
    assert_eq!(role.scope_of_application, Some(1));
    assert_eq!(data.page_token.as_deref(), Some("next-1"));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/roles?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=seed-token"));
}

#[tokio::test]
async fn hire_role_get_deserializes_role_detail() {
    let body = r#"{"code":0,"msg":"ok","data":{"role":{"id":"role-1","name":{"zh_cn":"管理员"},"has_business_management_scope":true,"socail_permission_collection":{"enabled":true}}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .hire()
        .role
        .get("role-1", &RequestOption::default())
        .await
        .unwrap();

    let role = resp.data.unwrap().role.unwrap();
    assert_eq!(role.id.as_deref(), Some("role-1"));
    assert_eq!(role.has_business_management_scope, Some(true));
    assert_eq!(
        role.socail_permission_collection
            .as_ref()
            .and_then(|value| value.get("enabled"))
            .and_then(|value| value.as_bool()),
        Some(true)
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/roles/role-1 "));
}

#[tokio::test]
async fn hire_catalog_iterator_pages_and_limits() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"source-1"},{"id":"source-2"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"source-3"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let hire = client.hire();
    let mut iter = hire.resume_source.list_by_iterator(Some(2)).limit(2);

    let first = iter.next(&RequestOption::default()).await.unwrap().unwrap();
    let second = iter.next(&RequestOption::default()).await.unwrap().unwrap();
    let third = iter.next(&RequestOption::default()).await.unwrap();

    assert_eq!(first.id.as_deref(), Some("source-1"));
    assert_eq!(second.id.as_deref(), Some("source-2"));
    assert!(third.is_none());
    assert_eq!(iter.next_page_token(), Some("next-1"));

    let reqs = requests.lock().unwrap();
    assert_eq!(reqs.len(), 1);
    assert!(reqs[0].contains("GET /open-apis/hire/v1/resume_sources?"));
    assert!(reqs[0].contains("page_size=2"));
}

#[tokio::test]
async fn hire_catalog_iterator_sends_resume_token() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"func-1"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"func-2"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let query = ListJobFunctionQuery::new()
        .page_size(Some(1))
        .page_token(Some("seed-token"));
    let hire = client.hire();
    let mut iter = hire.job_function.list_iterator_by_query(&query);

    let _ = iter.next(&RequestOption::default()).await.unwrap();
    let _ = iter.next(&RequestOption::default()).await.unwrap();

    let reqs = requests.lock().unwrap();
    assert!(reqs[0].contains("GET /open-apis/hire/v1/job_functions?"));
    assert!(reqs[0].contains("page_token=seed-token"));
    assert!(reqs[1].contains("page_token=next-1"));
}

#[tokio::test]
async fn hire_catalog_iterator_limit_zero_is_unlimited() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"site-1"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"site-2"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let query = ListWebsiteQuery::new().page_size(Some(1));
    let hire = client.hire();
    let mut iter = hire.website.list_iterator_by_query(&query).limit(0);

    let first = iter.next(&RequestOption::default()).await.unwrap().unwrap();
    let second = iter.next(&RequestOption::default()).await.unwrap().unwrap();
    let third = iter.next(&RequestOption::default()).await.unwrap();

    assert_eq!(first.id.as_deref(), Some("site-1"));
    assert_eq!(second.id.as_deref(), Some("site-2"));
    assert!(third.is_none());
    assert_eq!(requests.lock().unwrap().len(), 2);
}
