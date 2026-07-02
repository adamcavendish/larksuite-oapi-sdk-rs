use super::*;

// ── Contact ──

#[tokio::test]
async fn contact_get_user_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"user":{"open_id":"ou_1","name":"Alice"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .user
        .get("ou_1", Some("open_id"), None, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let user = resp.data.unwrap().user.unwrap();
    assert_eq!(user.name.as_deref(), Some("Alice"));
    assert_eq!(user.open_id.as_deref(), Some("ou_1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/users/ou_1?user_id_type=open_id"));
}

#[tokio::test]
async fn contact_list_department_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"name":"Engineering","department_id":"d-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .department
        .list(
            None,
            None,
            None,
            None,
            None,
            None,
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.as_ref().unwrap().len(), 1);
    assert_eq!(
        data.items.as_ref().unwrap()[0].name.as_deref(),
        Some("Engineering")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/departments"));
}
