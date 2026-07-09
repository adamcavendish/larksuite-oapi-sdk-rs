use super::prelude::*;
use std::time::Duration;

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::service::common::{DownloadBody, DownloadStreamResp};

// ── IM ──

fn message_resource_download_query() -> GetMessageResourceDownloadQuery<'static> {
    GetMessageResourceDownloadQuery::new("msg-1", "file-key-1", "image")
}

async fn open_message_resource_stream(client: &Client) -> DownloadStreamResp {
    let query = message_resource_download_query();
    tokio::time::timeout(
        Duration::from_secs(1),
        client
            .im()
            .message_resource
            .get_stream_by_query(&query, &RequestOption::default()),
    )
    .await
    .unwrap()
    .unwrap()
}

async fn next_body_chunk(body: &mut DownloadBody) -> bytes::Bytes {
    tokio::time::timeout(Duration::from_secs(1), body.next_chunk())
        .await
        .unwrap()
        .unwrap()
        .unwrap()
}

async fn collect_body(body: &mut DownloadBody) -> Vec<u8> {
    let mut data = Vec::new();
    while let Some(chunk) = body.next_chunk().await.unwrap() {
        data.extend_from_slice(&chunk);
    }
    data
}

#[tokio::test]
async fn im_message_resource_download_smoke() {
    let body = "resource-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"resource.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .im()
        .message_resource
        .get("msg-1", "file-key-1", "image", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("resource.bin"));
    assert_eq!(resp.data, body.as_bytes());
    assert_eq!(resp.api_resp.raw_body, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/messages/msg-1/resources/file-key-1?"));
    assert!(request.contains("type=image"));
}

#[tokio::test]
async fn im_message_resource_stream_download_smoke() {
    let body = "resource-bytes";
    let (addr, _handle, _requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"resource-stream.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let mut resp = open_message_resource_stream(&client).await;

    assert_eq!(resp.file_name.as_deref(), Some("resource-stream.bin"));
    assert_eq!(resp.content_length, Some(body.len() as u64));
    assert_eq!(resp.api_resp.status_code, 200);
    assert!(resp.api_resp.raw_body.is_empty());

    let data = collect_body(&mut resp.body).await;

    assert_eq!(data, body.as_bytes());
}

#[tokio::test]
async fn im_message_resource_stream_by_query_download_smoke() {
    let body = "resource-bytes";
    let (addr, _handle, _requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"resource-stream-query.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let query = message_resource_download_query();
    let mut resp = client
        .im()
        .message_resource
        .get_stream_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("resource-stream-query.bin"));
    assert_eq!(collect_body(&mut resp.body).await, body.as_bytes());
}

#[tokio::test]
async fn im_message_resource_stream_yields_multiple_chunks() {
    let (addr, _handle, requests, release) = mock_server_with_gated_chunked_body(
        200,
        "Content-Disposition: attachment; filename=\"resource-stream.bin\"\r\nContent-Type: application/octet-stream\r\n",
        "resource-",
        vec!["stream-", "bytes"],
    )
    .await;

    let client = client_for(addr);
    let mut resp = open_message_resource_stream(&client).await;

    assert_eq!(resp.file_name.as_deref(), Some("resource-stream.bin"));
    assert_eq!(resp.content_length, None);
    assert_eq!(resp.api_resp.status_code, 200);
    assert!(resp.api_resp.raw_body.is_empty());

    let first = next_body_chunk(&mut resp.body).await;
    assert_eq!(&first[..], b"resource-");

    release.send(()).unwrap();

    let mut data = first.to_vec();
    data.extend_from_slice(&collect_body(&mut resp.body).await);

    assert_eq!(data, b"resource-stream-bytes");

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/messages/msg-1/resources/file-key-1?"));
    assert!(request.contains("type=image"));
}

#[tokio::test]
async fn im_message_resource_stream_retries_non_success_json_token_error() {
    let err_body = r#"{"code":99991663,"msg":"invalid tenant access token"}"#;
    let body = "resource-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(401, err_body),
        http_response_with_headers(
            200,
            "Content-Disposition: attachment; filename=\"resource-stream.bin\"\r\nContent-Type: application/octet-stream\r\n",
            body,
        ),
    ])
    .await;

    let client = client_for(addr);
    let mut resp = open_message_resource_stream(&client).await;

    assert_eq!(resp.file_name.as_deref(), Some("resource-stream.bin"));
    assert_eq!(resp.content_length, Some(body.len() as u64));
    assert_eq!(resp.api_resp.status_code, 200);
    assert_eq!(requests.lock().unwrap().len(), 2);
    assert_eq!(collect_body(&mut resp.body).await, body.as_bytes());
}

#[tokio::test]
async fn im_message_resource_stream_replays_non_success_json_body() {
    let body = r#"{"code":123,"msg":"bad request"}"#;
    let (addr, _handle, _requests) =
        mock_server_with_requests(vec![http_response(400, body)]).await;

    let client = client_for(addr);
    let mut resp = open_message_resource_stream(&client).await;

    assert_eq!(resp.file_name, None);
    assert_eq!(resp.content_length, Some(body.len() as u64));
    assert_eq!(resp.api_resp.status_code, 400);
    assert_eq!(resp.api_resp.raw_body, body.as_bytes());
    assert_eq!(collect_body(&mut resp.body).await, body.as_bytes());
}

#[tokio::test]
async fn im_message_resource_stream_json_without_disposition_does_not_prebuffer() {
    let (addr, _handle, _requests, release) = mock_server_with_gated_chunked_body(
        200,
        "Content-Type: application/json\r\n",
        "{\"partial\":",
        vec!["\"body\"}"],
    )
    .await;

    let client = client_for(addr);
    let mut resp = open_message_resource_stream(&client).await;

    assert_eq!(resp.file_name, None);
    assert_eq!(resp.content_length, None);
    assert!(resp.api_resp.raw_body.is_empty());

    let first = next_body_chunk(&mut resp.body).await;
    assert_eq!(&first[..], b"{\"partial\":");

    release.send(()).unwrap();

    let mut data = first.to_vec();
    data.extend_from_slice(&collect_body(&mut resp.body).await);
    assert_eq!(data, br#"{"partial":"body"}"#);
}

#[tokio::test]
async fn im_message_resource_stream_json_file_with_disposition_does_not_prebuffer() {
    let (addr, _handle, _requests, release) = mock_server_with_gated_body(
        200,
        "Content-Disposition: attachment; filename=\"resource.json\"\r\nContent-Type: application/json\r\n",
        "{\"partial\":",
        vec!["\"body\"}"],
    )
    .await;

    let client = client_for(addr);
    let mut resp = open_message_resource_stream(&client).await;

    assert_eq!(resp.file_name.as_deref(), Some("resource.json"));
    assert_eq!(
        resp.content_length,
        Some(br#"{"partial":"body"}"#.len() as u64)
    );
    assert!(resp.api_resp.raw_body.is_empty());

    let first = next_body_chunk(&mut resp.body).await;
    assert_eq!(&first[..], b"{\"partial\":");

    release.send(()).unwrap();

    let mut data = first.to_vec();
    data.extend_from_slice(&collect_body(&mut resp.body).await);
    assert_eq!(data, br#"{"partial":"body"}"#);
}

#[tokio::test]
async fn im_file_download_smoke() {
    let body = "file-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"im-file.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .im()
        .file
        .get("file-key-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("im-file.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/files/file-key-1"));
}

#[tokio::test]
async fn im_image_download_smoke() {
    let body = "image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"image.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .im()
        .image
        .get("image-key-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("image.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/im/v1/images/image-key-1"));
}
