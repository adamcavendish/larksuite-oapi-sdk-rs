use super::prelude::*;

// ── ACS ──

#[tokio::test]
async fn acs_user_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"user":{"user_id":"ou-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .acs()
        .user
        .get_by_query(
            &GetAcsUserQuery::new("ou-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.user.as_ref())
            .and_then(|user| user.user_id.as_deref()),
        Some("ou-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/users/ou-1?user_id_type=open_id"));
}

#[tokio::test]
async fn acs_user_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .acs()
        .user
        .list_by_query(
            &ListAcsUserQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|user| user.user_id.as_deref()),
        Some("ou-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/users?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn acs_user_patch_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = User {
        user_id: Some("ou-1".into()),
        feature: Some(Feature {
            card: Some(7),
            face_uploaded: Some(true),
        }),
    };
    let resp = client
        .acs()
        .user
        .patch_by_query(
            &PatchAcsUserQuery::new("ou-1", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/acs/v1/users/ou-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""card":7"#));
}
