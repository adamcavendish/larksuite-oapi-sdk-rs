use super::prelude::*;

// ── Directory ──

#[tokio::test]
async fn directory_department_create_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"department_id":"od-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = CreateDepartmentReqBody::default();
    let resp = client
        .directory()
        .department
        .create_by_query(
            &CreateDirectoryDepartmentQuery::new(&body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true)
                .tenant_id("tenant-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/directory/v1/departments?"));
    assert!(request.contains("employee_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("is_admin_role=true"));
    assert!(request.contains("tenant_id=tenant-1"));
}

#[tokio::test]
async fn directory_department_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = DirectorySearchDepartmentReqBody::default();
    let resp = client
        .directory()
        .department
        .search_by_query(
            &SearchDirectoryDepartmentQuery::new(&body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true)
                .tenant_id("tenant-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/directory/v1/departments/search?"));
    assert!(request.contains("employee_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("is_admin_role=true"));
    assert!(request.contains("tenant_id=tenant-1"));
}

#[tokio::test]
async fn directory_department_delete_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .directory()
        .department
        .delete_by_query(
            &DeleteDepartmentQuery::new("od-1")
                .is_admin_role(true)
                .employee_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("DELETE /open-apis/directory/v1/departments/od-1?"));
    assert!(request.contains("is_admin_role=true"));
    assert!(request.contains("employee_id_type=open_id"));
}
