use super::*;

// ── EHR ──

#[tokio::test]
async fn ehr_attachment_get_download_smoke() {
    let body = "ehr-attachment-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"attachment.pdf\"\r\nContent-Type: application/pdf\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .ehr()
        .attachment
        .get("attachment-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("attachment.pdf"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/ehr/v1/attachments/attachment-token-1"));
}

#[tokio::test]
async fn ehr_employee_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let statuses = [2, 4];
    let resp = client
        .ehr()
        .employee
        .list_by_query(
            &ListEhrEmployeeQuery::new()
                .user_id_type("open_id")
                .view("basic")
                .status(&statuses[..])
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|employee| employee.user_id.as_deref()),
        Some("ou-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/ehr/v1/employees?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("view=basic"));
    assert!(request.contains("status=2"));
    assert!(request.contains("status=4"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
