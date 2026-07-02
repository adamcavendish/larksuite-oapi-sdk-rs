use super::prelude::*;

// ── Approval ──

#[tokio::test]
async fn approval_instance_comment_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"comment_id":"comment-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .instance_comment
        .list_by_query(
            &ListInstanceCommentQuery::new("inst-1")
                .user_id("ou-1")
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/instances/inst-1/comments?"));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
