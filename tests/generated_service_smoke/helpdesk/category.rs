use super::prelude::*;

// ── Helpdesk ──

#[tokio::test]
async fn helpdesk_category_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"category":{"id":"cat-1","name":"Billing"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .category
        .get_by_query(&GetCategoryQuery::new("cat-1"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.category.as_ref())
            .and_then(|category| category.name.as_deref()),
        Some("Billing")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/categories/cat-1"));
}

#[tokio::test]
async fn helpdesk_category_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"categories":[{"id":"cat-1","name":"Billing"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .category
        .list_by_query(
            &ListCategoryQuery::new().language("en-US"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/categories?"));
    assert!(request.contains("language=en-US"));
}
