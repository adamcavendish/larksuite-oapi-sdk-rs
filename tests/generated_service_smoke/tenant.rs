use super::prelude::*;

// ── Tenant ──

#[tokio::test]
async fn tenant_v2_by_query_smoke() {
    let tenant_body = r#"{"code":0,"msg":"ok","data":{"tenant":{"name":"Acme","tenant_key":"tenant-1","domain":"acme"}}}"#;
    let assign_body = r#"{"code":0,"msg":"ok","data":{"assign_info_list":[{"subscription_id":"sub-1","license_plan_key":"plan-1","product_name":"Base"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, tenant_body),
        http_response(200, assign_body),
    ])
    .await;

    let client = client_for(addr);
    let tenant_resp = client
        .tenant()
        .tenant
        .query_by_query(&QueryTenantQuery::new(), &RequestOption::default())
        .await
        .unwrap();
    let assign_resp = client
        .tenant()
        .product_assign_info
        .query_by_query(
            &QueryTenantProductAssignInfoQuery::new(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(tenant_resp.success());
    assert!(assign_resp.success());
    assert_eq!(
        tenant_resp
            .data
            .as_ref()
            .and_then(|data| data.tenant.as_ref())
            .and_then(|tenant| tenant.tenant_key.as_deref()),
        Some("tenant-1")
    );
    assert_eq!(
        assign_resp
            .data
            .as_ref()
            .and_then(|data| data.assign_info_list.as_ref())
            .and_then(|items| items.first())
            .and_then(|item| item.subscription_id.as_deref()),
        Some("sub-1")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/tenant/v2/tenant/query "));
    assert!(request.contains("GET /open-apis/tenant/v2/tenant/assign_info_list/query "));
}
