use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::error::LarkError;
use larksuite_oapi_sdk_rs::req::{ApiReq, RequestOption};

fn default_option() -> RequestOption {
    RequestOption::default()
}

#[tokio::test]
async fn transport_rejects_empty_app_id() {
    let client = Client::builder("", "secret").build();
    let api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    let err = client
        .do_req(&api_req, &default_option())
        .await
        .unwrap_err();
    assert!(matches!(err, LarkError::IllegalParam(_)));
}

#[tokio::test]
async fn transport_rejects_empty_app_secret() {
    let client = Client::builder("app_id", "").build();
    let api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    let err = client
        .do_req(&api_req, &default_option())
        .await
        .unwrap_err();
    assert!(matches!(err, LarkError::IllegalParam(_)));
}

#[tokio::test]
async fn transport_rejects_marketplace_missing_tenant_key() {
    use larksuite_oapi_sdk_rs::constants::AccessTokenType;

    let client = Client::builder("app_id", "secret").marketplace().build();
    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

    let err = client
        .do_req(&api_req, &default_option())
        .await
        .unwrap_err();
    assert!(matches!(err, LarkError::IllegalParam(_)));
}

#[tokio::test]
async fn transport_user_access_token_option_sets_header() {
    // When a user_access_token is provided in options, the request should proceed
    // (fail at network level, not at validation level).
    use larksuite_oapi_sdk_rs::constants::AccessTokenType;

    let client = Client::builder("app_id", "secret")
        .disable_token_cache()
        .base_url("http://127.0.0.1:1") // connection refused — intentional
        .build();
    let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/test");
    api_req.supported_access_token_types = vec![AccessTokenType::User];

    let option = RequestOption {
        user_access_token: Some("user_token_123".to_string()),
        ..Default::default()
    };

    let err = client.do_req(&api_req, &option).await.unwrap_err();
    // Should fail at network level (DialFailed), not at IllegalParam
    assert!(
        matches!(err, LarkError::DialFailed(_)) || matches!(err, LarkError::Http(_)),
        "unexpected error: {err:?}"
    );
}

#[tokio::test]
async fn transport_rejects_reserved_x_request_id_header() {
    let client = Client::builder("app_id", "secret").build();
    let api_req = ApiReq::new(http::Method::GET, "/open-apis/test");

    let mut headers = http::HeaderMap::new();
    headers.insert("X-Request-Id", "custom".parse().unwrap());
    let option = RequestOption {
        headers: Some(headers),
        ..Default::default()
    };

    let err = client.do_req(&api_req, &option).await.unwrap_err();
    assert!(matches!(err, LarkError::IllegalParam(_)));
}

#[tokio::test]
async fn transport_rejects_reserved_request_id_header() {
    let client = Client::builder("app_id", "secret").build();
    let api_req = ApiReq::new(http::Method::GET, "/open-apis/test");

    let mut headers = http::HeaderMap::new();
    headers.insert("Request-Id", "custom".parse().unwrap());
    let option = RequestOption {
        headers: Some(headers),
        ..Default::default()
    };

    let err = client.do_req(&api_req, &option).await.unwrap_err();
    assert!(matches!(err, LarkError::IllegalParam(_)));
}
