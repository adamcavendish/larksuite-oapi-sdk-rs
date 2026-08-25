use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::card::v1::{Card, CardDocument, Header};
use crate::{LarkClient, RequestOption};

use super::{SendInput, SendTarget, StreamUpdate};

#[derive(Clone)]
struct MockResponse {
    content_type: &'static str,
    extra_headers: String,
    body: &'static [u8],
}

impl MockResponse {
    fn json(body: &'static str) -> Self {
        Self {
            content_type: "application/json",
            extra_headers: String::new(),
            body: body.as_bytes(),
        }
    }

    fn download(file_name: &'static str, body: &'static [u8]) -> Self {
        Self {
            content_type: "application/octet-stream",
            extra_headers: format!("Content-Disposition: attachment; filename=\"{file_name}\"\r\n"),
            body,
        }
    }
}

async fn mock_server(
    responses: Vec<MockResponse>,
) -> (
    std::net::SocketAddr,
    Arc<Mutex<Vec<String>>>,
    tokio::task::JoinHandle<()>,
) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let responses = Arc::new(responses);
    let counter = Arc::new(AtomicUsize::new(0));
    let requests = Arc::new(Mutex::new(Vec::new()));
    let handle = tokio::spawn({
        let requests = Arc::clone(&requests);
        async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    break;
                };
                let responses = Arc::clone(&responses);
                let counter = Arc::clone(&counter);
                let requests = Arc::clone(&requests);
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};

                    let mut buf = vec![0u8; 131_072];
                    let Ok(n) = stream.read(&mut buf).await else {
                        return;
                    };
                    requests
                        .lock()
                        .unwrap()
                        .push(String::from_utf8_lossy(&buf[..n]).to_string());
                    let index = counter.fetch_add(1, Ordering::SeqCst);
                    let response = &responses[index.min(responses.len() - 1)];
                    let head = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\n{}Content-Length: {}\r\nConnection: close\r\n\r\n",
                        response.content_type,
                        response.extra_headers,
                        response.body.len(),
                    );
                    let _ = stream.write_all(head.as_bytes()).await;
                    let _ = stream.write_all(response.body).await;
                    let _ = stream.shutdown().await;
                });
            }
        }
    });
    (addr, requests, handle)
}

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("app_id", "app_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

const MESSAGE_OK: &str =
    r#"{"code":0,"msg":"ok","data":{"message_id":"om_sent","chat_id":"oc_chat"}}"#;

#[tokio::test]
async fn public_send_and_reply_operations_preserve_their_routes() {
    let responses = vec![
        MockResponse::json(MESSAGE_OK),
        MockResponse::json(MESSAGE_OK),
        MockResponse::json(MESSAGE_OK),
        MockResponse::json(MESSAGE_OK),
        MockResponse::json(r#"{"code":230001,"msg":"first target rejected"}"#),
        MockResponse::json(MESSAGE_OK),
        MockResponse::json(MESSAGE_OK),
    ];
    let (addr, requests, _handle) = mock_server(responses).await;
    let client = client_for(addr);
    let messaging = client.channel_messaging();
    let option = RequestOption::default();
    let target = SendTarget::new("chat_id", "oc_chat");

    messaging
        .send_text(&target, "plain", &option)
        .await
        .unwrap();
    messaging
        .send(
            &SendInput {
                chat_id: Some("oc_chat".into()),
                text: Some("high level".into()),
                ..Default::default()
            },
            &option,
        )
        .await
        .unwrap();
    messaging
        .reply(
            "om_parent",
            &SendInput {
                text: Some("strict reply".into()),
                ..Default::default()
            },
            &option,
        )
        .await
        .unwrap();
    messaging
        .reply_in_thread(
            "om_topic",
            &SendInput {
                text: Some("topic reply".into()),
                ..Default::default()
            },
            &option,
        )
        .await
        .unwrap();
    messaging
        .send_text_with_fallback(
            &[
                SendTarget::new("chat_id", "oc_first"),
                SendTarget::new("chat_id", "oc_second"),
            ],
            "fallback",
            &option,
        )
        .await
        .unwrap();
    messaging
        .reply_text("om_text", "text reply", false, &option)
        .await
        .unwrap();

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 7);
    assert!(requests[0].contains("POST /open-apis/im/v1/messages?receive_id_type=chat_id"));
    assert!(requests[2].contains("POST /open-apis/im/v1/messages/om_parent/reply"));
    assert!(requests[3].contains("POST /open-apis/im/v1/messages/om_topic/reply"));
    assert!(requests[3].contains(r#""reply_in_thread":true"#));
    assert!(requests[4].contains(r#""receive_id":"oc_first""#));
    assert!(requests[5].contains(r#""receive_id":"oc_second""#));
    assert!(requests[6].contains(r#""reply_in_thread":false"#));
}

#[tokio::test]
async fn public_update_chunk_and_stream_operations_preserve_their_contracts() {
    let markdown = "first\n\nsecond";
    let chunk_count = super::split_markdown(markdown, 7).len();
    assert!(chunk_count > 1);
    let mut responses = vec![
        MockResponse::json(r#"{"code":0,"msg":"ok","data":{"message_id":"om_text"}}"#),
        MockResponse::json(r#"{"code":0,"msg":"ok","data":{}}"#),
    ];
    responses.extend((0..chunk_count).map(|_| MockResponse::json(MESSAGE_OK)));
    responses.push(MockResponse::json(
        r#"{"code":0,"msg":"ok","data":{"message_id":"om_stream"}}"#,
    ));
    let (addr, requests, _handle) = mock_server(responses).await;
    let client = client_for(addr);
    let messaging = client.channel_messaging();
    let option = RequestOption::default();

    messaging
        .edit_text("om_text", "updated", &option)
        .await
        .unwrap();
    messaging
        .edit_card(
            "om_card",
            &CardDocument::new(Card::new().header(Header::new("updated"))).unwrap(),
            &option,
        )
        .await
        .unwrap();
    let chunks = messaging
        .send_markdown_chunks(&SendTarget::new("chat_id", "oc_chat"), markdown, 7, &option)
        .await
        .unwrap();
    assert_eq!(chunks.len(), chunk_count);

    let mut stream = StreamUpdate::new("om_stream", Duration::ZERO);
    stream.push("streamed");
    assert!(
        messaging
            .flush_stream_text(&mut stream, &option)
            .await
            .unwrap()
            .is_some()
    );
    let requests = requests.lock().unwrap();
    assert!(requests[0].contains("PUT /open-apis/im/v1/messages/om_text"));
    assert!(requests[1].contains("PATCH /open-apis/im/v1/messages/om_card"));
    assert!(requests[2].contains("POST /open-apis/im/v1/messages?receive_id_type=chat_id"));
    assert!(
        requests
            .last()
            .unwrap()
            .contains("PUT /open-apis/im/v1/messages/om_stream")
    );
}

#[tokio::test]
async fn public_upload_and_download_operations_remain_separate() {
    let responses = vec![
        MockResponse::json(r#"{"code":0,"msg":"ok","data":{"image_key":"img_key"}}"#),
        MockResponse::json(r#"{"code":0,"msg":"ok","data":{"file_key":"file_key"}}"#),
        MockResponse::download("resource.bin", b"resource-bytes"),
        MockResponse::download("file.bin", b"file-bytes"),
    ];
    let (addr, requests, _handle) = mock_server(responses).await;
    let client = client_for(addr);
    let messaging = client.channel_messaging();
    let option = RequestOption::default();

    let image = messaging
        .upload_image("message", b"image".to_vec(), &option)
        .await
        .unwrap();
    assert_eq!(image.data.unwrap().image_key.as_deref(), Some("img_key"));
    let file = messaging
        .upload_file("stream", "report.txt", None, b"file".to_vec(), &option)
        .await
        .unwrap();
    assert_eq!(file.data.unwrap().file_key.as_deref(), Some("file_key"));
    let resource = messaging
        .download_message_resource("om_message", "file_key", "image", &option)
        .await
        .unwrap();
    assert_eq!(resource.data, b"resource-bytes");
    assert_eq!(resource.file_name.as_deref(), Some("resource.bin"));
    let file = messaging
        .download_file("om_message", "file_key", "file", &option)
        .await
        .unwrap();
    assert_eq!(file.data, b"file-bytes");
    assert_eq!(file.file_name.as_deref(), Some("file.bin"));

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 4);
    assert!(requests[0].contains("POST /open-apis/im/v1/images"));
    assert!(requests[1].contains("POST /open-apis/im/v1/files"));
    assert!(
        requests[2]
            .contains("GET /open-apis/im/v1/messages/om_message/resources/file_key?type=image")
    );
    assert!(
        requests[3]
            .contains("GET /open-apis/im/v1/messages/om_message/resources/file_key?type=file")
    );
}
