use super::prelude::*;

// ── Application v6 ──

#[tokio::test]
async fn application_v6_management_and_owner_write_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let management_body = serde_json::json!({"enabled":true});
    let owner_body = serde_json::json!({"owner_id":"ou-1"});
    client
        .application_v6()
        .application_management
        .update_by_query(
            &UpdateApplicationManagementQuery::new("cli_a", &management_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .application_v6()
        .application_owner
        .update_by_query(
            &UpdateApplicationOwnerQuery::new("cli_a", &owner_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PUT /open-apis/application/v6/applications/cli_a/management "));
    assert!(request.contains("PUT /open-apis/application/v6/applications/cli_a/owner?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""enabled":true"#));
    assert!(request.contains(r#""owner_id":"ou-1""#));
}
