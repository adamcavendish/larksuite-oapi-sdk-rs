mod common;

use common::{http_response, mock_server, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::bitable::v1::{ListFieldQuery, SearchRecordReqBody};

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn bitable_table_iterator_pages_and_limits() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"table_id":"tbl1"},{"table_id":"tbl2"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"table_id":"tbl3"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle) =
        mock_server(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let mut iter = client
        .bitable()
        .table
        .list_by_iterator("app-token", Some(2))
        .limit(2);
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    };

    let first = Box::pin(iter.next(&option)).await.unwrap().unwrap();
    let second = Box::pin(iter.next(&option)).await.unwrap().unwrap();
    let third = Box::pin(iter.next(&option)).await.unwrap();

    assert_eq!(first.table_id.as_deref(), Some("tbl1"));
    assert_eq!(second.table_id.as_deref(), Some("tbl2"));
    assert!(third.is_none());
    assert_eq!(iter.next_page_token(), Some("next-1"));
}

#[tokio::test]
async fn bitable_view_iterator_sends_resume_token() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"view_id":"vew1"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"view_id":"vew2"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let mut iter = client
        .bitable()
        .view
        .list_by_iterator("app-token", "tbl1", Some(1), Some("open_id"))
        .page_token("seed-token");
    let option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..Default::default()
    };

    let _ = Box::pin(iter.next(&option)).await.unwrap();
    let _ = Box::pin(iter.next(&option)).await.unwrap();

    let reqs = requests.lock().unwrap();
    assert!(reqs[0].contains("GET /open-apis/bitable/v1/apps/app-token/tables/tbl1/views?"));
    assert!(reqs[0].contains("user_id_type=open_id"));
    assert!(reqs[0].contains("page_token=seed-token"));
    assert!(reqs[1].contains("page_token=next-1"));
}

#[tokio::test]
async fn bitable_field_iterator_uses_query_shape() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"field_id":"fld1"}],"page_token":"next-1","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1)]).await;

    let client = client_for(addr);
    let query = ListFieldQuery::new("app-token", "tbl1")
        .view_id(Some("vew1"))
        .text_field_as_array(Some(true))
        .user_id_type(Some("open_id"))
        .page_size(Some(20))
        .page_token(Some("seed-token"));
    let mut iter = client.bitable().field.list_iterator_by_query(&query);
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    };

    let first = Box::pin(iter.next(&option)).await.unwrap().unwrap();

    assert_eq!(first.field_id.as_deref(), Some("fld1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token/tables/tbl1/fields?"));
    assert!(request.contains("view_id=vew1"));
    assert!(request.contains("text_field_as_array=true"));
    assert!(request.contains("page_token=seed-token"));
}

#[tokio::test]
async fn bitable_record_search_iterator_posts_each_page() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"record_id":"rec1"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"record_id":"rec2"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let body = SearchRecordReqBody {
        view_id: Some("vew1".to_string()),
        field_names: Some(vec!["Name".to_string()]),
        ..Default::default()
    };
    let mut iter = client.bitable().record.search_by_iterator(
        "app-token",
        "tbl1",
        &body,
        Some("open_id"),
        Some(1),
    );
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    };

    let first = Box::pin(iter.next(&option)).await.unwrap().unwrap();
    let second = Box::pin(iter.next(&option)).await.unwrap().unwrap();
    let third = Box::pin(iter.next(&option)).await.unwrap();

    assert_eq!(first.record_id.as_deref(), Some("rec1"));
    assert_eq!(second.record_id.as_deref(), Some("rec2"));
    assert!(third.is_none());

    let reqs = requests.lock().unwrap();
    assert!(
        reqs[0].contains("POST /open-apis/bitable/v1/apps/app-token/tables/tbl1/records/search?")
    );
    assert!(reqs[0].contains("\"view_id\":\"vew1\""));
    assert!(reqs[1].contains("page_token=next-1"));
    assert!(reqs[1].contains("\"field_names\":[\"Name\"]"));
}
