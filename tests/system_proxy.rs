mod common;

use common::{http_response, mock_server_with_requests};
use larksuite_oapi_sdk_rs::dpop::{DPoPBinding, DPoPKey};
use larksuite_oapi_sdk_rs::{AccessTokenType, ApiReq, LarkClient, RequestOption, registration};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const OPERATIONS: [&str; 3] = ["registration", "openapi", "dpop"];

// Only child processes receive modified environments; parallel tests never
// mutate the process-wide proxy settings.
async fn probe(operation: &str, destination: &str, env: &[(&str, &str)]) {
    let mut command = Command::new(std::env::current_exe().unwrap());
    command.args(["--exact", "proxy_child", "--ignored", "--nocapture"]);
    for name in [
        "HTTP_PROXY",
        "http_proxy",
        "HTTPS_PROXY",
        "https_proxy",
        "ALL_PROXY",
        "all_proxy",
        "NO_PROXY",
        "no_proxy",
    ] {
        command.env_remove(name);
    }
    command.env("SDK_PROXY_TEST_OPERATION", operation);
    command.env("SDK_PROXY_TEST_DESTINATION", destination);
    command.envs(env.iter().copied());
    let output = tokio::task::spawn_blocking(move || command.output().unwrap())
        .await
        .unwrap();
    assert!(
        output.status.success(),
        "{operation} at {destination}: {}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn responses(operation: &str) -> Vec<String> {
    let bodies = match operation {
        "registration" => vec![
            r#"{"device_code":"test-device","verification_uri_complete":"https://example.invalid/scan","interval":1,"expire_in":60}"#,
            r#"{"client_id":"test-app","client_secret":"test-secret"}"#,
        ],
        "openapi" => vec![
            r#"{"code":0,"tenant_access_token":"test-token","expire":7200}"#,
            r#"{"code":0}"#,
        ],
        "dpop" => vec![r#"{"code":0}"#],
        _ => unreachable!(),
    };
    bodies.iter().map(|body| http_response(200, body)).collect()
}

#[tokio::test]
async fn http_proxy_routes_registration_tokens_and_api_requests() {
    for operation in OPERATIONS {
        for key in ["HTTP_PROXY", "http_proxy"] {
            let replies = responses(operation);
            let expected_requests = replies.len();
            let (origin, server, requests) = mock_server_with_requests(replies).await;
            let proxy = Proxy::start(Some(origin)).await;
            probe(
                operation,
                "http://sdk-proxy-test.invalid",
                &[(key, &proxy.url)],
            )
            .await;
            let requests = requests.lock().unwrap();
            assert_eq!(requests.len(), expected_requests);
            if operation == "dpop" {
                assert!(requests[0].to_ascii_lowercase().contains("\r\ndpop:"));
            }
            let tunnels = proxy.requests.lock().unwrap();
            assert!(!tunnels.is_empty());
            assert!(
                tunnels
                    .iter()
                    .all(|line| line == "CONNECT sdk-proxy-test.invalid:80 HTTP/1.1")
            );
            server.abort();
        }
    }
}

#[tokio::test]
async fn https_proxy_uses_connect_for_all_http_clients() {
    for operation in OPERATIONS {
        for key in ["HTTPS_PROXY", "https_proxy"] {
            // Reject after recording CONNECT; no public endpoint or test CA is needed.
            let proxy = Proxy::start(None).await;
            probe(
                operation,
                "https://sdk-proxy-test.invalid",
                &[(key, &proxy.url)],
            )
            .await;
            let tunnels = proxy.requests.lock().unwrap();
            assert!(!tunnels.is_empty(), "{operation} ignored {key}");
            assert!(
                tunnels
                    .iter()
                    .all(|line| line == "CONNECT sdk-proxy-test.invalid:443 HTTP/1.1")
            );
        }
    }
}

#[tokio::test]
async fn no_proxy_bypasses_all_http_clients() {
    for operation in OPERATIONS {
        for (proxy_key, bypass_key) in [("HTTP_PROXY", "NO_PROXY"), ("http_proxy", "no_proxy")] {
            let replies = responses(operation);
            let expected_requests = replies.len();
            let (origin, server, requests) = mock_server_with_requests(replies).await;
            let proxy = Proxy::start(None).await;
            probe(
                operation,
                &format!("http://{origin}"),
                &[(proxy_key, &proxy.url), (bypass_key, "127.0.0.1")],
            )
            .await;
            assert_eq!(requests.lock().unwrap().len(), expected_requests);
            assert!(proxy.requests.lock().unwrap().is_empty());
            server.abort();
        }
    }
}

#[tokio::test]
async fn requests_remain_direct_without_proxy_environment() {
    for operation in OPERATIONS {
        let replies = responses(operation);
        let expected_requests = replies.len();
        let (origin, server, requests) = mock_server_with_requests(replies).await;
        probe(operation, &format!("http://{origin}"), &[]).await;
        assert_eq!(requests.lock().unwrap().len(), expected_requests);
        server.abort();
    }
}

struct Proxy {
    url: String,
    requests: Arc<Mutex<Vec<String>>>,
    task: tokio::task::JoinHandle<()>,
}

impl Proxy {
    async fn start(origin: Option<std::net::SocketAddr>) -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let requests = Arc::new(Mutex::new(Vec::new()));
        let captured = requests.clone();
        let task = tokio::spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                let captured = captured.clone();
                tokio::spawn(async move {
                    let mut headers = Vec::new();
                    while !headers.ends_with(b"\r\n\r\n") {
                        assert!(headers.len() < 8192);
                        headers.push(socket.read_u8().await.unwrap());
                    }
                    let headers = String::from_utf8(headers).unwrap();
                    captured
                        .lock()
                        .unwrap()
                        .push(headers.lines().next().unwrap().to_owned());
                    if let Some(origin) = origin {
                        let mut upstream = tokio::net::TcpStream::connect(origin).await.unwrap();
                        socket
                            .write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n")
                            .await
                            .unwrap();
                        let _ = tokio::io::copy_bidirectional(&mut socket, &mut upstream).await;
                    } else {
                        socket.write_all(b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").await.unwrap();
                    }
                });
            }
        });
        Self {
            url,
            requests,
            task,
        }
    }
}

