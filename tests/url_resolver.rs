mod common;

use base64::Engine;
use common::{http_response, http_response_with_headers, mock_server_with_requests};
use larksuite_oapi_sdk_rs::cache::LocalCache;
use larksuite_oapi_sdk_rs::dpop::{DPoPBinding, DPoPKey, DPoPMode};
use larksuite_oapi_sdk_rs::token::{ClientAssertionProvider, Token, TokenManager};
use larksuite_oapi_sdk_rs::{AccessTokenType, ApiReq, LarkClient, LarkError, RequestOption};
use std::sync::{Arc, Mutex};

fn claims(request: &str) -> serde_json::Value {
    let proof = request
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("dpop").then(|| value.trim())
        })
        .unwrap();
    serde_json::from_slice(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(proof.split('.').nth(1).unwrap())
            .unwrap(),
    )
    .unwrap()
}

fn req(method: http::Method, path: &str) -> ApiReq {
    let mut req = ApiReq::new(method, path);
    req.supported_access_token_types = vec![AccessTokenType::User];
    req
}

fn user_option() -> RequestOption {
    RequestOption {
        user_access_token: Some("user-token".into()),
        ..Default::default()
    }
}

#[tokio::test]
async fn api_and_download_route_but_external_presigned_url_stays_verbatim() {
    let (addr, server, requests) =
        mock_server_with_requests(vec![http_response(200, "payload")]).await;
    let seen = Arc::new(Mutex::new(Vec::new()));
    let recorded = seen.clone();
    let client = LarkClient::builder("app", "secret")
        .base_url("https://platform.example")
        .platform_url_resolver(Arc::new(move |logical: &str| {
            recorded.lock().unwrap().push(logical.to_owned());
            let url = url::Url::parse(logical).unwrap();
            Ok(format!(
                "http://{addr}/gateway{}{}",
                url.path(),
                url.query().map(|q| format!("?{q}")).unwrap_or_default()
            ))
        }))
        .build()
        .unwrap();
    let mut request = req(http::Method::GET, "/open-apis/example/:id");
    request.path_params.set("id", "a/b");
    request.query_params.add("page", "2");
    client.raw_request(&request, &user_option()).await.unwrap();
    let download = client
        .drive()
        .file
        .download("file", &user_option())
        .await
        .unwrap();
    assert_eq!(download.data, b"payload");
    assert_eq!(
        client
            .download_file(&format!("http://{addr}/external?signature=exact%2Fvalue"))
            .await
            .unwrap(),
        b"payload"
    );
    let seen = seen.lock().unwrap();
    assert_eq!(seen.len(), 2);
    assert_eq!(
        seen[0],
        "https://platform.example/open-apis/example/a%2Fb?page=2"
    );
    let requests = requests.lock().unwrap();
    assert!(requests[0].starts_with("GET /gateway/open-apis/example/a%2Fb?page=2 "));
    assert!(requests[1].starts_with("GET /gateway/open-apis/drive/v1/files/file/download "));
    assert!(requests[2].starts_with("GET /external?signature=exact%2Fvalue "));
    assert!(!requests[2].to_ascii_lowercase().contains("authorization:"));
    server.abort();
}

