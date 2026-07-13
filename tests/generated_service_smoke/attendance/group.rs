use super::prelude::*;

// ── Attendance ──

#[tokio::test]
async fn attendance_group_create_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"group":{"group_id":"group-1","group_name":"Engineering"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = CreateOrUpdateGroupReqBody {
        group_id: Some("group-1".to_string()),
        operator_id: Some("u-1".to_string()),
        group: Some(AttendanceGroup {
            group_name: Some("Engineering".to_string()),
            ..Default::default()
        }),
    };
    let resp = client
        .attendance()
        .group
        .create_by_query(
            &CreateAttendanceGroupQuery::new(&body, "employee_id").dept_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.group.as_ref())
            .and_then(|group| group.group_name.as_deref()),
        Some("Engineering")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/attendance/v1/groups?"));
    assert!(request.contains("employee_type=employee_id"));
    assert!(request.contains("dept_type=open_department_id"));
    assert!(request.contains(r#""group_id":"group-1""#));
    assert!(request.contains(r#""group_name":"Engineering""#));
}

#[tokio::test]
async fn attendance_group_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"group":{"group_id":"group-1","group_name":"Engineering"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .attendance()
        .group
        .get_by_query(
            &GetAttendanceGroupQuery::new("group-1", "employee_id").dept_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.group.as_ref())
            .and_then(|group| group.group_id.as_deref()),
        Some("group-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/groups/group-1?"));
    assert!(request.contains("employee_type=employee_id"));
    assert!(request.contains("dept_type=open_department_id"));
}

#[tokio::test]
async fn attendance_group_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"group_list":[{"group_id":"group-1","group_name":"Engineering"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListAttendanceGroupQuery::new()
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .attendance()
        .group
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.group_list.first())
            .and_then(|group| group.get("group_id"))
            .and_then(|group_id| group_id.as_str()),
        Some("group-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/groups?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn attendance_group_list_user_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"users":[{"user_id":"u-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListUserGroupQuery::new("group-1")
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .attendance()
        .group
        .list_user_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.users.first())
            .and_then(|item| item.user_id.as_deref()),
        Some("u-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/groups/group-1/list_user?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
