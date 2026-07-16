use super::prelude::*;

// ── Application v6 ──

#[tokio::test]
async fn application_v6_visibility_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let check_body = CheckWhiteBlackListApplicationVisibilityReqBody::default();
    let patch_body = PatchApplicationVisibilityReqBody::default();
    client
        .application_v6()
        .application_visibility
        .check_white_black_list_by_query(
            &CheckWhiteBlackListApplicationVisibilityQuery::new("cli_a", &check_body)
                .user_id_type("open_id")
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .application_v6()
        .application_visibility
        .patch_by_query(
            &PatchApplicationVisibilityQuery::new("cli_a", &patch_body)
                .department_id_type("open_department_id")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains(
        "POST /open-apis/application/v6/applications/cli_a/visibility/check_white_black_list?"
    ));
    assert!(request.contains("PATCH /open-apis/application/v6/applications/cli_a/visibility?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
}