#[derive(Debug)]
struct Assertion(Arc<Mutex<Vec<String>>>);
impl ClientAssertionProvider for Assertion {
    fn retrieve_token(
        &self,
        aud: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Token, LarkError>> + Send + '_>>
    {
        self.0.lock().unwrap().push(aud.into());
        Box::pin(async {
            Ok(Token {
                value: "assertion".into(),
                target_info: None,
            })
        })
    }
}

#[tokio::test]
async fn oauth_and_resource_proofs_bind_effective_url_without_changing_audience() {
    let (addr, server, requests) = mock_server_with_requests(vec![
        http_response(
            200,
            r#"{"code":0,"access_token":"bound","expires_in":7200,"token_type":"DPoP"}"#,
        ),
        http_response(200, r#"{"code":0}"#),
    ])
    .await;
    let audiences = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::new(Mutex::new(Vec::new()));
    let recorded = seen.clone();
    let client = LarkClient::builder("app", "secret")
        .base_url("https://api.example")
        .oauth_base_url("https://oauth.example")
        .client_assertion_provider(Arc::new(Assertion(audiences.clone())))
        .dpop_mode(DPoPMode::Required)
        .platform_url_resolver(Arc::new(move |logical: &str| {
            recorded.lock().unwrap().push(logical.to_owned());
            Ok(format!(
                "http://{addr}/mapped{}",
                url::Url::parse(logical).unwrap().path()
            ))
        }))
        .build()
        .unwrap();
    let token = TokenManager::new(Arc::new(LocalCache::new()))
        .get_client_assertion_tenant_token(client.config(), None)
        .await
        .unwrap();
    let option = RequestOption {
        user_access_token: Some(token.access_token),
        dpop_binding: token.dpop_binding,
        ..Default::default()
    };
    client
        .raw_request(&req(http::Method::GET, "/open-apis/resource"), &option)
        .await
        .unwrap();
    assert_eq!(*audiences.lock().unwrap(), vec!["oauth.example"]);
    assert_eq!(seen.lock().unwrap().len(), 2);
    let requests = requests.lock().unwrap();
    assert_eq!(
        claims(&requests[0])["htu"],
        format!("http://{addr}/mapped/oauth/v3/token")
    );
    assert_eq!(claims(&requests[0])["htm"], "POST");
    assert_eq!(
        claims(&requests[1])["htu"],
        format!("http://{addr}/mapped/open-apis/resource")
    );
    server.abort();
}

#[tokio::test]
async fn invalid_destination_or_resolver_failure_does_not_send_or_echo_secret() {
    let (addr, server, requests) = mock_server_with_requests(vec![http_response(200, "{}")]).await;
    for destination in [
        "file:///secret",
        "https://user:password@example.com/path?secret=value",
        "https://example.com/#secret",
        "invalid?secret=value",
    ] {
        let client = LarkClient::builder("app", "secret")
            .base_url(format!("http://{addr}"))
            .platform_url_resolver(Arc::new(move |_: &str| Ok(destination.to_owned())))
            .build()
            .unwrap();
        let err = client
            .raw_request(&req(http::Method::GET, "/resource"), &user_option())
            .await
            .unwrap_err();
        assert!(!err.to_string().contains("password"));
        assert!(!err.to_string().contains("secret=value"));
    }
    let client = LarkClient::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .platform_url_resolver(Arc::new(|_: &str| {
            Err(LarkError::IllegalParam("route unavailable".into()))
        }))
        .build()
        .unwrap();
    assert!(
        client
            .raw_request(&req(http::Method::GET, "/resource"), &user_option())
            .await
            .is_err()
    );
    assert!(requests.lock().unwrap().is_empty());
    server.abort();
}

#[tokio::test]
async fn redirects_strip_credentials_and_dpop_proofs_are_not_replayed() {
    let (target, target_server, target_requests) =
        mock_server_with_requests(vec![http_response(200, "{}")]).await;
    let redirect =
        http_response_with_headers(302, &format!("Location: http://{target}/next\r\n"), "");
    let (addr, server, requests) = mock_server_with_requests(vec![redirect]).await;
    let client = LarkClient::builder("app", "secret")
        .platform_url_resolver(Arc::new(move |_: &str| {
            Ok(format!("http://{addr}/gateway"))
        }))
        .build()
        .unwrap();
    client
        .raw_request(&req(http::Method::GET, "/resource"), &user_option())
        .await
        .unwrap();
    assert_eq!(target_requests.lock().unwrap().len(), 1);
    assert!(
        !target_requests.lock().unwrap()[0]
            .to_ascii_lowercase()
            .contains("authorization:")
    );
    let option = RequestOption {
        user_access_token: Some("bound".into()),
        dpop_binding: Some(DPoPBinding::new("bound", DPoPKey::generate()).unwrap()),
        ..Default::default()
    };
    let response = client
        .raw_request(&req(http::Method::GET, "/resource"), &option)
        .await
        .unwrap();
    assert_eq!(response.status_code, 302);
    assert_eq!(target_requests.lock().unwrap().len(), 1);
    assert!(
        requests.lock().unwrap()[1]
            .to_ascii_lowercase()
            .contains("dpop:")
    );
    // A cross-origin redirect must not replay an unsafe method/body.
    let response = client
        .raw_request(&req(http::Method::POST, "/resource"), &user_option())
        .await
        .unwrap();
    assert_eq!(response.status_code, 302);
    assert_eq!(target_requests.lock().unwrap().len(), 1);
    server.abort();
    target_server.abort();
}

#[tokio::test]
async fn registration_resolves_begin_poll_and_brand_switch_without_rewriting_qr_link() {
    use larksuite_oapi_sdk_rs::registration::{Options, register_app_with_url_resolver};
    let (addr, server, requests) = mock_server_with_requests(vec![
        http_response(200, r#"{"device_code":"device","verification_uri_complete":"https://verify.example/scan?code=keep","interval":1,"expire_in":60}"#),
        http_response(200, r#"{"user_info":{"open_id":"user","tenant_brand":"lark"}}"#),
        http_response(200, r#"{"client_id":"registered","client_secret":"secret"}"#),
    ]).await;
    let seen = Arc::new(Mutex::new(Vec::new()));
    let recorded = seen.clone();
    let qr = Arc::new(Mutex::new(String::new()));
    let qr_record = qr.clone();
    let options = Options {
        source: String::new(),
        domain: "https://accounts.feishu.cn".into(),
        lark_domain: "https://accounts.larksuite.com".into(),
        app_preset: None,
        addons: None,
        create_only: false,
        app_id: String::new(),
        on_qr_code: Box::new(move |info| *qr_record.lock().unwrap() = info.url.clone()),
        on_status_change: None,
    };
    let result = register_app_with_url_resolver(
        options,
        Some(DPoPKey::generate()),
        Some(Arc::new(move |logical: &str| {
            recorded.lock().unwrap().push(logical.to_owned());
            Ok(format!("http://{addr}/registration"))
        })),
    )
    .await
    .unwrap();
    assert_eq!(result.client_id, "registered");
    assert!(
        qr.lock()
            .unwrap()
            .starts_with("https://verify.example/scan?code=keep")
    );
    let seen = seen.lock().unwrap();
    assert_eq!(seen.len(), 3);
    assert_eq!(
        seen[0],
        "https://accounts.feishu.cn/oauth/v1/app/registration"
    );
    assert_eq!(
        seen[2],
        "https://accounts.larksuite.com/oauth/v1/app/registration"
    );
    let requests = requests.lock().unwrap();
    assert!(!requests[0].to_ascii_lowercase().contains("dpop:"));
    assert_eq!(
        claims(&requests[1])["htu"],
        format!("http://{addr}/registration")
    );
    assert_eq!(
        claims(&requests[2])["htu"],
        format!("http://{addr}/registration")
    );
    server.abort();
}

#[tokio::test]
async fn streaming_download_uses_resolver_before_reading_body() {
    use larksuite_oapi_sdk_rs::service::im::v1::GetMessageResourceDownloadQuery;
    let (addr, server, requests) =
        mock_server_with_requests(vec![http_response(200, "stream-body")]).await;
    let client = LarkClient::builder("app", "secret")
        .platform_url_resolver(Arc::new(move |_: &str| Ok(format!("http://{addr}/stream"))))
        .build()
        .unwrap();
    let mut response = client
        .im()
        .message_resource
        .get_stream_by_query(
            &GetMessageResourceDownloadQuery::new("message", "file", "file"),
            &user_option(),
        )
        .await
        .unwrap();
    let mut bytes = Vec::new();
    while let Some(chunk) = response.body.next_chunk().await.unwrap() {
        bytes.extend_from_slice(&chunk);
    }
    assert_eq!(bytes, b"stream-body");
    assert!(requests.lock().unwrap()[0].starts_with("GET /stream "));
    server.abort();
}

#[tokio::test]
async fn device_authorization_resolves_absolute_platform_url_and_keeps_verification_link() {
    let (addr, server, requests) = mock_server_with_requests(vec![http_response(200,
        r#"{"device_code":"device","user_code":"user","verification_uri":"https://verify.example/scan","verification_uri_complete":"https://verify.example/scan?code=user","expires_in":600,"interval":5}"#)]).await;
    let logical = Arc::new(Mutex::new(String::new()));
    let recorded = logical.clone();
    let client = LarkClient::builder("app", "secret")
        .platform_url_resolver(Arc::new(move |url: &str| {
            *recorded.lock().unwrap() = url.into();
            Ok(format!("http://{addr}/device"))
        }))
        .build()
        .unwrap();
    let response = client
        .authen()
        .oauth
        .request_device_authorization(None, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(
        response.verification_uri_complete,
        "https://verify.example/scan?code=user"
    );
    assert!(
        logical
            .lock()
            .unwrap()
            .starts_with("https://accounts.feishu.cn/")
    );
    assert!(requests.lock().unwrap()[0].starts_with("POST /device "));
    assert!(
        requests.lock().unwrap()[0]
            .to_ascii_lowercase()
            .contains("authorization: basic ")
    );
    server.abort();
}

#[tokio::test]
async fn registration_rejects_redirect_before_parsing_success_shaped_body() {
    use larksuite_oapi_sdk_rs::registration::{Options, register_app_with_url_resolver};
    let (addr, server, requests) = mock_server_with_requests(vec![http_response_with_headers(302,
        "Location: https://other.example/registration\r\n",
        r#"{"device_code":"device","verification_uri_complete":"https://verify.example/scan","interval":1,"expire_in":60}"#)]).await;
    let options = Options {
        source: String::new(),
        domain: String::new(),
        lark_domain: String::new(),
        app_preset: None,
        addons: None,
        create_only: false,
        app_id: String::new(),
        on_qr_code: Box::new(|_| panic!("redirect body must not start registration")),
        on_status_change: None,
    };
    let err = register_app_with_url_resolver(
        options,
        None,
        Some(Arc::new(move |_: &str| {
            Ok(format!("http://{addr}/registration"))
        })),
    )
    .await
    .unwrap_err();
    assert!(
        matches!(err, LarkError::Registration(ref message) if message == "registration redirect refused")
    );
    assert_eq!(requests.lock().unwrap().len(), 1);
    server.abort();
}
