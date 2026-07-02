use super::prelude::*;

// ── Attendance ──

#[tokio::test]
async fn attendance_archive_rule_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"archive_rule_id":"rule-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListArchiveRuleQuery::new()
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .attendance()
        .archive_rule
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.get("items"))
            .and_then(|items| items.as_array())
            .and_then(|items| items.first())
            .and_then(|item| item.get("archive_rule_id"))
            .and_then(|rule_id| rule_id.as_str()),
        Some("rule-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/archive_rule?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
