use super::prelude::*;

// Contact unit smoke tests

#[tokio::test]
async fn contact_unit_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"unitlist":[{"unit_id":"unit-1","name":"Engineering Unit"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListUnitQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .unit
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.unitlist.as_ref())
            .and_then(|items| items.first())
            .and_then(|unit| unit.name.as_deref()),
        Some("Engineering Unit")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/unit?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_unit_list_department_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"departmentlist":[{"unit_id":"unit-1","department_id":"od-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListDepartmentUnitQuery::new("unit-1")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .unit
        .list_department_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.departmentlist.as_ref())
            .and_then(|items| items.first())
            .and_then(|department| department.department_id.as_deref()),
        Some("od-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/unit/list_department?"));
    assert!(request.contains("unit_id=unit-1"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
