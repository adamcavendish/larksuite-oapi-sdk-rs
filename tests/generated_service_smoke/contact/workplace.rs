use super::prelude::*;

// Contact workplace smoke tests

#[tokio::test]
async fn contact_work_city_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"work_city_id":"city-1","name":"Shanghai"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListWorkCityQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .work_city
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|city| city.work_city_id.as_deref()),
        Some("city-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/work_cities?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_custom_attr_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"attr-1","name":"Nickname"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCustomAttrQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .custom_attr
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|item| item.get("id"))
            .and_then(|id| id.as_str()),
        Some("attr-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/custom_attrs?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
