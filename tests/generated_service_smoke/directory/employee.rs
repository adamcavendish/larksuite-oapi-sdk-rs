use super::prelude::*;

// ── Directory ──

#[tokio::test]
async fn directory_employee_create_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"employee_id":"emp-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = CreateEmployeeReqBody::default();
    let resp = client
        .directory()
        .employee
        .create_by_query(
            &CreateDirectoryEmployeeQuery::new(&body)
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
    assert!(request.contains("POST /open-apis/directory/v1/employees?"));
    assert!(request.contains("employee_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("is_admin_role=true"));
    assert!(request.contains("tenant_id=tenant-1"));
}

#[tokio::test]
async fn directory_employee_mget_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"employee_id":"emp-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = MgetEmployeeReqBody::default();
    let resp = client
        .directory()
        .employee
        .mget_by_query(
            &MgetDirectoryEmployeeQuery::new(&body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/directory/v1/employees/mget?"));
    assert!(request.contains("employee_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("is_admin_role=true"));
}

#[tokio::test]
async fn directory_employee_patch_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = PatchEmployeeReqBody::default();
    let resp = client
        .directory()
        .employee
        .patch_by_query(
            &PatchDirectoryEmployeeQuery::new("emp-1", &body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/directory/v1/employees/emp-1?"));
    assert!(request.contains("employee_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("is_admin_role=true"));
}

#[tokio::test]
async fn directory_employee_action_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let delete_body = DeleteEmployeeReqBody::default();
    let regular_body = RegularEmployeeReqBody::default();
    let resurrect_body = ResurrectEmployeeReqBody::default();
    let resigned_body = ToBeResignedEmployeeReqBody::default();
    let delete_resp = client
        .directory()
        .employee
        .delete_by_query(
            &DeleteDirectoryEmployeeQuery::new("emp-1")
                .body(&delete_body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let regular_resp = client
        .directory()
        .employee
        .regular_by_query(
            &RegularDirectoryEmployeeQuery::new("emp-1")
                .body(&regular_body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let resurrect_resp = client
        .directory()
        .employee
        .resurrect_by_query(
            &ResurrectDirectoryEmployeeQuery::new("emp-1")
                .body(&resurrect_body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let resigned_resp = client
        .directory()
        .employee
        .to_be_resigned_by_query(
            &ToBeResignedDirectoryEmployeeQuery::new("emp-1", &resigned_body)
                .employee_id_type("open_id")
                .department_id_type("department_id")
                .is_admin_role(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(delete_resp.success());
    assert!(regular_resp.success());
    assert!(resurrect_resp.success());
    assert!(resigned_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("DELETE /open-apis/directory/v1/employees/emp-1?"));
    assert!(request.contains("PATCH /open-apis/directory/v1/employees/emp-1/regular?"));
    assert!(request.contains("POST /open-apis/directory/v1/employees/emp-1/resurrect?"));
    assert!(request.contains("PATCH /open-apis/directory/v1/employees/emp-1/to_be_resigned?"));
    assert!(request.contains("employee_id_type=open_id"));
    assert!(request.contains("department_id_type=department_id"));
    assert!(request.contains("is_admin_role=true"));
}
