use super::prelude::*;

// ── Bitable ──

#[tokio::test]
async fn bitable_workflow_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"workflows":[{"workflow_id":"flow-1","title":"Notify"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListAppWorkflowQuery::new("app-token-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .bitable()
        .workflow
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.workflows[0].title.as_deref(), Some("Notify"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/workflows?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn bitable_workflow_list_positional_adapter_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"workflows":[{"workflow_id":"flow-1","title":"Notify"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .bitable()
        .workflow
        .list(
            "app-token-1",
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/bitable/v1/apps/app-token-1/workflows?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
