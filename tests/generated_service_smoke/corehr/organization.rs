use super::prelude::*;

// CoreHR organization smoke tests

#[tokio::test]
async fn corehr_company_create_uses_typed_response() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"company":{"id":"company-1","name":[{"text":"Acme"}]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr()
        .company
        .create(Company::default(), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().company.unwrap().id.as_deref(),
        Some("company-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/corehr/v1/companies"));
}

#[tokio::test]
async fn corehr_job_level_create_uses_typed_response() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"job_level":{"id":"job-level-1","name":[{"text":"L1"}]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr()
        .job_level
        .create(JobLevel::default(), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().job_level.unwrap().id.as_deref(),
        Some("job-level-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/corehr/v1/job_levels"));
}

#[tokio::test]
async fn corehr_employee_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"employment":{"id":"emp-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetCorehrEmployeeQuery::new("emp-1").user_id_type("open_id");
    let resp = client
        .corehr()
        .employee
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().employment.unwrap().id.as_deref(),
        Some("emp-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/employments/emp-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn corehr_employee_get_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"employment":{"id":"emp-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr()
        .employee
        .get("emp-1", Some("open_id"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().employment.unwrap().id.as_deref(),
        Some("emp-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/employments/emp-1?user_id_type=open_id"));
}

#[tokio::test]
async fn corehr_employee_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"emp-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrEmployeeQuery::new()
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .employee
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("emp-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/employments?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_department_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"department":{"id":"dept-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetCorehrDepartmentQuery::new("dept-1").user_id_type("open_id");
    let resp = client
        .corehr()
        .department
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().department.unwrap().id.as_deref(),
        Some("dept-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/departments/dept-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn corehr_department_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"dept-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrDepartmentQuery::new()
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .department
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("dept-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/departments?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_company_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"company-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrCompanyQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .company
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("company-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/companies?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_location_get_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"location":{"id":"loc-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr()
        .location
        .get("loc-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().location.unwrap().id.as_deref(),
        Some("loc-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/locations/loc-1"));
}

#[tokio::test]
async fn corehr_location_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"loc-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrLocationQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .location
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("loc-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/locations?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_country_region_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"country-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrCountryRegionQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .country_region
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("country-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/country_regions?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_employee_type_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"type-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrEmployeeTypeQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .employee_type
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("type-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/employee_types?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_subdivision_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"subdivision-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSubdivisionQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .subdivision
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("subdivision-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/subdivisions?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_subregion_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"subregion-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSubregionQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .subregion
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("subregion-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/subregions?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_currency_get_uses_typed_response() {
    let body = r#"{"code":0,"msg":"ok","data":{"currency":{"id":"currency-1","country_region_id_list":["country-1"]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr()
        .currency
        .get("currency-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let currency = resp.data.unwrap().currency.unwrap();
    assert_eq!(currency.id.as_deref(), Some("currency-1"));
    assert_eq!(currency.country_region_id_list.unwrap(), ["country-1"]);
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/currencies/currency-1"));
}

#[tokio::test]
async fn corehr_transfer_reason_query_uses_typed_response() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"transfer_reason_unique_identifier":"transfer-1","name":[{"lang":"en-US","value":"Transfer"}]}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr()
        .transfer_reason
        .query(&RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let reason = resp.data.unwrap().items.into_iter().next().unwrap();
    assert_eq!(
        reason.transfer_reason_unique_identifier.as_deref(),
        Some("transfer-1")
    );
    assert_eq!(reason.name.unwrap()[0].value.as_deref(), Some("Transfer"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/transfer_reasons/query"));
}
