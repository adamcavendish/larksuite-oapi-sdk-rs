use super::prelude::*;

// Drive media smoke tests

#[tokio::test]
async fn drive_media_download_smoke() {
    let body = "media-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"media.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .media
        .download(
            "media-token-1",
            Some("extra-value"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("media.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/medias/media-token-1/download?"));
    assert!(request.contains("extra=extra-value"));
}

#[tokio::test]
async fn drive_media_upload_all_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"file_token":"media-token-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .media
        .upload_all(
            "clip.mp4",
            "explorer",
            "folder-token-1",
            4,
            Some("checksum-1"),
            Some("extra-value"),
            b"clip".to_vec(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().file_token.as_deref(),
        Some("media-token-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/medias/upload_all"));
    assert!(request.contains("name=\"file_name\""));
    assert!(request.contains("clip.mp4"));
    assert!(request.contains("name=\"parent_type\""));
    assert!(request.contains("explorer"));
    assert!(request.contains("name=\"parent_node\""));
    assert!(request.contains("folder-token-1"));
    assert!(request.contains("name=\"size\""));
    assert!(request.contains("\r\n4\r\n"));
    assert!(request.contains("name=\"checksum\""));
    assert!(request.contains("checksum-1"));
    assert!(request.contains("name=\"extra\""));
    assert!(request.contains("extra-value"));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("filename=\"clip.mp4\""));
    assert!(request.contains("clip"));
}

#[tokio::test]
async fn drive_media_upload_part_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .media
        .upload_part(
            "upload-id-1",
            2,
            4,
            Some("checksum-1"),
            b"clip".to_vec(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/medias/upload_part"));
    assert!(request.contains("name=\"upload_id\""));
    assert!(request.contains("upload-id-1"));
    assert!(request.contains("name=\"seq\""));
    assert!(request.contains("\r\n2\r\n"));
    assert!(request.contains("name=\"size\""));
    assert!(request.contains("\r\n4\r\n"));
    assert!(request.contains("name=\"checksum\""));
    assert!(request.contains("checksum-1"));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("filename=\"part\""));
    assert!(request.contains("clip"));
}
