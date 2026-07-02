use super::prelude::*;

// Contact functional role member smoke tests

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
