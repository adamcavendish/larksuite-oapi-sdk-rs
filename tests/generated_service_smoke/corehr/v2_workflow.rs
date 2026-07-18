use super::prelude::*;

// CoreHR v2 workflow smoke tests

#[tokio::test]
async fn corehr_v2_company_active_uses_code_only_response() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr_v2()
        .company
        .active(
            json_value!({"company_id": "company-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(resp.data, None);
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/corehr/v2/companies/active"));
}

#[tokio::test]
async fn corehr_v2_company_recent_change_uses_typed_response() {
    let body = r#"{"code":0,"msg":"ok","data":{"company_ids":["company-1"],"deleted_company_ids":["company-0"],"page_token":"next","has_more":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr_v2()
        .company
        .query_recent_change(&RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.company_ids, ["company-1"]);
    assert_eq!(data.deleted_company_ids, ["company-0"]);
    assert_eq!(data.page_token.as_deref(), Some("next"));
    assert_eq!(data.has_more, Some(true));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/companies/query_recent_change"));
}

#[tokio::test]
async fn corehr_v2_department_recent_change_uses_typed_response() {
    let body = r#"{"code":0,"msg":"ok","data":{"department_ids":["department-1"],"deleted_department_ids":["department-0"],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .corehr_v2()
        .department
        .query_recent_change(&RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.department_ids, ["department-1"]);
    assert_eq!(data.deleted_department_ids, ["department-0"]);
    assert_eq!(data.has_more, Some(false));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/departments/query_recent_change"));
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
