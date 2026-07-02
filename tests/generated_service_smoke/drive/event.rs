use super::prelude::*;

// Drive event smoke tests

#[tokio::test]
async fn event_outbound_ip_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ip_list":["1.1.1.1","2.2.2.2"]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .event()
        .outbound_ip
        .list_by_query(&ListOutboundIpQuery::new(), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.ip_list.as_ref())
            .and_then(|ips| ips.first())
            .map(String::as_str),
        Some("1.1.1.1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/event/v1/outbound_ip "));
}
