use super::prelude::*;

// Contact employee type smoke tests

#[tokio::test]
async fn contact_employee_type_enum_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"enum_id":"enum-1","enum_value":"full_time"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListEmployeeTypeEnumQuery::new()
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .contact()
        .employee_type_enum
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.items.as_ref().unwrap()[0].enum_id.as_deref(),
        Some("enum-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/employee_type_enums?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
