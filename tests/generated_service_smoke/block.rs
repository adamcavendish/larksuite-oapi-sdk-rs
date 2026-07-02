use super::*;

// ── Block ──

#[tokio::test]
async fn block_v1_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"block":{"block_id":"block-1","title":"Summary"}}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let create_body = CreateBlockV1ReqBody {
        block_type_id: Some("type-1".into()),
        title: Some("Summary".into()),
        summary: Some("Daily summary".into()),
        ..Default::default()
    };

    client
        .block()
        .block
        .create_by_query(
            &CreateBlockV1Query::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .block()
        .block
        .get_by_query(&GetBlockV1Query::new("block-1"), &RequestOption::default())
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/block/v2/blocks "));
    assert!(request.contains("GET /open-apis/block/v2/blocks/block-1 "));
    assert!(request.contains(r#""block_type_id":"type-1""#));
    assert!(request.contains(r#""title":"Summary""#));
    assert!(request.contains(r#""summary":"Daily summary""#));
}

#[tokio::test]
async fn block_v2_by_query_smoke() {
    let entity_body = r#"{"code":0,"msg":"ok","data":{"entity":{"block_id":"block-1"}}}"#;
    let message_body = r#"{"code":0,"msg":"ok","data":{"message":{"id":"message-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, entity_body),
        http_response(200, entity_body),
        http_response(200, message_body),
    ])
    .await;

    let client = client_for(addr);
    let entity_body = serde_json::json!({"entity":{"name":"Entity"}});
    let update_body = serde_json::json!({"entity":{"name":"Entity updated"}});
    let message_body = serde_json::json!({"message":{"text":"hello"}});

    client
        .block_v2()
        .entity
        .create_by_query(
            &CreateBlockEntityQuery::new(&entity_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .block_v2()
        .entity
        .update_by_query(
            &UpdateBlockEntityQuery::new("block-1", &update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .block_v2()
        .message
        .create_by_query(
            &CreateBlockMessageQuery::new(&message_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/block/v2/entities "));
    assert!(request.contains("PUT /open-apis/block/v2/entities/block-1 "));
    assert!(request.contains("POST /open-apis/block/v2/message "));
    assert!(request.contains(r#""name":"Entity""#));
    assert!(request.contains(r#""name":"Entity updated""#));
    assert!(request.contains(r#""text":"hello""#));
}
