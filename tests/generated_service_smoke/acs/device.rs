use super::prelude::*;

// ── ACS ──

#[tokio::test]
async fn acs_list_devices_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"device_id":"dev-1","device_name":"Front Gate"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .acs()
        .device
        .list_by_query(&ListDeviceQuery::new(), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert!(resp.data.is_some());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/devices"));
}
