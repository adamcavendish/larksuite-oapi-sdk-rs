use super::prelude::*;

// ── IM ──

#[tokio::test]
async fn im_list_pin_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"message_id":"om_1","chat_id":"oc_1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .im()
        .pin
        .list_by_query(
            &ListPinQuery::new("oc_1")
                .start_time("1782910000")
                .end_time("1782913600")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/pins?"));
    assert!(request.contains("chat_id=oc_1"));
    assert!(request.contains("start_time=1782910000"));
    assert!(request.contains("end_time=1782913600"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
