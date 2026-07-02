use super::*;

// ── Passport ──

#[tokio::test]
async fn passport_session_by_query_smoke() {
    let query_body = r#"{"code":0,"msg":"ok","data":{"session_list":[{"sid":"sid-1","uid":"u-1"}],"has_more":false}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, query_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let query_body = QuerySessionReqBody {
        user_ids: Some(vec!["u-1".into()]),
    };
    let logout_body = LogoutSessionReqBody {
        logout_type: Some(1),
        user_id: Some("u-1".into()),
        sid: Some("sid-1".into()),
        ..Default::default()
    };

    client
        .passport()
        .session
        .query_by_query(
            &QuerySessionQuery::new(&query_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .passport()
        .session
        .logout_by_query(
            &LogoutSessionQuery::new(&logout_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/passport/v1/sessions/query?"));
    assert!(request.contains("POST /open-apis/passport/v1/sessions/logout?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""user_ids":["u-1"]"#));
    assert!(request.contains(r#""logout_type":1"#));
    assert!(request.contains(r#""sid":"sid-1""#));
}
