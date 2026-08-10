use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

const DEFAULT_FEISHU_REDIRECT_URI: &str = "http://127.0.0.1:8787/oauth/feishu/callback";

#[tokio::test]
#[ignore = "requires a live Feishu application, user authorization code, and PKCE verifier"]
async fn feishu_user_oauth_live_code_exchange_and_refresh() {
    let app_id = required_env("FEISHU_APP_ID");
    let app_secret = required_env("FEISHU_APP_SECRET");
    let authorization_code = required_env("FEISHU_OAUTH_CODE");
    let code_verifier = required_env("FEISHU_CODE_VERIFIER");
    let redirect_uri = std::env::var("FEISHU_REDIRECT_URI")
        .unwrap_or_else(|_| DEFAULT_FEISHU_REDIRECT_URI.to_owned());

    let client = LarkClient::builder(app_id, app_secret)
        .build()
        .expect("live Feishu OAuth client should build");
    let option = RequestOption::default();

    let authorization = client
        .authen()
        .oauth
        .retrieve_by_authorization_code(
            authorization_code.as_str(),
            Some(redirect_uri.as_str()),
            Some(code_verifier.as_str()),
            None,
            &option,
        )
        .await
        .expect("authorization code exchange should succeed");
    let refresh_token = authorization
        .data
        .as_ref()
        .and_then(|data| data.refresh_token.as_deref())
        .filter(|token| !token.is_empty())
        .map(str::to_owned)
        .expect("authorization response should include a refresh token");

    let refreshed = client
        .authen()
        .oauth
        .refresh(refresh_token.as_str(), None, &RequestOption::default())
        .await
        .expect("refresh token exchange should succeed");

    assert!(
        refreshed
            .data
            .as_ref()
            .and_then(|data| data.access_token.as_deref())
            .is_some_and(|token| !token.is_empty()),
        "refresh response should include an access token"
    );
}

fn required_env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} must be set for the live OAuth test"))
}
