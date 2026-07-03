mod common;

use common::{http_response, mock_server, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::contact::v3::{FindUserByDepartmentQuery, ListDepartmentQuery};

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn contact_department_iterator_pages_and_limits() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"d1"},{"department_id":"d2"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"d3"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle) =
        mock_server(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let mut iter = client
        .contact()
        .department
        .list_by_iterator(Some("open_id"), None, None, Some(false), Some(2))
        .limit(2);
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    };

    let first = iter.next(&option).await.unwrap().unwrap();
    let second = iter.next(&option).await.unwrap().unwrap();
    let third = iter.next(&option).await.unwrap();

    assert_eq!(first.department_id.as_deref(), Some("d1"));
    assert_eq!(second.department_id.as_deref(), Some("d2"));
    assert!(third.is_none());
    assert_eq!(iter.next_page_token(), Some("next-1"));
}

#[tokio::test]
async fn contact_user_iterator_sends_resume_token() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"u1"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"u2"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let mut iter = client
        .contact()
        .user
        .list_by_iterator(Some("user_id"), Some("department_id"), Some("d1"), Some(1))
        .page_token("seed-token");
    let option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..Default::default()
    };

    let _ = iter.next(&option).await.unwrap();
    let _ = iter.next(&option).await.unwrap();

    let reqs = requests.lock().unwrap();
    assert!(reqs[0].contains("GET /open-apis/contact/v3/users?"));
    assert!(reqs[0].contains("department_id=d1"));
    assert!(reqs[0].contains("page_token=seed-token"));
    assert!(reqs[1].contains("page_token=next-1"));
}

#[tokio::test]
async fn contact_children_iterator_preserves_token_on_empty_page() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"d1"}],"page_token":"next-1","has_more":true}}"#;
    let page2 =
        r#"{"code":0,"msg":"ok","data":{"items":[],"page_token":"next-2","has_more":true}}"#;
    let (addr, _handle) =
        mock_server(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let query = ListDepartmentQuery::new()
        .user_id_type(Some("open_id"))
        .page_size(Some(1));
    let mut iter = client.contact().department.list_iterator_by_query(&query);
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    };

    let first = iter.next(&option).await.unwrap().unwrap();
    let second = iter.next(&option).await.unwrap();

    assert_eq!(first.department_id.as_deref(), Some("d1"));
    assert!(second.is_none());
    assert_eq!(iter.next_page_token(), Some("next-1"));
}

#[tokio::test]
async fn contact_find_user_by_department_iterator_uses_query_shape() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"u1"}],"page_token":"next-1","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1)]).await;

    let client = client_for(addr);
    let query = FindUserByDepartmentQuery::new("d1")
        .user_id_type(Some("open_id"))
        .department_id_type(Some("department_id"))
        .page_size(Some(20))
        .page_token(Some("seed-token"));
    let mut iter = client
        .contact()
        .user
        .find_by_department_iterator_by_query(&query);
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    };

    let first = iter.next(&option).await.unwrap().unwrap();

    assert_eq!(first.user_id.as_deref(), Some("u1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/users/find_by_department?"));
    assert!(request.contains("department_id=d1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_token=seed-token"));
}

#[tokio::test]
async fn contact_unit_iterator_pages_unitlist() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"unitlist":[{"unit_id":"unit-1"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"unitlist":[{"unit_id":"unit-2"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle) =
        mock_server(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let mut iter = client.contact().unit.list_by_iterator(Some(1));
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    };

    let first = iter.next(&option).await.unwrap().unwrap();
    let second = iter.next(&option).await.unwrap().unwrap();
    let third = iter.next(&option).await.unwrap();

    assert_eq!(first.unit_id.as_deref(), Some("unit-1"));
    assert_eq!(second.unit_id.as_deref(), Some("unit-2"));
    assert!(third.is_none());
}
