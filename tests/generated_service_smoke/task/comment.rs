use super::prelude::*;

// ── Task ──

#[tokio::test]
async fn task_comment_by_query_smoke() {
    let comment_body =
        r#"{"code":0,"msg":"ok","data":{"comment":{"id":"comment-1","content":"First"}}}"#;
    let comment_list_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"comment-1","content":"First"}],"has_more":false}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, comment_body),
        http_response(200, comment_body),
        http_response(200, comment_body),
        http_response(200, empty_body),
        http_response(200, comment_list_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateCommentReqBody {
        content: Some("First".into()),
        ..Default::default()
    };
    let update_body = UpdateCommentReqBody {
        content: Some("Updated".into()),
    };

    client
        .task()
        .comment
        .create_by_query(
            &CreateTaskCommentQuery::new("task-1", &create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .comment
        .get_by_query(
            &GetTaskCommentQuery::new("task-1", "comment-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .comment
        .update_by_query(
            &UpdateTaskCommentQuery::new("task-1", "comment-1", &update_body)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .comment
        .delete_by_query(
            &DeleteCommentQuery::new("task-1", "comment-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .comment
        .list_by_query(
            &ListTaskCommentQuery::new("task-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v1/tasks/task-1/comments?"));
    assert!(request.contains("GET /open-apis/task/v1/tasks/task-1/comments/comment-1?"));
    assert!(request.contains("PUT /open-apis/task/v1/tasks/task-1/comments/comment-1?"));
    assert!(request.contains("DELETE /open-apis/task/v1/tasks/task-1/comments/comment-1 "));
    assert!(request.contains("GET /open-apis/task/v1/tasks/task-1/comments?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""content":"First""#));
    assert!(request.contains(r#""content":"Updated""#));
}
