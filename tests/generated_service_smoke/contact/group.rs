use super::prelude::*;

// Contact group smoke tests

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
