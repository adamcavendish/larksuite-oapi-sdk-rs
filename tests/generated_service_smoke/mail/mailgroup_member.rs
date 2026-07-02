use super::prelude::*;

// ── Mail ──

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
