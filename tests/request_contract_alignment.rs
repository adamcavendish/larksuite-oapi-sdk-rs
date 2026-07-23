mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::{FormDataBuilder, LarkClient, RequestOption};

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn document_ai_recognition_sends_a_multipart_file() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let response = client_for(addr)
        .document_ai()
        .ai
        .recognize_bank_card(
            FormDataBuilder::new()
                .file(
                    "file",
                    "bank-card.png",
                    b"bank-card-bytes".to_vec(),
                    Some("image/png"),
                )
                .build(),
            &RequestOption {
                tenant_access_token: Some("tenant-contract-token".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assert!(response.success());
    let request = requests.lock().unwrap().join("\n").to_ascii_lowercase();
    assert!(request.contains("post /open-apis/document_ai/v1/bank_card/recognize"));
    assert!(request.contains("content-type: multipart/form-data; boundary="));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("bank-card-bytes"));
    assert!(request.contains("authorization: bearer tenant-contract-token"));
}

#[tokio::test]
async fn calendar_event_subscription_accepts_a_tenant_token() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    client_for(addr)
        .calendar()
        .event
        .subscription(
            "calendar-1",
            &RequestOption {
                tenant_access_token: Some("tenant-contract-token".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n").to_ascii_lowercase();
    assert!(
        request.contains("post /open-apis/calendar/v4/calendars/calendar-1/events/subscription")
    );
    assert!(request.contains("authorization: bearer tenant-contract-token"));
}
