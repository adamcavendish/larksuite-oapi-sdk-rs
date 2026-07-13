use super::prelude::*;

// ── Application v6 ──

#[tokio::test]
async fn application_v6_feedback_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"feedback_id":"fb-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_feedback
        .list_by_query(
            &ListApplicationFeedbackQuery::new("cli_a")
                .from_date("2026-06-01")
                .to_date("2026-06-30")
                .feedback_type(1)
                .status(2)
                .user_id_type("open_id")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications/cli_a/feedbacks?"));
    assert!(request.contains("from_date=2026-06-01"));
    assert!(request.contains("to_date=2026-06-30"));
    assert!(request.contains("feedback_type=1"));
    assert!(request.contains("status=2"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn application_v6_feedback_patch_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_feedback
        .patch_by_query(
            &PatchApplicationFeedbackQuery::new("cli_a", "fb-1")
                .user_id_type("open_id")
                .status(2)
                .operator_id("ou-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/application/v6/applications/cli_a/feedbacks/fb-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("status=2"));
    assert!(request.contains("operator_id=ou-1"));
}
