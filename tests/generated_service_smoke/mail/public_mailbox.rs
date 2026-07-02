use super::prelude::*;

// ── Mail ──

#[tokio::test]
async fn mail_public_mailbox_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"public_mailbox_id":"pm-1","email":"public@example.com"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .mail()
        .public_mailbox
        .list_by_query(
            &ListMailPublicMailboxQuery::new()
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
            .and_then(|mailbox| mailbox.public_mailbox_id.as_deref()),
        Some("pm-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/mail/v1/public_mailboxes?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn mail_public_mailbox_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"public_mailbox":{"public_mailbox_id":"pm-1","email":"public@example.com","name":"Support"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreatePublicMailboxReqBody {
        email: Some("public@example.com".into()),
        name: Some("Support".into()),
        geo: Some("cn".into()),
    };
    let patch_body = CreatePublicMailboxReqBody {
        name: Some("Support updated".into()),
        ..Default::default()
    };
    let update_body = CreatePublicMailboxReqBody {
        email: Some("public@example.com".into()),
        name: Some("Support".into()),
        geo: Some("cn".into()),
    };

    client
        .mail()
        .public_mailbox
        .create_by_query(
            &CreatePublicMailboxQuery::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mail()
        .public_mailbox
        .get_by_query(
            &GetPublicMailboxQuery::new("pm-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mail()
        .public_mailbox
        .patch_by_query(
            &PatchPublicMailboxQuery::new("pm-1", &patch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mail()
        .public_mailbox
        .update_by_query(
            &UpdatePublicMailboxQuery::new("pm-1", &update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/mail/v1/public_mailboxes "));
    assert!(request.contains("GET /open-apis/mail/v1/public_mailboxes/pm-1 "));
    assert!(request.contains("PATCH /open-apis/mail/v1/public_mailboxes/pm-1 "));
    assert!(request.contains("PUT /open-apis/mail/v1/public_mailboxes/pm-1 "));
    assert!(request.contains(r#""email":"public@example.com""#));
    assert!(request.contains(r#""name":"Support""#));
    assert!(request.contains(r#""name":"Support updated""#));
}
