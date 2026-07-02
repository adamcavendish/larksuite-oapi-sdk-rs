use super::prelude::*;

// ── Admin v1 ──

#[tokio::test]
async fn admin_audit_info_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"event_name":"login"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .audit_info
        .list_by_query(
            &ListAuditInfoQuery::new()
                .user_id_type("open_id")
                .latest("2026-06-30")
                .oldest("2026-06-01")
                .event_name("login")
                .operator_type("user")
                .operator_value("ou-1")
                .event_module(1)
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/audit_infos?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("latest=2026-06-30"));
    assert!(request.contains("oldest=2026-06-01"));
    assert!(request.contains("event_name=login"));
    assert!(request.contains("operator_type=user"));
    assert!(request.contains("operator_value=ou-1"));
    assert!(request.contains("event_module=1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
