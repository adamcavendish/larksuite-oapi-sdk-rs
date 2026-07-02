use super::prelude::*;

// ── Task ──

#[tokio::test]
async fn task_member_by_query_smoke() {
    let follower_body =
        r#"{"code":0,"msg":"ok","data":{"follower":{"id":"u-1","id_type":"open_id"}}}"#;
    let collaborator_body =
        r#"{"code":0,"msg":"ok","data":{"collaborator":{"id":"u-1","id_type":"open_id"}}}"#;
    let list_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"u-1","id_type":"open_id"}],"has_more":false}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, follower_body),
        http_response(200, empty_body),
        http_response(200, list_body),
        http_response(200, collaborator_body),
        http_response(200, empty_body),
        http_response(200, list_body),
    ])
    .await;

    let client = client_for(addr);
    let follower_body = CreateFollowerReqBody {
        id: Some("u-1".into()),
        id_type: Some("open_id".into()),
    };
    let collaborator_body = CreateCollaboratorReqBody {
        id: Some("u-1".into()),
        id_type: Some("open_id".into()),
    };

    client
        .task()
        .follower
        .create_by_query(
            &CreateFollowerQuery::new("task-1", &follower_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .follower
        .delete_by_query(
            &DeleteFollowerQuery::new("task-1", "u-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .follower
        .list_by_query(
            &ListFollowerQuery::new("task-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .collaborator
        .create_by_query(
            &CreateCollaboratorQuery::new("task-1", &collaborator_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .collaborator
        .delete_by_query(
            &DeleteCollaboratorQuery::new("task-1", "u-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task()
        .collaborator
        .list_by_query(
            &ListCollaboratorQuery::new("task-1")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v1/tasks/task-1/followers?"));
    assert!(request.contains("DELETE /open-apis/task/v1/tasks/task-1/followers/u-1?"));
    assert!(request.contains("GET /open-apis/task/v1/tasks/task-1/followers?"));
    assert!(request.contains("POST /open-apis/task/v1/tasks/task-1/collaborators?"));
    assert!(request.contains("DELETE /open-apis/task/v1/tasks/task-1/collaborators/u-1?"));
    assert!(request.contains("GET /open-apis/task/v1/tasks/task-1/collaborators?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""id":"u-1""#));
    assert!(request.contains(r#""id_type":"open_id""#));
}
