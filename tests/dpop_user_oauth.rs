mod common;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use common::{http_response, http_response_with_headers, mock_server_with_requests};
use http::{HeaderMap, HeaderValue};
use larksuite_oapi_sdk_rs::{
    DPoPClock, DPoPError, DPoPKey, LarkClient, LarkError, OffsetDPoPClock,
};
use serde_json::{Value, json};

const LOCAL_TIME: i64 = 1_700_000_000;
const SERVER_DATE: &str = "Tue, 14 Nov 2023 22:14:20 GMT";
const REJECTED_PROOF: &str = r#"{"code":20153,"error":"invalid_dpop_proof"}"#;
const ROTATED_TOKEN: &str = r#"{"access_token":"new-uat","token_type":"DPoP","refresh_token":"new-refresh","expires_in":7200,"refresh_token_expires_in":86400,"scope":"offline_access"}"#;

#[derive(Debug)]
struct FixedClock;

impl DPoPClock for FixedClock {
    fn now_unix_seconds(&self) -> Result<i64, DPoPError> {
        Ok(LOCAL_TIME)
    }
}

fn clock(max_offset: i64) -> OffsetDPoPClock {
    OffsetDPoPClock::with_base(Arc::new(FixedClock), max_offset).unwrap()
}

fn header<'a>(request: &'a str, name: &str) -> Option<&'a str> {
    request.lines().skip(1).find_map(|line| {
        let (key, value) = line.split_once(':')?;
        key.eq_ignore_ascii_case(name).then(|| value.trim())
    })
}

fn proof_part(proof: &str, index: usize) -> Value {
    let bytes = URL_SAFE_NO_PAD
        .decode(proof.split('.').nth(index).unwrap())
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

fn ambient_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    for name in [
        "authorization",
        "dpop",
        "cookie",
        "x-lark-helpdesk-authorization",
    ] {
        headers.insert(name, HeaderValue::from_static("caller-credential"));
    }
    headers
}

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("app", "secret")
        .oauth_base_url(format!("http://{addr}"))
        .build()
        .unwrap()
}

#[tokio::test]
async fn refresh_resolves_accounts_routes_and_signs_with_the_original_key() {
    for (brand, oauth_override, logical_base) in [
        ("https://open.feishu.cn", None, "https://accounts.feishu.cn"),
        (
            "https://open.larksuite.com",
            None,
            "https://accounts.larksuite.com",
        ),
        (
            "https://open.larksuite.com",
            Some("https://oauth.example/deployment/"),
            "https://oauth.example/deployment",
        ),
    ] {
        let (addr, server, requests) = mock_server_with_requests(vec![
            http_response(200, ROTATED_TOKEN),
            http_response(200, r#"{"access_token":"other","token_type":"DPoP"}"#),
            http_response(
                200,
                r#"{"access_token":"other","token_type":"DPoP","refresh_token":""}"#,
            ),
        ])
        .await;
        let seen = Arc::new(Mutex::new(Vec::new()));
        let routes = Arc::clone(&seen);
        let mut builder = LarkClient::builder("app", "secret")
            .base_url(brand)
            .default_headers(ambient_headers())
            .platform_url_resolver(Arc::new(move |logical: &str| {
                routes.lock().unwrap().push(logical.to_owned());
                Ok(format!("http://{addr}/gateway/oauth/v3/token"))
            }));
        if let Some(base) = oauth_override {
            builder = builder.oauth_base_url(base);
        }
        let client = builder.build().unwrap();
        let authen = client.authen();
        let key = DPoPKey::generate();
        let clock = clock(120);
        let result = authen
            .oauth
            .refresh_dpop("old-refresh", &key, &clock)
            .await
            .unwrap();
        assert_eq!(result.binding.access_token(), "new-uat");
        assert_eq!(result.binding.key_thumbprint(), key.thumbprint());
        assert_eq!(result.data.refresh_token.as_deref(), Some("new-refresh"));
        assert_eq!(result.data.scope.as_deref(), Some("offline_access"));
        assert_eq!(result.data.expires_in, Some(7200));
        assert_eq!(result.data.refresh_token_expires_in, Some(86400));
        assert!(!format!("{result:?}").contains("new-refresh"));
        assert!(!format!("{result:?}").contains("new-uat"));
        for _ in 0..2 {
            let result = authen
                .oauth
                .refresh_dpop("new-refresh", &key, &clock)
                .await
                .unwrap();
            assert!(result.data.refresh_token.is_none());
        }

        assert_eq!(
            *seen.lock().unwrap(),
            vec![format!("{logical_base}/oauth/v3/token"); 3]
        );
        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 3);
        let request = &requests[0];
        assert!(request.starts_with("POST /gateway/oauth/v3/token "));
        assert!(!request.contains("caller-credential"));
        let body: Value = serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(
            body,
            json!({
                "grant_type": "refresh_token",
                "client_id": "app",
                "client_secret": "secret",
                "refresh_token": "old-refresh",
            })
        );
        let proof = header(request, "dpop").unwrap();
        let expected = key
            .proof("POST", "https://accounts.example/", LOCAL_TIME)
            .unwrap();
        assert_eq!(proof_part(proof, 0), proof_part(&expected, 0));
        let claims = proof_part(proof, 1);
        assert_eq!(claims["htm"], "POST");
        assert_eq!(
            claims["htu"],
            format!("http://{addr}/gateway/oauth/v3/token")
        );
        assert_eq!(claims["iat"], LOCAL_TIME);
        assert!(claims.get("ath").is_none());
        server.abort();
    }
}

