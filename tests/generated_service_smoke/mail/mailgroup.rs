use super::prelude::*;

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