impl Drop for Proxy {
    fn drop(&mut self) {
        self.task.abort();
    }
}

#[tokio::test]
#[ignore = "subprocess helper with isolated proxy environment"]
async fn proxy_child() {
    let operation = std::env::var("SDK_PROXY_TEST_OPERATION").unwrap();
    let destination = std::env::var("SDK_PROXY_TEST_DESTINATION").unwrap();
    let result = tokio::time::timeout(Duration::from_secs(10), async {
        if operation == "registration" {
            registration::register_app(registration::Options {
                source: "proxy-test".into(),
                domain: destination.clone(),
                lark_domain: destination.clone(),
                app_preset: None,
                addons: None,
                create_only: false,
                app_id: String::new(),
                on_qr_code: Box::new(|_| {}),
                on_status_change: None,
            })
            .await
            .map(|result| assert_eq!(result.client_id, "test-app"))
        } else {
            let mut builder = LarkClient::builder("test-app", "test-secret")
                .base_url(&destination)
                .max_retries(1);
            let (token_type, option) = if operation == "dpop" {
                // A resolver makes the builder create a separate DPoP HTTP client.
                builder = builder.platform_url_resolver(Arc::new(|url: &str| Ok(url.to_owned())));
                (
                    AccessTokenType::User,
                    RequestOption {
                        user_access_token: Some("test-token".into()),
                        dpop_binding: Some(
                            DPoPBinding::new("test-token", DPoPKey::generate()).unwrap(),
                        ),
                        ..Default::default()
                    },
                )
            } else {
                (AccessTokenType::Tenant, RequestOption::default())
            };
            builder
                .build()
                .unwrap()
                .raw_request_with_token(
                    ApiReq::new(http::Method::GET, "/open-apis/test"),
                    token_type,
                    &option,
                )
                .await
                .map(|response| assert_eq!(response.status_code, 200))
        }
    })
    .await
    .expect("proxy probe timed out");
    if destination.starts_with("https://") {
        assert!(result.is_err(), "the fixture rejects HTTPS tunnels");
    } else {
        result.unwrap();
    }
}
