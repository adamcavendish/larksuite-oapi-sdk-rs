use super::prelude::*;

// ── Helpdesk ──

#[tokio::test]
async fn helpdesk_ticket_image_download_smoke() {
    let body = "ticket-image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"ticket-image.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket
        .ticket_image("ticket-1", "msg-1", Some(2), &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("ticket-image.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/ticket_images?"));
    assert!(request.contains("ticket_id=ticket-1"));
    assert!(request.contains("msg_id=msg-1"));
    assert!(request.contains("index=2"));
}

#[tokio::test]
async fn helpdesk_ticket_message_image_download_smoke() {
    let body = "ticket-message-image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"ticket-message-image.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket_message
        .image("ticket-1", "msg-1", Some(3), &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("ticket-message-image.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/ticket_images?"));
    assert!(request.contains("ticket_id=ticket-1"));
    assert!(request.contains("msg_id=msg-1"));
    assert!(request.contains("index=3"));
}

#[tokio::test]
async fn helpdesk_faq_image_download_smoke() {
    let body = "faq-image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"faq-image.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .image("faq-1", "image-key-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("faq-image.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs/faq-1/image/image-key-1"));
}
