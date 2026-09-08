use super::prelude::*;
use larksuite_oapi_sdk_rs::service::mail::v1::{
    BatchModifyUserMailboxThreadReqBody, BatchTrashUserMailboxThreadReqBody,
    GetUserMailboxThreadQuery, ListUserMailboxThreadQuery,
};

#[tokio::test]
async fn mail_user_mailbox_thread_contract_smoke() {
    let list_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"th-1","body_preview":"Latest update"}],"page_token":"next","has_more":true}}"#;
    let get_body = r#"{"code":0,"msg":"ok","data":{"thread":{"id":"th/read","messages":[{"message_id":"msg-1","subject":"Status"}]}}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, list_body),
        http_response(200, get_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let user_option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    };
    let tenant_option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..RequestOption::default()
    };
    let mailbox = "alice+sdk@example.com";

    let list = client
        .mail()
        .user_mailbox_thread
        .list(
            &ListUserMailboxThreadQuery::new(mailbox)
                .page(PageQuery::new().page_size(20).page_token("next page"))
                .folder_id("folder id")
                .only_unread(true)
                .label_id("FLAGGED"),
            &user_option,
        )
        .await
        .unwrap();
    let get = client
        .mail()
        .user_mailbox_thread
        .get(
            &GetUserMailboxThreadQuery::new(mailbox, "th/read")
                .format("plain_text_full")
                .include_spam_trash(true),
            &tenant_option,
        )
        .await
        .unwrap();
    client
        .mail()
        .user_mailbox_thread
        .batch_modify(
            mailbox,
            &BatchModifyUserMailboxThreadReqBody {
                thread_ids: vec!["th-1".into(), "th-2".into()],
                add_label_ids: Some(vec![]),
                remove_label_ids: None,
                add_folder: None,
            },
            &user_option,
        )
        .await
        .unwrap();
    client
        .mail()
        .user_mailbox_thread
        .batch_trash(
            mailbox,
            &BatchTrashUserMailboxThreadReqBody {
                thread_ids: vec!["th-3".into()],
            },
            &tenant_option,
        )
        .await
        .unwrap();

    assert!(list.success());
    assert_eq!(
        list.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|thread| thread.id.as_deref()),
        Some("th-1")
    );
    assert_eq!(
        get.data
            .as_ref()
            .and_then(|data| data.thread.as_ref())
            .and_then(|thread| thread.messages.first())
            .and_then(|message| message.message_id.as_deref()),
        Some("msg-1")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("GET /open-apis/mail/v1/user_mailboxes/alice+sdk@example.com/threads?")
    );
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next+page"));
    assert!(request.contains("folder_id=folder+id"));
    assert!(request.contains("only_unread=true"));
    assert!(request.contains("label_id=FLAGGED"));
    assert!(request.contains(
        "GET /open-apis/mail/v1/user_mailboxes/alice+sdk@example.com/threads/th%2Fread?format=plain_text_full&include_spam_trash=true"
    ));
    assert!(request.contains(
        "POST /open-apis/mail/v1/user_mailboxes/alice+sdk@example.com/threads/batch_modify "
    ));
    assert!(request.contains(
        "POST /open-apis/mail/v1/user_mailboxes/alice+sdk@example.com/threads/batch_trash "
    ));
    assert!(request.contains(r#""thread_ids":["th-1","th-2"]"#));
    assert!(request.contains(r#""add_label_ids":[]"#));
    assert!(!request.contains("remove_label_ids"));
    assert!(!request.contains("add_folder"));
    assert!(request.contains(r#""thread_ids":["th-3"]"#));
    assert!(request.contains("authorization: Bearer user-token"));
    assert!(request.contains("authorization: Bearer tenant-token"));
}
