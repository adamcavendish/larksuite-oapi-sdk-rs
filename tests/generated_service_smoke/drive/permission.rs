use super::prelude::*;

// Drive permission smoke tests

#[tokio::test]
async fn drive_permission_member_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id":"ou-1","member_type":"user","perm":"view"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .permission_member
        .list_by_query(
            &ListPermissionMemberQuery::new("file-token-1", "doc")
                .fields("member_id,perm")
                .perm_type("container"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|member| member.member_id.as_deref()),
        Some("ou-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/permissions/file-token-1/members?"));
    assert!(request.contains("type=doc"));
    assert!(request.contains("fields=member_id%2Cperm"));
    assert!(request.contains("perm_type=container"));
}

#[tokio::test]
async fn drive_permission_public_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"permission_public":{"external_access":true,"share_entity":"tenant_readable"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .permission_public
        .get_by_query(
            &GetDrivePermissionPublicQuery::new("file-token-1", "doc"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.permission_public.as_ref())
            .and_then(|permission| permission.external_access),
        Some(true)
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/permissions/file-token-1/public?"));
    assert!(request.contains("type=doc"));
}

#[tokio::test]
async fn drive_v2_permission_public_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"external_access_entity":"anyone","share_entity":"tenant_readable"}}"#;
    let patch_body =
        r#"{"code":0,"msg":"ok","data":{"external_access_entity":"tenant","lock_switch":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, patch_body),
    ])
    .await;

    let client = client_for(addr);
    let resp = client
        .drive_v2()
        .permission_public
        .get_by_query(
            &GetDrivePermissionPublicV2Query::new("file-token-1", "doc"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|permission| permission.external_access_entity.as_deref()),
        Some("anyone")
    );
    let update_body = PatchPermissionPublicV2ReqBody {
        external_access_entity: Some("tenant".into()),
        lock_switch: Some(true),
        ..Default::default()
    };
    let patch_resp = client
        .drive_v2()
        .permission_public
        .patch_by_query(
            &PatchDrivePermissionPublicV2Query::new("file-token-1", "doc", &update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(patch_resp.success());
    assert_eq!(
        patch_resp
            .data
            .as_ref()
            .and_then(|permission| permission.lock_switch),
        Some(true)
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v2/permissions/file-token-1/public?"));
    assert!(request.contains("PATCH /open-apis/drive/v2/permissions/file-token-1/public?"));
    assert!(request.contains("type=doc"));
    assert!(request.contains(r#""external_access_entity":"tenant""#));
    assert!(request.contains(r#""lock_switch":true"#));
}
