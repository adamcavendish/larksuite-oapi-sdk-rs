use super::prelude::*;

// ── Payroll ──

#[tokio::test]
async fn payroll_record_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"record_id":"record-1","employee_id":"emp-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .payroll()
        .payroll_record
        .list_by_query(
            &ListPayrollRecordQuery::new()
                .period("2026-06")
                .employee_id("emp-1")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.first())
            .and_then(|record| record.record_id.as_deref()),
        Some("record-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/payroll/v1/payroll_records?"));
    assert!(request.contains("period=2026-06"));
    assert!(request.contains("employee_id=emp-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn payroll_cost_allocation_detail_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"detail-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .payroll()
        .cost_allocation_detail
        .list_by_query(
            &ListCostAllocationDetailQuery::new()
                .cost_allocation_plan_id("plan-1")
                .pay_period("2026-06")
                .report_type(2)
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/payroll/v1/cost_allocation_details?"));
    assert!(request.contains("cost_allocation_plan_id=plan-1"));
    assert!(request.contains("pay_period=2026-06"));
    assert!(request.contains("report_type=2"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn payroll_datasource_record_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"record-1"}]}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"datasource_id":"ds-1"});
    let resp = client
        .payroll()
        .datasource_record
        .query_by_query(
            &QueryDatasourceRecordQuery::new(&body)
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let save_body = serde_json::json!({"datasource_id":"ds-1","record_id":"record-2"});
    client
        .payroll()
        .datasource_record
        .save_by_query(
            &SaveDatasourceRecordQuery::new(&save_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/payroll/v1/datasource_records/query?"));
    assert!(request.contains("POST /open-apis/payroll/v1/datasource_records/save "));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""datasource_id":"ds-1""#));
    assert!(request.contains(r#""record_id":"record-2""#));
}

#[tokio::test]
async fn payroll_payment_activity_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"activity_id":"act-1"}]}}"#;
    let archive_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, archive_body),
    ])
    .await;

    let client = client_for(addr);
    let statuses = [1, 3];
    let resp = client
        .payroll()
        .payment_activity
        .list_by_query(
            &ListPaymentActivityQuery::new()
                .pay_period_start_date("2026-06-01")
                .pay_period_end_date("2026-06-30")
                .statuses(&statuses[..])
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let archive_body = serde_json::json!({"payment_activity_id":"activity-1"});
    client
        .payroll()
        .payment_activity
        .archive_by_query(
            &ArchivePaymentActivityQuery::new(&archive_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/payroll/v1/payment_activitys?"));
    assert!(request.contains("POST /open-apis/payroll/v1/payment_activitys/archive "));
    assert!(request.contains("pay_period_start_date=2026-06-01"));
    assert!(request.contains("pay_period_end_date=2026-06-30"));
    assert!(request.contains("statuses=1"));
    assert!(request.contains("statuses=3"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains(r#""payment_activity_id":"activity-1""#));
}

#[tokio::test]
async fn payroll_payment_activity_detail_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"detail_id":"detail-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let acct_item_ids = ["acct-1", "acct-2"];
    let resp = client
        .payroll()
        .payment_activity_detail
        .list_by_query(
            &ListPaymentActivityDetailQuery::new("act-1")
                .page_index(2)
                .page_size(50)
                .include_segment_data(true)
                .acct_item_ids(&acct_item_ids[..]),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/payroll/v1/payment_activity_details?"));
    assert!(request.contains("activity_id=act-1"));
    assert!(request.contains("page_index=2"));
    assert!(request.contains("page_size=50"));
    assert!(request.contains("include_segment_data=true"));
    assert!(request.contains("acct_item_ids=acct-1"));
    assert!(request.contains("acct_item_ids=acct-2"));
}

#[tokio::test]
async fn payroll_payment_detail_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"payment_id":"pay-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = serde_json::json!({"payment_activity_id":"act-1"});
    let resp = client
        .payroll()
        .payment_detail
        .query_by_query(
            &QueryPaymentDetailQuery::new(&body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/payroll/v1/payment_detail/query"));
    assert!(request.contains(r#""payment_activity_id":"act-1""#));
}
