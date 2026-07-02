use super::prelude::*;

// ── Application v6 ──

#[tokio::test]
async fn application_v6_collaborators_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v6()
        .application_collaborators
        .get_by_query(
            &GetApplicationCollaboratorsQuery::new("cli_a").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v6/applications/cli_a/collaborators?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn application_v6_collaborators_and_contacts_range_write_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let collaborators_body = serde_json::json!({"collaborators":["ou-1"]});
    let contacts_body = serde_json::json!({"visible_users":["ou-1"]});
    client
        .application_v6()
        .application_collaborators
        .update_by_query(
            &UpdateApplicationCollaboratorsQuery::new("cli_a", &collaborators_body)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .application_v6()
        .application_contacts_range
        .patch_by_query(
            &PatchApplicationContactsRangeQuery::new("cli_a", &contacts_body)
                .user_id_type("open_id")
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PUT /open-apis/application/v6/applications/cli_a/collaborators?"));
    assert!(request.contains("PATCH /open-apis/application/v6/applications/cli_a/contacts_range?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains(r#""collaborators":["ou-1"]"#));
    assert!(request.contains(r#""visible_users":["ou-1"]"#));
}
