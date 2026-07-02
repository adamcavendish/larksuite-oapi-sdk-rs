use super::*;

// ── Board ──

#[tokio::test]
async fn board_whiteboard_download_as_image_smoke() {
    let body = "whiteboard-image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"whiteboard.png\"\r\nContent-Type: image/png\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .board()
        .whiteboard
        .download_as_image("whiteboard-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("whiteboard.png"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/board/v1/whiteboards/whiteboard-1/download_as_image"));
}

#[tokio::test]
async fn board_whiteboard_by_query_smoke() {
    let board_body = r#"{"code":0,"msg":"ok","data":{"whiteboard":{"whiteboard_id":"whiteboard-1","title":"Roadmap"}}}"#;
    let theme_body = r#"{"code":0,"msg":"ok","data":{"theme":{"background":"grid"}}}"#;
    let update_theme_body = r#"{"code":0,"msg":"ok","data":{"updated":true}}"#;
    let download_body = "whiteboard-image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, board_body),
        http_response(200, board_body),
        http_response_with_headers(
            200,
            "Content-Disposition: attachment; filename=\"whiteboard.png\"\r\nContent-Type: image/png\r\n",
            download_body,
        ),
        http_response(200, theme_body),
        http_response(200, update_theme_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateBoardReqBody {
        title: Some("Roadmap".into()),
        folder_token: Some("folder-1".into()),
    };
    let update_theme_body = UpdateThemeWhiteboardReqBody {
        theme: Some(serde_json::json!({"background":"grid"})),
    };

    client
        .board()
        .whiteboard
        .create_by_query(
            &CreateWhiteboardQuery::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .board()
        .whiteboard
        .get_by_query(
            &GetWhiteboardQuery::new("whiteboard-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let download = client
        .board()
        .whiteboard
        .download_as_image_by_query(
            &DownloadAsImageWhiteboardQuery::new("whiteboard-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .board()
        .whiteboard
        .theme_by_query(
            &ThemeWhiteboardQuery::new("whiteboard-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .board()
        .whiteboard
        .update_theme_by_query(
            &UpdateThemeWhiteboardQuery::new("whiteboard-1", &update_theme_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(download.file_name.as_deref(), Some("whiteboard.png"));
    assert_eq!(download.data, download_body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/board/v1/whiteboards "));
    assert!(request.contains("GET /open-apis/board/v1/whiteboards/whiteboard-1 "));
    assert!(
        request.contains("GET /open-apis/board/v1/whiteboards/whiteboard-1/download_as_image ")
    );
    assert!(request.contains("GET /open-apis/board/v1/whiteboards/whiteboard-1/theme "));
    assert!(request.contains("POST /open-apis/board/v1/whiteboards/whiteboard-1/update_theme "));
    assert!(request.contains(r#""title":"Roadmap""#));
    assert!(request.contains(r#""folder_token":"folder-1""#));
    assert!(request.contains(r#""background":"grid""#));
}

#[tokio::test]
async fn board_whiteboard_node_by_query_smoke() {
    let node_body = r#"{"code":0,"msg":"ok","data":{"node":{"id":"node-1"}}}"#;
    let list_body = r#"{"code":0,"msg":"ok","data":{"nodes":[{"id":"node-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, node_body),
        http_response(200, node_body),
        http_response(200, list_body),
    ])
    .await;

    let client = client_for(addr);
    let node_body = serde_json::json!({"shape":"rectangle"});
    let plantuml_body = CreatePlantumlWhiteboardNodeReqBody {
        content: Some("@startuml\nAlice -> Bob\n@enduml".into()),
        position: Some(serde_json::json!({"x":1,"y":2})),
    };

    client
        .board()
        .whiteboard_node
        .create_by_query(
            &CreateWhiteboardNodeQuery::new("whiteboard-1", &node_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .board()
        .whiteboard_node
        .create_plantuml_by_query(
            &CreatePlantumlWhiteboardNodeQuery::new("whiteboard-1", &plantuml_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .board()
        .whiteboard_node
        .list_by_query(
            &ListWhiteboardNodeQuery::new("whiteboard-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/board/v1/whiteboards/whiteboard-1/nodes "));
    assert!(request.contains("POST /open-apis/board/v1/whiteboards/whiteboard-1/nodes/plantuml "));
    assert!(request.contains("GET /open-apis/board/v1/whiteboards/whiteboard-1/nodes "));
    assert!(request.contains(r#""shape":"rectangle""#));
    assert!(request.contains(r#""content":"@startuml\nAlice -> Bob\n@enduml""#));
    assert!(request.contains(r#""x":1"#));
}
