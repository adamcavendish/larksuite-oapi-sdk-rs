use super::prelude::*;

// ── CoreHR ──

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
async fn corehr_job_level_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"job-level-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrJobLevelQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .job_level
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("job-level-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/job_levels?"));
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
async fn corehr_currency_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"currency_id":"CNY"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrCurrencyQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .currency
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].currency_id.as_deref(), Some("CNY"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/currencies?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_working_hours_type_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"hours-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrWorkingHoursTypeQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .working_hours_type
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("hours-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/working_hours_types?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_contract_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"contract-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrContractQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .contract
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("contract-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/contracts?"));
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
    assert_eq!(data["items"][0]["id"].as_str(), Some("country-1"));
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
    assert_eq!(data["items"][0]["id"].as_str(), Some("type-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/employee_types?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_job_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"job-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrJobQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .job
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("job-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/jobs?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_job_data_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"job-data-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrJobDataQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .job_data
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("job-data-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/job_datas?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_job_family_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"job-family-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrJobFamilyQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .job_family
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("job-family-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/job_families?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_national_id_type_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"nid-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrNationalIdTypeQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .national_id_type
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("nid-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/national_id_types?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_offboarding_search_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"offboarding_id":"off-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SearchCorehrOffboardingQuery::new()
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let body = serde_json::json!({"employment_id": "emp-1"});
    let resp = client
        .corehr()
        .offboarding
        .search_by_query(&query, body, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.items.unwrap()[0].offboarding_id.as_deref(),
        Some("off-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/corehr/v1/offboardings/search?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""employment_id":"emp-1""#));
}

#[tokio::test]
async fn corehr_pre_hire_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"pre-hire-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrPreHireQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .pre_hire
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("pre-hire-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/pre_hires?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_security_group_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"security-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSecurityGroupQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr()
        .security_group
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("security-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/security_groups?"));
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
    assert_eq!(data["items"][0]["id"].as_str(), Some("subdivision-1"));
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
    assert_eq!(data["items"][0]["id"].as_str(), Some("subregion-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/subregions?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_transfer_reason_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"reason-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr()
        .transfer_reason
        .query(&RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("reason-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/transfer_reasons/query"));
}

#[tokio::test]
async fn corehr_transfer_type_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"type-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr()
        .transfer_type
        .query(&RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data["items"][0]["id"].as_str(), Some("type-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v1/transfer_types/query"));
}

#[tokio::test]
async fn corehr_v2_approver_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"approver-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrApproverV2Query::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .approver
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("approver-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/approvers?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_bp_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"bp-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrBpV2Query::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .bp
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("bp-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/bps?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_job_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"job-v2-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCorehrJobV2Query::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .job
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("job-v2-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/jobs?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_process_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"process-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrProcessV2Query::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .process
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("process-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/processes?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_signature_file_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"signature-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSignatureFileV2Query::new()
        .signature_file_id("signature-1")
        .states("sign_finished")
        .update_time_start("2026-01-01")
        .update_time_end("2026-12-31")
        .user_id_type("people_corehr_id")
        .template_ids("template-1")
        .select_sign_url(true)
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .signature_file
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("signature-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_files?"));
    assert!(request.contains("signature_file_id=signature-1"));
    assert!(request.contains("states=sign_finished"));
    assert!(request.contains("update_time_start=2026-01-01"));
    assert!(request.contains("update_time_end=2026-12-31"));
    assert!(request.contains("user_id_type=people_corehr_id"));
    assert!(request.contains("template_ids=template-1"));
    assert!(request.contains("select_sign_url=true"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_signature_file_list_by_biz_id_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"signature-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSignatureFileByBizIdV2Query::new()
        .biz_process_id("biz-process-1")
        .biz_type("OpenAPI")
        .user_id_type("people_corehr_id")
        .select_sign_url(true);
    let resp = client
        .corehr_v2()
        .signature_file
        .list_by_biz_id_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("signature-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_files/list_by_biz_id?"));
    assert!(request.contains("biz_process_id=biz-process-1"));
    assert!(request.contains("biz_type=OpenAPI"));
    assert!(request.contains("user_id_type=people_corehr_id"));
    assert!(request.contains("select_sign_url=true"));
}

#[tokio::test]
async fn corehr_v2_signature_node_list_by_file_id_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"node-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSignatureNodeByFileIdV2Query::new()
        .file_id("file-1")
        .user_id_type("people_corehr_id");
    let resp = client
        .corehr_v2()
        .signature_node
        .list_by_file_id_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("node-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_nodes/list_by_file_id?"));
    assert!(request.contains("file_id=file-1"));
    assert!(request.contains("user_id_type=people_corehr_id"));
}

#[tokio::test]
async fn corehr_v2_signature_template_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"template-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SearchCorehrSignatureTemplateV2Query::new()
        .template_ids("template-1")
        .select_custom_field(true);
    let resp = client
        .corehr_v2()
        .signature_template
        .search_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("template-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_templates/search?"));
    assert!(request.contains("template_ids=template-1"));
    assert!(request.contains("select_custom_field=true"));
}

#[tokio::test]
async fn corehr_v2_signature_template_info_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"template-info-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSignatureTemplateInfoWithThumbnailV2Query::new()
        .name("Onboarding")
        .category_apiname("contract_agreement")
        .usage_apiname("general")
        .active(false)
        .need_region_info(true)
        .user_id_type("people_corehr_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .signature_template_info_with_thumbnail
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("template-info-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_template_info_with_thumbnails?"));
    assert!(request.contains("name=Onboarding"));
    assert!(request.contains("category_apiname=contract_agreement"));
    assert!(request.contains("usage_apiname=general"));
    assert!(request.contains("active=false"));
    assert!(request.contains("need_region_info=true"));
    assert!(request.contains("user_id_type=people_corehr_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_workforce_plan_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"workforce-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrWorkforcePlanV2Query::new()
        .limit(10)
        .offset(1)
        .get_all_plan(true)
        .active(true)
        .start_date("2026-01-01")
        .end_date("2026-12-31")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .workforce_plan
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("workforce-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/workforce_plans?"));
    assert!(request.contains("limit=10"));
    assert!(request.contains("offset=1"));
    assert!(request.contains("get_all_plan=true"));
    assert!(request.contains("active=true"));
    assert!(request.contains("start_date=2026-01-01"));
    assert!(request.contains("end_date=2026-12-31"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
