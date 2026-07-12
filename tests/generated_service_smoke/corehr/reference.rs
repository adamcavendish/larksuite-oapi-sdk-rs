use super::prelude::*;

// CoreHR reference smoke tests

#[tokio::test]
async fn corehr_common_data_metadata_uses_typed_response() {
    let body = r#"{"code":0,"msg":"ok","data":{"enum_field_api_name":"grade","enum_field_options":[{"option_api_name":"grade_a","active":true,"name":{"zh_cn":"A","en_us":"A"}}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr()
        .common_data_meta_data
        .add_enum_option(
            serde_json::json!({"object_api_name": "employee"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.enum_field_api_name.as_deref(), Some("grade"));
    assert_eq!(
        data.enum_field_options[0]
            .name
            .as_ref()
            .and_then(|name| name.zh_cn.as_deref()),
        Some("A")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/corehr/v1/common_data/meta_data/add_enum_option"));
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
