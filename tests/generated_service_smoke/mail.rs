use super::*;

// ── Mail ──

#[tokio::test]
async fn mail_mailgroup_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"mailgroup_id":"mg-1","email":"group@example.com"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .mail()
        .mailgroup
        .list_by_query(
            &ListMailMailgroupQuery::new()
                .manager_user_id("ou-1")
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
            .and_then(|mailgroup| mailgroup.mailgroup_id.as_deref()),
        Some("mg-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/mail/v1/mailgroups?"));
    assert!(request.contains("manager_user_id=ou-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn mail_mailgroup_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"mailgroup":{"mailgroup_id":"mg-1","email":"group@example.com","name":"Engineering"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateMailgroupReqBody {
        email: Some("group@example.com".into()),
        name: Some("Engineering".into()),
        description: Some("Team group".into()),
        who_can_send_mail: Some("ANYONE".into()),
    };
    let patch_body = CreateMailgroupReqBody {
        name: Some("Engineering updated".into()),
        ..Default::default()
    };
    let update_body = CreateMailgroupReqBody {
        email: Some("group@example.com".into()),
        name: Some("Engineering".into()),
        ..Default::default()
    };

    client
        .mail()
        .mailgroup
        .create_by_query(
            &CreateMailgroupQuery::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mail()
        .mailgroup
        .get_by_query(&GetMailgroupQuery::new("mg-1"), &RequestOption::default())
        .await
        .unwrap();
    client
        .mail()
        .mailgroup
        .patch_by_query(
            &PatchMailgroupQuery::new("mg-1", &patch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mail()
        .mailgroup
        .update_by_query(
            &UpdateMailgroupQuery::new("mg-1", &update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/mail/v1/mailgroups "));
    assert!(request.contains("GET /open-apis/mail/v1/mailgroups/mg-1 "));
    assert!(request.contains("PATCH /open-apis/mail/v1/mailgroups/mg-1 "));
    assert!(request.contains("PUT /open-apis/mail/v1/mailgroups/mg-1 "));
    assert!(request.contains(r#""email":"group@example.com""#));
    assert!(request.contains(r#""name":"Engineering""#));
    assert!(request.contains(r#""name":"Engineering updated""#));
}

#[tokio::test]
async fn mail_mailgroup_member_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id":"member-1","user_id":"ou-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .mail()
        .mailgroup_member
        .list_by_query(
            &ListMailMailgroupMemberQuery::new("mg-1")
                .user_id_type("open_id")
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
            .and_then(|member| member.member_id.as_deref()),
        Some("member-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/mail/v1/mailgroups/mg-1/members?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn mail_mailgroup_member_by_query_smoke() {
    let member_body = r#"{"code":0,"msg":"ok","data":{"member":{"member_id":"member-1","user_id":"ou-1","email":"user@example.com"}}}"#;
    let batch_body = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id":"member-1"}]}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, member_body),
        http_response(200, batch_body),
        http_response(200, empty_body),
        http_response(200, member_body),
    ])
    .await;

    let client = client_for(addr);
    let member = MailgroupMember {
        member_id: Some("member-1".into()),
        user_id: Some("ou-1".into()),
        email: Some("user@example.com".into()),
        ..Default::default()
    };
    let batch_create_body = BatchCreateMailgroupMemberReqBody {
        items: Some(vec![member.clone()]),
    };
    let batch_delete_body = BatchDeleteMailgroupMemberReqBody {
        member_id_list: Some(vec!["member-1".to_string()]),
    };

    client
        .mail()
        .mailgroup_member
        .create_by_query(
            &CreateMailgroupMemberQuery::new("mg-1", &member).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mail()
        .mailgroup_member
        .batch_create_by_query(
            &BatchCreateMailgroupMemberQuery::new("mg-1", &batch_create_body)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mail()
        .mailgroup_member
        .batch_delete_by_query(
            &BatchDeleteMailgroupMemberQuery::new("mg-1", &batch_delete_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mail()
        .mailgroup_member
        .get_by_query(
            &GetMailgroupMemberQuery::new("mg-1", "member-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/mail/v1/mailgroups/mg-1/members?"));
    assert!(request.contains("POST /open-apis/mail/v1/mailgroups/mg-1/members/batch_create?"));
    assert!(request.contains("DELETE /open-apis/mail/v1/mailgroups/mg-1/members/batch_delete "));
    assert!(request.contains("GET /open-apis/mail/v1/mailgroups/mg-1/members/member-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""member_id":"member-1""#));
    assert!(request.contains(r#""user_id":"ou-1""#));
    assert!(request.contains(r#""member_id_list":["member-1"]"#));
}

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

#[tokio::test]
async fn mail_public_mailbox_member_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id":"member-1","user_id":"ou-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .mail()
        .public_mailbox_member
        .list_by_query(
            &ListMailPublicMailboxMemberQuery::new("pm-1")
                .user_id_type("open_id")
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
            .and_then(|member| member.member_id.as_deref()),
        Some("member-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/mail/v1/public_mailboxes/pm-1/members?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn mail_public_mailbox_member_by_query_smoke() {
    let member_body = r#"{"code":0,"msg":"ok","data":{"member":{"member_id":"member-1","user_id":"ou-1","type_":"USER"}}}"#;
    let batch_body = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id":"member-1"}]}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, empty_body),
        http_response(200, batch_body),
        http_response(200, empty_body),
        http_response(200, member_body),
    ])
    .await;

    let client = client_for(addr);
    let member = PublicMailboxMember {
        member_id: Some("member-1".into()),
        user_id: Some("ou-1".into()),
        type_: Some("USER".into()),
    };
    let batch_create_body = BatchCreatePublicMailboxMemberReqBody {
        items: Some(vec![member.clone()]),
    };
    let batch_delete_body = BatchDeletePublicMailboxMemberReqBody {
        member_id_list: Some(vec!["member-1".to_string()]),
    };

    client
        .mail()
        .public_mailbox_member
        .create_by_query(
            &CreatePublicMailboxMemberQuery::new("pm-1", &member).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mail()
        .public_mailbox_member
        .batch_create_by_query(
            &BatchCreatePublicMailboxMemberQuery::new("pm-1", &batch_create_body)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mail()
        .public_mailbox_member
        .batch_delete_by_query(
            &BatchDeletePublicMailboxMemberQuery::new("pm-1", &batch_delete_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .mail()
        .public_mailbox_member
        .get_by_query(
            &GetPublicMailboxMemberQuery::new("pm-1", "member-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/mail/v1/public_mailboxes/pm-1/members?"));
    assert!(
        request.contains("POST /open-apis/mail/v1/public_mailboxes/pm-1/members/batch_create?")
    );
    assert!(
        request.contains("DELETE /open-apis/mail/v1/public_mailboxes/pm-1/members/batch_delete ")
    );
    assert!(request.contains("GET /open-apis/mail/v1/public_mailboxes/pm-1/members/member-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""member_id":"member-1""#));
    assert!(request.contains(r#""user_id":"ou-1""#));
    assert!(request.contains(r#""member_id_list":["member-1"]"#));
}
