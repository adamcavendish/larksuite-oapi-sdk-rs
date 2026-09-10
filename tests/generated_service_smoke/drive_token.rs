use super::prelude::*;

#[tokio::test]
async fn drive_token_lookup_contract() {
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, r#"{"code":0,"msg":"ok","data":{"obj_token":"doc-1","obj_type":"docx","is_wiki_token":true}}"#),
        http_response(200, r#"{"code":0,"msg":"ok","data":{"obj_token":"file-1","obj_type":"file","is_wiki_token":false}}"#),
        http_response(200, r#"{"code":99991672,"msg":"permission denied"}"#),
    ]).await;
    let client = client_for(addr);
    for (user, token, object, kind) in [
        (true, "user-token", "doc-1", "docx"),
        (false, "tenant-token", "file-1", "file"),
    ] {
        let option = if user {
            RequestOption {
                user_access_token: Some(token.into()),
                ..Default::default()
            }
        } else {
            RequestOption {
                tenant_access_token: Some(token.into()),
                ..Default::default()
            }
        };
        let response = client
            .drive_v2()
            .file
            .query_by_token("node /?&+", &option)
            .await
            .unwrap();
        assert!(response.success());
        let data = response.data.unwrap();
        assert_eq!(data.obj_token.as_deref(), Some(object));
        assert_eq!(data.obj_type.as_deref(), Some(kind));
        assert_eq!(data.is_wiki_token, Some(user));
    }
    let response = client
        .drive_v2()
        .file
        .query_by_token(
            "node /?&+",
            &RequestOption {
                tenant_access_token: Some("tenant-token".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    assert!(
        matches!(response, larksuite_oapi_sdk_rs::LarkError::Api(error) if error.code == 99991672)
    );
    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    for (request, token) in requests
        .iter()
        .zip(["user-token", "tenant-token", "tenant-token"])
    {
        let (headers, body) = request.split_once("\r\n\r\n").unwrap();
        assert_eq!(
            headers.lines().next(),
            Some("GET /open-apis/drive/v2/files/query_by_token?token=node+%2F%3F%26%2B HTTP/1.1")
        );
        assert!(
            headers
                .lines()
                .any(|line| line == format!("authorization: Bearer {token}"))
        );
        assert!(body.is_empty());
    }
}
