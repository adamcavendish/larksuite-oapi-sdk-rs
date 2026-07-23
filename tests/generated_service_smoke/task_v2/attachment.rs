use super::prelude::*;

// Task v2 attachment smoke tests

#[tokio::test]
async fn task_v2_attachment_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"attachment":{"guid":"att-1","name":"spec.pdf"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetAttachmentV2Query::new("att-1").user_id_type("open_id");
    let resp = client
        .task_v2()
        .attachment
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let attachment = resp.data.unwrap().attachment.unwrap();
    assert_eq!(attachment.guid.as_deref(), Some("att-1"));
    assert_eq!(attachment.name.as_deref(), Some("spec.pdf"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/attachments/att-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_attachment_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"att-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task_v2()
        .attachment
        .list(
            Some("task"),
            Some("task-guid-1"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/attachments?"));
    assert!(request.contains("resource_type=task"));
    assert!(request.contains("resource_id=task-guid-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn task_v2_attachment_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"att-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListAttachmentV2Query::new()
        .resource_type("task")
        .resource_id("task-guid-1")
        .updated_mesc("1700000000000")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .task_v2()
        .attachment
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].guid.as_deref(), Some("att-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/attachments?"));
    assert!(request.contains("resource_type=task"));
    assert!(request.contains("resource_id=task-guid-1"));
    assert!(request.contains("updated_mesc=1700000000000"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn task_v2_attachment_upload_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"attachment":{"guid":"att-1","name":"spec.pdf"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let upload_body = FormDataBuilder::new()
        .file(
            "file",
            "spec.pdf",
            b"spec-bytes".to_vec(),
            Some("application/pdf"),
        )
        .build();
    let resp = client
        .task_v2()
        .attachment
        .upload_by_query(
            &UploadAttachmentV2Query::new(&upload_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/attachments/upload "));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("spec-bytes"));
}
