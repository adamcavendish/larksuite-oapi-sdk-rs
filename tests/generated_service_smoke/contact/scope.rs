use super::prelude::*;

// Contact scope smoke tests

#[tokio::test]
async fn contact_scope_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"department_ids":["od-1"],"user_ids":["u-1"],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListScopeQuery::new()
        .user_id_type("open_id")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .scope
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.department_ids.as_ref().unwrap()[0], "od-1");
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/scopes?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
