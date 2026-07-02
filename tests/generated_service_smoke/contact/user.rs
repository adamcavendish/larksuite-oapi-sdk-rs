use super::prelude::*;

// Contact user smoke tests

#[tokio::test]
async fn contact_user_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"u-1","name":"Ada"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListUserQuery::new()
        .user_id_type("open_id")
        .department_id_type("department_id")
        .department_id("od-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .user
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.as_ref().unwrap()[0].name.as_deref(), Some("Ada"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/users?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_user_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"user":{"user_id":"u-1","name":"Ada"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .user
        .get_by_query(
            &GetContactUserQuery::new("u-1")
                .user_id_type("open_id")
                .department_id_type("department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.user.as_ref())
            .and_then(|user| user.name.as_deref()),
        Some("Ada")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/users/u-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
}

#[tokio::test]
async fn contact_user_batch_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"u-1","name":"Ada"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let user_ids = ["u-1", "u-2"];
    let resp = client
        .contact()
        .user
        .batch_by_query(
            &BatchContactUserQuery::new(&user_ids)
                .user_id_type("open_id")
                .department_id_type("department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/users/batch?"));
    assert!(request.contains("user_ids=u-1"));
    assert!(request.contains("user_ids=u-2"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
}

#[tokio::test]
async fn contact_user_batch_get_id_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"user_list":[{"user_id":"u-1","email":"ada@example.com"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = BatchGetIdUserReqBody {
        emails: Some(vec!["ada@example.com".to_string()]),
        mobiles: None,
        include_resigned: Some(true),
    };
    let resp = client
        .contact()
        .user
        .batch_get_id_by_query(
            &BatchGetIdUserQuery::new(&body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/contact/v3/users/batch_get_id?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""emails":["ada@example.com"]"#));
    assert!(request.contains(r#""include_resigned":true"#));
}

#[tokio::test]
async fn contact_user_resurrect_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = ResurrectUserReqBody {
        departments: Some(vec![UserDepartmentInfo {
            department_id: Some("od-1".to_string()),
        }]),
        subscription_ids: Some(vec!["sub-1".to_string()]),
    };
    let resp = client
        .contact()
        .user
        .resurrect_by_query(
            &ResurrectUserQuery::new("u-1", &body)
                .user_id_type("open_id")
                .department_id_type("department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/contact/v3/users/u-1/resurrect?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains(r#""department_id":"od-1""#));
    assert!(request.contains(r#""subscription_ids":["sub-1"]"#));
}

#[tokio::test]
async fn contact_user_find_by_department_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"u-2","name":"Grace"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .user
        .find_by_department(
            "od-1",
            Some("open_id"),
            Some("department_id"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.items.as_ref().unwrap()[0].name.as_deref(),
        Some("Grace")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/users/find_by_department?"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_user_find_by_department_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = FindUserByDepartmentQuery::new("od-1")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .user
        .find_by_department_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/users/find_by_department?"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
