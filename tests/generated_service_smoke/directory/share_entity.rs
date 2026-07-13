use super::prelude::*;

// ── Directory ──

#[tokio::test]
async fn directory_share_entity_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListShareEntityQuery::new()
        .target_tenant_key("target-tenant")
        .target_department_id("department-1")
        .target_group_id("group-1")
        .is_select_subject(true)
        .tenant_id("tenant-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .directory()
        .share_entity
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/directory/v1/share_entities?"));
    assert!(request.contains("target_tenant_key=target-tenant"));
    assert!(request.contains("target_department_id=department-1"));
    assert!(request.contains("target_group_id=group-1"));
    assert!(request.contains("is_select_subject=true"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn directory_share_entity_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .directory()
        .share_entity
        .list(
            Some("target-tenant"),
            Some("department-1"),
            Some("group-1"),
            Some(true),
            Some("tenant-1"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/directory/v1/share_entities?"));
    assert!(request.contains("target_tenant_key=target-tenant"));
    assert!(request.contains("target_department_id=department-1"));
    assert!(request.contains("target_group_id=group-1"));
    assert!(request.contains("is_select_subject=true"));
    assert!(request.contains("tenant_id=tenant-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
