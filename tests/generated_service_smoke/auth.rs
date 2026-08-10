use super::prelude::*;

// ── Auth ──

#[tokio::test]
async fn auth_v3_token_by_query_smoke() {
    let token_body = r#"{"code":0,"msg":"ok","app_access_token":"app-token","tenant_access_token":"tenant-token","expire":7200}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, token_body),
        http_response(200, token_body),
        http_response(200, token_body),
        http_response(200, token_body),
        http_response(200, token_body),
    ])
    .await;

    let client = client_for(addr);
    let app_token_body = CreateAppAccessTokenReqBody {
        app_id: Some("cli_a".into()),
        app_secret: Some("secret".into()),
        app_ticket: Some("ticket-1".into()),
    };
    let internal_app_body = InternalAppAccessTokenReqBody {
        app_id: Some("cli_a".into()),
        app_secret: Some("secret".into()),
    };
    let resend_body = ResendAppTicketReqBody {
        app_id: Some("cli_a".into()),
        app_secret: Some("secret".into()),
    };
    let tenant_body = CreateTenantAccessTokenReqBody {
        app_access_token: Some("app-token".into()),
        tenant_key: Some("tenant-1".into()),
    };
    let internal_tenant_body = InternalTenantAccessTokenReqBody {
        app_id: Some("cli_a".into()),
        app_secret: Some("secret".into()),
    };

    client
        .auth()
        .app_access_token
        .create_by_query(
            &CreateAppAccessTokenQuery::new(&app_token_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .auth()
        .app_access_token
        .internal_by_query(
            &InternalAppAccessTokenQuery::new(&internal_app_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .auth()
        .app_ticket
        .resend_by_query(
            &ResendAppTicketQuery::new(&resend_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .auth()
        .tenant_access_token
        .create_by_query(
            &CreateTenantAccessTokenQuery::new(&tenant_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .auth()
        .tenant_access_token
        .internal_by_query(
            &InternalTenantAccessTokenQuery::new(&internal_tenant_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/auth/v3/app_access_token "));
    assert!(request.contains("POST /open-apis/auth/v3/app_access_token/internal "));
    assert!(request.contains("POST /open-apis/auth/v3/app_ticket/resend "));
    assert!(request.contains("POST /open-apis/auth/v3/tenant_access_token "));
    assert!(request.contains("POST /open-apis/auth/v3/tenant_access_token/internal "));
    assert!(request.contains(r#""app_ticket":"ticket-1""#));
    assert!(request.contains(r#""app_access_token":"app-token""#));
    assert!(request.contains(r#""tenant_key":"tenant-1""#));
}

#[tokio::test]
async fn authen_v1_by_query_smoke() {
    let user_token_body = r#"{"code":0,"msg":"ok","data":{"access_token":"user-token","refresh_token":"refresh-token","open_id":"ou-1"}}"#;
    let oidc_token_body = r#"{"code":0,"msg":"ok","data":{"access_token":"oidc-token","refresh_token":"refresh-token","token_type":"Bearer"}}"#;
    let user_info_body = r#"{"code":0,"msg":"ok","data":{"name":"Alice","open_id":"ou-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, user_token_body),
        http_response(200, oidc_token_body),
        http_response(200, oidc_token_body),
        http_response(200, user_token_body),
        http_response(200, user_info_body),
    ])
    .await;

    let client = client_for(addr);
    let access_body = CreateAccessTokenReqBody {
        grant_type: Some("authorization_code".into()),
        code: Some("code-1".into()),
    };
    let oidc_body = CreateOidcAccessTokenReqBody {
        grant_type: Some("authorization_code".into()),
        code: Some("oidc-code-1".into()),
    };
    let oidc_refresh_body = CreateOidcRefreshAccessTokenReqBody {
        grant_type: Some("refresh_token".into()),
        refresh_token: Some("oidc-refresh-1".into()),
    };
    let refresh_body = CreateRefreshAccessTokenReqBody {
        grant_type: Some("refresh_token".into()),
        refresh_token: Some("refresh-1".into()),
    };

    client
        .authen()
        .access_token
        .create_by_query(
            &CreateAuthenAccessTokenQuery::new(&access_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .authen()
        .oidc_access_token
        .create_by_query(
            &CreateOidcAccessTokenQuery::new(&oidc_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .authen()
        .oidc_refresh_access_token
        .create_by_query(
            &CreateOidcRefreshAccessTokenQuery::new(&oidc_refresh_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .authen()
        .refresh_access_token
        .create_by_query(
            &CreateRefreshAccessTokenQuery::new(&refresh_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .authen()
        .user_info
        .get_by_query(&GetUserInfoQuery::new(), &RequestOption::default())
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/authen/v1/access_token "));
    assert!(request.contains("POST /open-apis/authen/v1/oidc/access_token "));
    assert!(request.contains("POST /open-apis/authen/v1/oidc/refresh_access_token "));
    assert!(request.contains("POST /open-apis/authen/v1/refresh_access_token "));
    assert!(request.contains("GET /open-apis/authen/v1/user_info "));
    assert!(request.contains(r#""code":"code-1""#));
    assert!(request.contains(r#""code":"oidc-code-1""#));
    assert!(request.contains(r#""refresh_token":"oidc-refresh-1""#));
    assert!(request.contains(r#""refresh_token":"refresh-1""#));
}

#[tokio::test]
async fn authen_oauth_v2_uses_open_api_base_url() {
    let token_body = r#"{"code":0,"access_token":"user-token","refresh_token":"refresh-token","token_type":"Bearer","expires_in":7200,"scope":"scope-a"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, token_body),
        http_response(200, token_body),
    ])
    .await;

    let client = client_for(addr);
    let option = RequestOption::default();

    client
        .authen()
        .oauth
        .retrieve_by_authorization_code(
            "authorization-code",
            Some("https://example.com/oauth/callback"),
            Some("pkce-verifier"),
            Some("requested-at-authorization"),
            &option,
        )
        .await
        .unwrap();
    client
        .authen()
        .oauth
        .refresh("refresh-token", Some("scope-a"), &option)
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert_eq!(
        request
            .matches("POST /open-apis/authen/v2/oauth/token ")
            .count(),
        2
    );
    assert!(request.contains(r#""grant_type":"authorization_code""#));
    assert!(request.contains(r#""code":"authorization-code""#));
    assert!(request.contains(r#""redirect_uri":"https://example.com/oauth/callback""#));
    assert!(request.contains(r#""code_verifier":"pkce-verifier""#));
    assert!(!request.contains("requested-at-authorization"));
    assert!(request.contains(r#""grant_type":"refresh_token""#));
    assert!(request.contains(r#""refresh_token":"refresh-token""#));
    assert!(request.contains(r#""scope":"scope-a""#));
    assert!(request.contains("content-type: application/json; charset=utf-8"));
}
