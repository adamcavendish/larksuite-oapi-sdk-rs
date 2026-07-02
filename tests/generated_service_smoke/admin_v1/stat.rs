use super::prelude::*;

// ── Admin v1 ──

#[tokio::test]
async fn admin_dept_stat_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"date":"2026-06-01","department_id":"od-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .dept_stat
        .get_by_query(
            &GetAdminDeptStatQuery::new(
                "open_department_id",
                "2026-06-01",
                "2026-06-30",
                "od-1",
                true,
            )
            .page_size(20)
            .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|stat| stat.department_id.as_deref()),
        Some("od-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/dept_stats?"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("start_date=2026-06-01"));
    assert!(request.contains("end_date=2026-06-30"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("contains_child_dept=true"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn admin_dept_stat_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"department_id":"od-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .dept_stat
        .list_by_query(
            &ListAdminDeptStatQuery::new(
                "open_department_id",
                "2026-06-01",
                "2026-06-30",
                "od-1",
                true,
            )
            .page_size(20)
            .page_token("next-page")
            .target_geo("cn")
            .with_product_version(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/admin_dept_stats?"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("start_date=2026-06-01"));
    assert!(request.contains("end_date=2026-06-30"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("contains_child_dept=true"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("target_geo=cn"));
    assert!(request.contains("with_product_version=true"));
}

#[tokio::test]
async fn admin_user_stat_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .admin()
        .user_stat
        .list_by_query(
            &ListAdminUserStatQuery::new("2026-06-01", "2026-06-30")
                .user_id_type("open_id")
                .department_id_type("open_department_id")
                .department_id("od-1")
                .user_id("ou-1")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/admin/v1/admin_user_stats?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("start_date=2026-06-01"));
    assert!(request.contains("end_date=2026-06-30"));
    assert!(request.contains("department_id=od-1"));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
