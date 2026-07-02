use super::prelude::*;

// Drive comment smoke tests

#[tokio::test]
async fn drive_file_comment_batch_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"comment_id":"comment-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = BatchQueryFileCommentReqBody {
        comment_ids: Some(vec!["comment-1".to_string()]),
    };
    let resp = client
        .drive()
        .file_comment
        .batch_query_by_query(
            &BatchQueryFileCommentQuery::new("file-token-1", "doc", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/files/file-token-1/comments/batch_query?"));
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""comment_ids":["comment-1"]"#));
}

#[tokio::test]
async fn drive_file_comment_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"comment_id":"comment-1","user_id":"ou-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file_comment
        .get_by_query(
            &GetFileCommentQuery::new("file-token-1", "comment-1", "doc").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|comment| comment.comment_id.as_deref()),
        Some("comment-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/comments/comment-1?"));
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn drive_file_comment_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"comment_id":"comment-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file_comment
        .list_by_query(
            &ListFileCommentQuery::new("file-token-1", "doc")
                .is_whole(true)
                .is_solved(false)
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/comments?"));
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("is_whole=true"));
    assert!(request.contains("is_solved=false"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn drive_file_comment_reply_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"reply_id":"reply-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file_comment_reply
        .list_by_query(
            &ListFileCommentReplyQuery::new("file-token-1", "comment-1", "doc")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("GET /open-apis/drive/v1/files/file-token-1/comments/comment-1/replies?")
    );
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}
