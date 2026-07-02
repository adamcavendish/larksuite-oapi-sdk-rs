use super::prelude::*;

// ── Contact ──

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
async fn contact_unit_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"unitlist":[{"unit_id":"unit-1","name":"Engineering Unit"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListUnitQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .unit
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.unitlist.as_ref())
            .and_then(|items| items.first())
            .and_then(|unit| unit.name.as_deref()),
        Some("Engineering Unit")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/unit?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_unit_list_department_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"departmentlist":[{"unit_id":"unit-1","department_id":"od-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListDepartmentUnitQuery::new("unit-1")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .unit
        .list_department_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.departmentlist.as_ref())
            .and_then(|items| items.first())
            .and_then(|department| department.department_id.as_deref()),
        Some("od-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/unit/list_department?"));
    assert!(request.contains("unit_id=unit-1"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_group_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"group":{"group_id":"group-1","name":"Engineering"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .group
        .get_by_query(
            &GetContactGroupQuery::new("group-1")
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
            .and_then(|data| data.group.as_ref())
            .and_then(|group| group.name.as_deref()),
        Some("Engineering")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/group/group-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
}

#[tokio::test]
async fn contact_group_simplelist_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"grouplist":[{"group_id":"group-1","name":"Engineering"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SimplelistGroupQuery::new()
        .group_type(1)
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .group
        .simplelist_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.grouplist.as_ref())
            .and_then(|items| items.first())
            .and_then(|group| group.name.as_deref()),
        Some("Engineering")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/group/simplelist?"));
    assert!(request.contains("type=1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_group_member_belong_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"group_list":["group-1"],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = MemberBelongGroupQuery::new("u-1")
        .member_id_type("open_id")
        .group_type(1)
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .group
        .member_belong_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.group_list.as_ref())
            .and_then(|groups| groups.first())
            .map(String::as_str),
        Some("group-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/group/member_belong?"));
    assert!(request.contains("member_id=u-1"));
    assert!(request.contains("member_id_type=open_id"));
    assert!(request.contains("group_type=1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_group_member_simplelist_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"memberlist":[{"member_id":"u-1","member_type":"user","member_id_type":"open_id"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SimplelistGroupMemberQuery::new("group-1")
        .member_id_type("open_id")
        .member_type("user")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .group_member
        .simplelist_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.memberlist.as_ref())
            .and_then(|members| members.first())
            .and_then(|member| member.member_id.as_deref()),
        Some("u-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/group/group-1/member/simplelist?"));
    assert!(request.contains("member_id_type=open_id"));
    assert!(request.contains("member_type=user"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
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

#[tokio::test]
async fn contact_scope_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"department_ids":["od-1"],"user_ids":["u-1"],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListScopeQuery::new()
        .user_id_type("open_id")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .scope
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.department_ids.as_ref().unwrap()[0], "od-1");
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/scopes?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_employee_type_enum_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"enum_id":"enum-1","enum_value":"full_time"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListEmployeeTypeEnumQuery::new()
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .contact()
        .employee_type_enum
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(
        data.items.as_ref().unwrap()[0].enum_id.as_deref(),
        Some("enum-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/employee_type_enums?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_functional_role_member_batch_create_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"results":[{"user_id":"u-1","reason":0}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = BatchCreateFunctionalRoleMemberReqBody {
        members: Some(vec!["u-1".to_string()]),
    };
    let resp = client
        .contact()
        .functional_role_member
        .batch_create_by_query(
            &BatchCreateFunctionalRoleMemberQuery::new("role-1", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.results.as_ref())
            .and_then(|results| results.first())
            .and_then(|result| result.user_id.as_deref()),
        Some("u-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request
            .contains("POST /open-apis/contact/v3/functional_roles/role-1/members/batch_create?")
    );
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""members":["u-1"]"#));
}

#[tokio::test]
async fn contact_functional_role_member_batch_delete_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"result":[{"user_id":"u-1","reason":0}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = BatchDeleteFunctionalRoleMemberReqBody {
        members: Some(vec!["u-1".to_string()]),
    };
    let resp = client
        .contact()
        .functional_role_member
        .batch_delete_by_query(
            &BatchDeleteFunctionalRoleMemberQuery::new("role-1", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.result.as_ref())
            .and_then(|results| results.first())
            .and_then(|result| result.user_id.as_deref()),
        Some("u-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request
            .contains("PATCH /open-apis/contact/v3/functional_roles/role-1/members/batch_delete?")
    );
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""members":["u-1"]"#));
}

#[tokio::test]
async fn contact_functional_role_member_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"member":{"user_id":"u-1","scope_type":"department","department_ids":["od-1"]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .contact()
        .functional_role_member
        .get_by_query(
            &GetContactFunctionalRoleMemberQuery::new("role-1", "u-1")
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
            .and_then(|data| data.member.as_ref())
            .and_then(|member| member.user_id.as_deref()),
        Some("u-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/functional_roles/role-1/members/u-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
}

#[tokio::test]
async fn contact_functional_role_member_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"members":[{"user_id":"u-1","scope_type":"department","department_ids":["od-1"]}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListContactFunctionalRoleMemberQuery::new("role-1")
        .user_id_type("open_id")
        .department_id_type("department_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .functional_role_member
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.members.as_ref())
            .and_then(|members| members.first())
            .and_then(|member| member.scope_type.as_deref()),
        Some("department")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/functional_roles/role-1/members?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_functional_role_member_scopes_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"results":[{"user_id":"u-1","reason":0}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = ScopesFunctionalRoleMemberReqBody {
        members: Some(vec!["u-1".to_string()]),
        departments: Some(vec!["od-1".to_string()]),
    };
    let resp = client
        .contact()
        .functional_role_member
        .scopes_by_query(
            &ScopesFunctionalRoleMemberQuery::new("role-1", &body)
                .user_id_type("open_id")
                .department_id_type("department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("PATCH /open-apis/contact/v3/functional_roles/role-1/members/scopes?")
    );
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains(r#""members":["u-1"]"#));
    assert!(request.contains(r#""departments":["od-1"]"#));
}

#[tokio::test]
async fn contact_job_level_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"job_level_id":"level-1","name":"L1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListContactJobLevelQuery::new()
        .name("L1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .job_level
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|level| level.job_level_id.as_deref()),
        Some("level-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/job_levels?"));
    assert!(request.contains("name=L1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_job_family_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"job_family_id":"family-1","name":"Engineering"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListContactJobFamilyQuery::new()
        .name("Engineering")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .job_family
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|family| family.job_family_id.as_deref()),
        Some("family-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/job_families?"));
    assert!(request.contains("name=Engineering"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_job_title_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"job_title_id":"title-1","name":"Engineer"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListContactJobTitleQuery::new()
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .contact()
        .job_title
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|title| title.job_title_id.as_deref()),
        Some("title-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/job_titles?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_work_city_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"work_city_id":"city-1","name":"Shanghai"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListWorkCityQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .work_city
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|city| city.work_city_id.as_deref()),
        Some("city-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/work_cities?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_custom_attr_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"attr-1","name":"Nickname"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query =
        ListCustomAttrQuery::new().page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .custom_attr
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|item| item.get("id"))
            .and_then(|id| id.as_str()),
        Some("attr-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/custom_attrs?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
