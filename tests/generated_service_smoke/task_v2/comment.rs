use super::prelude::*;

// Task v2 comment smoke tests

#[tokio::test]
async fn task_v2_comment_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"comment":{"id":"comment-1","content":"Looks good"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetCommentV2Query::new("comment-1").user_id_type("open_id");
    let resp = client
        .task_v2()
        .comment
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let comment = resp.data.unwrap().comment.unwrap();
    assert_eq!(comment.id.as_deref(), Some("comment-1"));
    assert_eq!(comment.content.as_deref(), Some("Looks good"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/comments/comment-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_comment_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"comment-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task_v2()
        .comment
        .list(
            Some("task"),
            Some("task-guid-1"),
            Some(20),
            Some("next-page"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/comments?"));
    assert!(request.contains("resource_type=task"));
    assert!(request.contains("resource_id=task-guid-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_comment_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"comment-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCommentV2Query::new()
        .resource_type("task")
        .resource_id("task-guid-1")
        .direction("asc")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .task_v2()
        .comment
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].id.as_deref(), Some("comment-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/comments?"));
    assert!(request.contains("resource_type=task"));
    assert!(request.contains("resource_id=task-guid-1"));
    assert!(request.contains("direction=asc"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn task_v2_comment_write_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"comment":{"id":"comment-1","content":"Looks good"}}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let create_body = TaskV2InputComment::default();
    let patch_body = PatchCommentV2ReqBody::default();
    client
        .task_v2()
        .comment
        .create_by_query(
            &CreateCommentV2Query::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .comment
        .patch_by_query(
            &PatchCommentV2Query::new("comment-1", &patch_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/comments?"));
    assert!(request.contains("PATCH /open-apis/task/v2/comments/comment-1?"));
    assert!(request.contains("user_id_type=open_id"));
}
