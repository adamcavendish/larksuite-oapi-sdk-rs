use super::prelude::*;

// ── Security and Compliance ──

#[tokio::test]
async fn security_openapi_log_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"log-1","api_path":"/open-apis/im/v1/messages"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .security_and_compliance()
        .openapi_log
        .list(
            Some("/open-apis/im/v1/messages"),
            Some("1700000000"),
            Some("1700000100"),
            Some(1),
            Some("ou-1"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/security_and_compliance/v1/openapi_logs?"));
    assert!(request.contains("api_path=%2Fopen-apis%2Fim%2Fv1%2Fmessages"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("operator_type=1"));
    assert!(request.contains("operator_value=ou-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn security_openapi_log_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"log-1","api_path":"/open-apis/im/v1/messages"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListOpenapiLogQuery::new()
        .api_path("/open-apis/im/v1/messages")
        .start_time("1700000000")
        .end_time("1700000100")
        .operator_type(1)
        .operator_value("ou-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .security_and_compliance()
        .openapi_log
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].id.as_deref(), Some("log-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/security_and_compliance/v1/openapi_logs?"));
    assert!(request.contains("api_path=%2Fopen-apis%2Fim%2Fv1%2Fmessages"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700000100"));
    assert!(request.contains("operator_type=1"));
    assert!(request.contains("operator_value=ou-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn security_device_record_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"device_record_id":"device-1","device_name":"Laptop"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .security_and_compliance_v2()
        .device_record
        .list(Some(20), Some("next-page"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/security_and_compliance/v2/device_records?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn security_device_record_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"device_record_id":"device-1","device_name":"Laptop"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListDeviceRecordV2Query::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .security_and_compliance_v2()
        .device_record
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].device_record_id.as_deref(), Some("device-1"));
    assert_eq!(data.items[0].device_name.as_deref(), Some("Laptop"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/security_and_compliance/v2/device_records?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn security_device_records_write_by_query_smoke() {
    let create_body_resp = r#"{"code":0,"msg":"ok","data":{"device_record_id":"device-1"}}"#;
    let record_body =
        r#"{"code":0,"msg":"ok","data":{"device_record":{"device_record_id":"device-1"}}}"#;
    let mine_body = r#"{"code":0,"msg":"ok","data":{"device_record_id":"device-1","device_ownership":1,"device_status":2}}"#;
    let ok_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, create_body_resp),
        http_response(200, record_body),
        http_response(200, mine_body),
        http_response(200, ok_body),
        http_response(200, ok_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"device_id":"device-1","status":"active"});
    let update_body = serde_json::json!({"device_id":"device-1","status":"disabled"});
    let apply_update_body = serde_json::json!({"status":"approved"});

    let create_resp = client
        .security_and_compliance_v2()
        .device_record
        .create_by_query(
            &CreateDeviceRecordV2Query::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let get_resp = client
        .security_and_compliance_v2()
        .device_record
        .get_by_query(
            &GetDeviceRecordV2Query::new("device-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let mine_resp = client
        .security_and_compliance_v2()
        .device_record
        .mine_by_query(&MineDeviceRecordV2Query::new(), &RequestOption::default())
        .await
        .unwrap();
    client
        .security_and_compliance_v2()
        .device_record
        .update_by_query(
            &UpdateDeviceRecordV2Query::new("device-1", &update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .security_and_compliance_v2()
        .device_record
        .delete_by_query(
            &DeleteDeviceRecordV2Query::new("device-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .security_and_compliance_v2()
        .device_apply_record
        .update_by_query(
            &UpdateDeviceApplyRecordV2Query::new("apply-1", &apply_update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(
        create_resp.data.unwrap().device_record_id.as_deref(),
        Some("device-1")
    );
    assert_eq!(
        get_resp
            .data
            .unwrap()
            .device_record
            .unwrap()
            .device_record_id
            .as_deref(),
        Some("device-1")
    );
    assert_eq!(mine_resp.data.unwrap().device_status, Some(2));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/security_and_compliance/v2/device_records "));
    assert!(request.contains("GET /open-apis/security_and_compliance/v2/device_records/device-1 "));
    assert!(request.contains("GET /open-apis/security_and_compliance/v2/device_records/mine "));
    assert!(request.contains("PUT /open-apis/security_and_compliance/v2/device_records/device-1 "));
    assert!(
        request.contains("DELETE /open-apis/security_and_compliance/v2/device_records/device-1 ")
    );
    assert!(
        request.contains("PUT /open-apis/security_and_compliance/v2/device_apply_records/apply-1 ")
    );
    assert!(request.contains(r#""status":"active""#));
    assert!(request.contains(r#""status":"disabled""#));
    assert!(request.contains(r#""status":"approved""#));
}