#[tokio::test]
async fn an_explicit_clock_rejection_retries_once_with_a_new_proof() {
    for second_response in [
        http_response(200, ROTATED_TOKEN),
        http_response_with_headers(401, &format!("Date: {SERVER_DATE}\r\n"), REJECTED_PROOF),
    ] {
        let second_rejected = second_response.starts_with("HTTP/1.1 401");
        let (addr, server, requests) = mock_server_with_requests(vec![
            http_response_with_headers(400, &format!("Date: {SERVER_DATE}\r\n"), REJECTED_PROOF),
            second_response,
        ])
        .await;
        let key = DPoPKey::generate();
        let clock = clock(120);
        let result = client_for(addr)
            .authen()
            .oauth
            .refresh_dpop("old-refresh", &key, &clock)
            .await;
        if second_rejected {
            let LarkError::Api(error) = result.unwrap_err() else {
                panic!("expected OAuth rejection");
            };
            assert_eq!(error.code, 20153);
            assert!(error.msg.contains("HTTP 401"));
        } else {
            assert_eq!(result.unwrap().binding.access_token(), "new-uat");
        }

        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 2);
        let first = header(&requests[0], "dpop").unwrap();
        let second = header(&requests[1], "dpop").unwrap();
        assert_eq!(proof_part(first, 0), proof_part(second, 0));
        let first = proof_part(first, 1);
        let second = proof_part(second, 1);
        assert_eq!(first["iat"], LOCAL_TIME);
        assert_eq!(second["iat"], LOCAL_TIME + 60);
        assert_ne!(first["jti"], second["jti"]);
        assert_eq!(first["htu"], second["htu"]);
        assert_eq!(clock.now_unix_seconds().unwrap(), LOCAL_TIME + 60);
        server.abort();
    }
}

