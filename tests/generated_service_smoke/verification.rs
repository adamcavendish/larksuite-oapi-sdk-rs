use super::prelude::*;

// ── Verification ──

#[tokio::test]
async fn verification_by_query_smoke() {
    let task_body = r#"{"code":0,"msg":"ok","data":{"task":{"task_id":"task-1","status":1}}}"#;
    let verification_body =
        r#"{"code":0,"msg":"ok","data":{"verification":{"id":"verification-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, task_body),
        http_response(200, task_body),
        http_response(200, verification_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateVerificationTaskReqBody {
        redirect_url: Some("https://example.test/done".into()),
        mobile: Some("13800138000".into()),
    };

    client
        .verification()
        .verification_task
        .create_by_query(
            &CreateVerificationTaskQuery::new("ou-1", &create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .verification()
        .verification_task
        .get_by_query(
            &GetVerificationTaskQuery::new("task-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .verification()
        .verification
        .get_by_query(&GetVerificationQuery::new(), &RequestOption::default())
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/verification/v1/verification_tasks?"));
    assert!(request.contains("GET /open-apis/verification/v1/verification_tasks/task-1 "));
    assert!(request.contains("GET /open-apis/verification/v1/verification "));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""redirect_url":"https://example.test/done""#));
    assert!(request.contains(r#""mobile":"13800138000""#));
}
