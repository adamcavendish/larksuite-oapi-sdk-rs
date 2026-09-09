use super::prelude::*;

// ── Wiki v2 ──

#[tokio::test]
async fn wiki_v1_node_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"node_token":"node-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let request_body = json_value!({"query":"roadmap"});
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

#[tokio::test]
async fn wiki_node_by_token_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"node":{"node_token":"wik_1","obj_token":"doxcn_1","obj_type":"docx"}}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let user_option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    };
    let tenant_option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..RequestOption::default()
    };

    let user_response = client
        .wiki_v2()
        .space
        .node_by_token("wik token/1", &user_option)
        .await
        .unwrap();
    let tenant_response = client
        .wiki_v2()
        .space
        .node_by_token("wik token/1", &tenant_option)
        .await
        .unwrap();

    assert!(user_response.success());
    assert!(tenant_response.success());
    assert_eq!(
        user_response
            .data
            .as_ref()
            .and_then(|data| data.node.as_ref())
            .and_then(|node| node.obj_type.as_deref()),
        Some("docx")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/wiki/v2/spaces/node_by_token?token=wik+token%2F1"));
    assert!(!request.contains("obj_type="));
    assert!(request.contains("authorization: Bearer user-token"));
    assert!(request.contains("authorization: Bearer tenant-token"));
}
