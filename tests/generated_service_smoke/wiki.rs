use super::*;

// ── Wiki v2 ──

#[tokio::test]
async fn wiki_v1_node_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"node_token":"node-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let request_body = serde_json::json!({"query":"roadmap"});
    let resp = client
        .wiki_v1()
        .node
        .search_by_query(
            &SearchNodeV1Query::new(&request_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|node| node.get("node_token"))
            .and_then(serde_json::Value::as_str),
        Some("node-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/wiki/v1/nodes/search "));
    assert!(request.contains(r#""query":"roadmap""#));
}

#[tokio::test]
async fn wiki_get_space_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"space":{"name":"Team Wiki","space_id":"sp-1","description":"docs"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .wiki_v2()
        .space
        .get("sp-1", None, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let space = resp.data.unwrap().space.unwrap();
    assert_eq!(space.name.as_deref(), Some("Team Wiki"));
    assert_eq!(space.space_id.as_deref(), Some("sp-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/wiki/v2/spaces/sp-1"));
}