#[tokio::test]
async fn refresh_failure_does_not_replay_or_change_the_clock() {
    let date = format!("Date: {SERVER_DATE}\r\n");
    let responses = [
        http_response(200, REJECTED_PROOF),
        http_response_with_headers(400, "Date: invalid\r\n", REJECTED_PROOF),
        http_response_with_headers(400, &date, r#"{"code":20153,"error":"invalid_grant"}"#),
        http_response_with_headers(400, &date, r#"{"code":999,"error":"invalid_dpop_proof"}"#),
        http_response_with_headers(
            200,
            &date,
            r#"{"code":20153,"error":"invalid_dpop_proof","refresh_token":"possibly-rotated"}"#,
        ),
        http_response_with_headers(429, &date, REJECTED_PROOF),
        http_response_with_headers(504, &date, REJECTED_PROOF),
        http_response(200, r#"{"access_token":"new-uat","token_type":"Bearer"}"#),
        http_response(200, r#"{"token_type":"DPoP"}"#),
        http_response(200, r#"{"code":20153,"error":"invalid_dpop_proof""#),
        format!("HTTP/1.1 200 OK\r\n{date}Content-Length: 10000\r\n\r\n{REJECTED_PROOF}"),
    ];
    let key = DPoPKey::generate();
    for failure in responses {
        let (addr, server, requests) =
            mock_server_with_requests(vec![failure, http_response(200, ROTATED_TOKEN)]).await;
        let clock = clock(120);
        let result = client_for(addr)
            .authen()
            .oauth
            .refresh_dpop("old-refresh", &key, &clock)
            .await;
        assert!(result.is_err());
        assert_eq!(requests.lock().unwrap().len(), 1);
        assert_eq!(clock.now_unix_seconds().unwrap(), LOCAL_TIME);
        server.abort();
    }

    let (addr, server, requests) = mock_server_with_requests(vec![
        http_response_with_headers(400, &date, REJECTED_PROOF),
        http_response(200, ROTATED_TOKEN),
    ])
    .await;
    let clock = clock(30);
    let result = client_for(addr)
        .authen()
        .oauth
        .refresh_dpop("old-refresh", &key, &clock)
        .await;
    assert!(matches!(result, Err(LarkError::Api(error)) if error.code == 20153));
    assert_eq!(requests.lock().unwrap().len(), 1);
    assert_eq!(clock.now_unix_seconds().unwrap(), LOCAL_TIME);
    server.abort();
}

#[tokio::test]
async fn accounts_requests_reject_redirects_without_a_url_resolver() {
    let (addr, server, requests) = mock_server_with_requests(vec![http_response_with_headers(
        307,
        "Location: /other\r\n",
        "",
    )])
    .await;
    let clock = clock(120);
    let client = client_for(addr);
    let authen = client.authen();
    let result = authen
        .oauth
        .refresh_dpop("old-refresh", &DPoPKey::generate(), &clock)
        .await;
    assert!(matches!(result, Err(LarkError::Token(message)) if message.contains("HTTP 307")));
    let result = authen.oauth.sync_dpop_clock(&clock).await;
    assert!(matches!(result, Err(LarkError::Token(message)) if message.contains("HTTP 307")));
    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert!(requests[0].starts_with("POST /oauth/v3/token "));
    assert!(requests[1].starts_with("POST /accounts/status/heartbeat "));
    server.abort();
}

#[tokio::test]
async fn heartbeat_is_credential_free_and_updates_only_for_a_valid_response() {
    let valid = r#"{"code":0,"data":{"now":"1700000060"}}"#;
    let oversized = format!("{valid}{}", " ".repeat(1024 * 1024));
    let failures = [
        http_response(500, valid),
        http_response(200, r#"{"data":{"now":"1700000000"}}"#),
        http_response(200, r#"{"code":42}"#),
        http_response(200, r#"{"code":0,"data":{"now":"-1"}}"#),
        http_response(200, r#"{"code":0,"data":{"now":"invalid"}}"#),
        http_response(200, r#"{"code":0,"data":{"now":"1700009999"}}"#),
        http_response(200, &oversized),
    ];
    let (addr, server, requests) = mock_server_with_requests(
        std::iter::once(http_response(200, valid))
            .chain(failures.clone())
            .collect(),
    )
    .await;
    let client = LarkClient::builder("app", "secret")
        .base_url("https://open.larksuite.com")
        .default_headers(ambient_headers())
        .platform_url_resolver(Arc::new(move |logical: &str| {
            assert_eq!(
                logical,
                "https://accounts.larksuite.com/accounts/status/heartbeat"
            );
            Ok(format!("http://{addr}/gateway/accounts/status/heartbeat"))
        }))
        .build()
        .unwrap();
    let authen = client.authen();
    let clock = clock(120);
    authen.oauth.sync_dpop_clock(&clock).await.unwrap();
    for _ in failures {
        assert!(authen.oauth.sync_dpop_clock(&clock).await.is_err());
        assert_eq!(clock.now_unix_seconds().unwrap(), LOCAL_TIME + 60);
    }
    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 8);
    for request in requests.iter() {
        assert!(request.starts_with("POST /gateway/accounts/status/heartbeat "));
        assert!(!request.contains("caller-credential"));
        assert_eq!(request.split_once("\r\n\r\n").unwrap().1, "");
    }
    server.abort();
}

#[tokio::test]
async fn accounts_deadline_covers_the_response_body() {
    for heartbeat in [true, false] {
        let (addr, server, requests, release) =
            common::mock_server_with_gated_body(200, "", "{", vec!["}"]).await;
        let client = LarkClient::builder("app", "secret")
            .oauth_base_url(format!("http://{addr}"))
            .timeout(Duration::from_millis(300))
            .build()
            .unwrap();
        let clock = clock(120);
        let authen = client.authen();
        let result = if heartbeat {
            authen.oauth.sync_dpop_clock(&clock).await
        } else {
            authen
                .oauth
                .refresh_dpop("old-refresh", &DPoPKey::generate(), &clock)
                .await
                .map(|_| ())
        };
        assert!(matches!(result, Err(LarkError::ClientTimeout(_))));
        assert_eq!(requests.lock().unwrap().len(), 1);
        assert_eq!(clock.now_unix_seconds().unwrap(), LOCAL_TIME);
        drop(release);
        server.abort();
    }
}

#[tokio::test]
async fn accounts_refresh_uses_a_fresh_connection_and_does_not_replay_after_eof() {
    use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
    use tokio::net::{TcpListener, TcpStream};

    async fn read_grant(stream: &mut TcpStream) {
        let mut stream = BufReader::new(stream);
        let mut bytes = 0;
        loop {
            let mut line = String::new();
            assert!(stream.read_line(&mut line).await.unwrap() > 0);
            if line == "\r\n" {
                break;
            }
            if let Some((name, value)) = line.split_once(':')
                && name.eq_ignore_ascii_case("content-length")
            {
                bytes = value.trim().parse().unwrap();
            }
        }
        assert!(bytes > 0);
        stream.read_exact(&mut vec![0; bytes]).await.unwrap();
    }

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut original, _) = listener.accept().await.unwrap();
        read_grant(&mut original).await;
        original
            .write_all(http_response(200, ROTATED_TOKEN).as_bytes())
            .await
            .unwrap();
        let (mut next, _) = listener.accept().await.unwrap();
        read_grant(&mut next).await;
        drop(next);
        assert!(
            tokio::time::timeout(Duration::from_millis(100), listener.accept())
                .await
                .is_err(),
            "received an automatic replay after closing the refresh connection"
        );
        drop(original);
    });

    let client = LarkClient::builder("app", "secret")
        .oauth_base_url(format!("http://{addr}"))
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();
    let authen = client.authen();
    let clock = clock(120);
    let key = DPoPKey::generate();
    authen
        .oauth
        .refresh_dpop("old-refresh", &key, &clock)
        .await
        .unwrap();
    let result = authen.oauth.refresh_dpop("new-refresh", &key, &clock).await;
    assert!(
        result.is_err(),
        "a lost refresh response was treated as success"
    );
    tokio::time::timeout(Duration::from_secs(2), server)
        .await
        .unwrap()
        .unwrap();
}
