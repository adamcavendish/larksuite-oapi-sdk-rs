use super::prelude::*;

// ── Directory ──

#[tokio::test]
async fn directory_collaboration_rule_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCollaborationRuleQuery::new()
        .target_tenant_key("target-tenant")
        .tenant_id("tenant-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .directory()
        .collaboration_rule
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/directory/v1/collaboration_rules?"));
    assert!(request.contains("target_tenant_key=target-tenant"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn directory_collaboration_rule_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .directory()
        .collaboration_rule
        .list(
            Some("target-tenant"),
            Some("tenant-1"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/directory/v1/collaboration_rules?"));
    assert!(request.contains("target_tenant_key=target-tenant"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn directory_collaboration_rule_write_by_query_smoke() {
    let create_body = r#"{"code":0,"msg":"ok","data":{"collaboration_rule_id":"rule-1"}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, create_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let create_payload = serde_json::json!({"name":"Rule"});
    let update_payload = serde_json::json!({"name":"Rule updated"});
    let create_resp = client
        .directory()
        .collaboration_rule
        .create_by_query(
            &CreateCollaborationRuleQuery::new(&create_payload)
                .target_tenant_key("target-tenant")
                .tenant_id("tenant-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete_resp = client
        .directory()
        .collaboration_rule
        .delete_by_query(
            &DeleteCollaborationRuleQuery::new("rule-1")
                .target_tenant_key("target-tenant")
                .tenant_id("tenant-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_resp = client
        .directory()
        .collaboration_rule
        .update_by_query(
            &UpdateCollaborationRuleQuery::new("rule-1", &update_payload)
                .target_tenant_key("target-tenant")
                .tenant_id("tenant-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(delete_resp.success());
    assert!(update_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/directory/v1/collaboration_rules?"));
    assert!(request.contains("DELETE /open-apis/directory/v1/collaboration_rules/rule-1?"));
    assert!(request.contains("PUT /open-apis/directory/v1/collaboration_rules/rule-1?"));
    assert!(request.contains("target_tenant_key=target-tenant"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains(r#""name":"Rule""#));
    assert!(request.contains(r#""name":"Rule updated""#));
}
