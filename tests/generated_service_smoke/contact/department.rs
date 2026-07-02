use super::prelude::*;

// Contact department smoke tests

#[tokio::test]
async fn contact_department_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-1","name":"Engineering"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListDepartmentQuery::new()
        .user_id_type("open_id")
        .department_id_type("department_id")
        .parent_department_id("od-parent")
        .fetch_child(true)
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .department
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.items.as_ref().unwrap()[0].name.as_deref(),
        Some("Engineering")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/departments?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("parent_department_id=od-parent"));
    assert!(request.contains("fetch_child=true"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_department_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"department":{"department_id":"od-1","name":"Engineering"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .department
        .get_by_query(
            &GetContactDepartmentQuery::new("od-1")
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
            .and_then(|data| data.department.as_ref())
            .and_then(|department| department.name.as_deref()),
        Some("Engineering")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/departments/od-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
}

#[tokio::test]
async fn contact_department_batch_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-1","name":"Engineering"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let department_ids = ["od-1", "od-2"];
    let resp = client
        .contact()
        .department
        .batch_by_query(
            &BatchContactDepartmentQuery::new(&department_ids)
                .department_id_type("department_id")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/departments/batch?"));
    assert!(request.contains("department_ids=od-1"));
    assert!(request.contains("department_ids=od-2"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn contact_department_unbind_chat_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = UnbindDepartmentChatReqBody {
        department_id: Some("od-1".to_string()),
    };
    let resp = client
        .contact()
        .department
        .unbind_department_chat_by_query(
            &UnbindDepartmentChatQuery::new(&body).department_id_type("department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/contact/v3/departments/unbind_department_chat?"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains(r#""department_id":"od-1""#));
}

#[tokio::test]
async fn contact_department_children_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-child","name":"Platform"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .department
        .children(
            "od-parent",
            Some("open_id"),
            Some("department_id"),
            Some(false),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.items.as_ref().unwrap()[0].department_id.as_deref(),
        Some("od-child")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/departments/od-parent/children?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("fetch_child=false"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_department_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-search","name":"Search Result"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SearchDepartmentQuery::new()
        .user_id_type("open_id")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let body = SearchDepartmentReqBody {
        query: Some("search".to_string()),
    };
    let resp = client
        .contact()
        .department
        .search_by_query(&query, &body, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/contact/v3/departments/search?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""query":"search""#));
}

#[tokio::test]
async fn contact_department_children_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ChildrenDepartmentQuery::new("od-parent")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .department
        .children_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/departments/od-parent/children?"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
