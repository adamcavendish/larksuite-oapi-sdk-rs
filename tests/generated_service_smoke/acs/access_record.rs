use super::prelude::*;

// ── ACS ──

#[tokio::test]
async fn acs_access_record_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"access_record_id":"rec-1","device_id":"dev-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .acs()
        .access_record
        .list_by_query(
            &ListAcsAccessRecordQuery::new()
                .page(PageQuery::new().page_size(50).page_token("next-page"))
                .from(1_700_000_000)
                .to(1_700_000_100)
                .device_id("dev-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|record| record.access_record_id.as_deref()),
        Some("rec-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/access_records?"));
    assert!(request.contains("page_size=50"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("from=1700000000"));
    assert!(request.contains("to=1700000100"));
    assert!(request.contains("device_id=dev-1"));
    assert!(request.contains("user_id_type=open_id"));
}
