use super::prelude::*;

// ── Human Authentication ──

#[tokio::test]
async fn human_authentication_identity_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"verify_uid":"verify-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let create_body = CreateIdentityReqBody {
        identity_name: Some("Alice".into()),
        identity_code: Some("ID-1".into()),
        mobile: Some("13800000000".into()),
    };

    client
        .human_authentication()
        .identity
        .create_by_query(
            &CreateHumanIdentityQuery::new("user-1", &create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/human_authentication/v1/identities?"));
    assert!(request.contains("user_id=user-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""identity_name":"Alice""#));
    assert!(request.contains(r#""identity_code":"ID-1""#));
    assert!(request.contains(r#""mobile":"13800000000""#));
}
