use super::prelude::*;

// ── Approval ──

#[tokio::test]
async fn approval_get_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"approval_code":"apv-1","approval_name":"Leave Request","status":"ACTIVE"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .approval
        .get("apv-1", None, None, None, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.approval_name.as_deref(), Some("Leave Request"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/approvals/apv-1"));
}

#[tokio::test]
async fn approval_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"approval_code":"apv-1","approval_name":"Leave Request"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .approval
        .get_by_query(
            &GetApprovalQuery::new("apv-1")
                .locale("en-US")
                .user_id("ou-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.approval_name.as_deref()),
        Some("Leave Request")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/approvals/apv-1?"));
    assert!(request.contains("locale=en-US"));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("user_id_type=open_id"));
}
