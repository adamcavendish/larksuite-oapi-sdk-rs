use super::prelude::*;

// ── Performance ──

#[tokio::test]
async fn performance_v2_metric_by_query_smoke() {
    let value_body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, value_body),
        http_response(200, value_body),
        http_response(200, value_body),
        http_response(200, value_body),
        http_response(200, value_body),
        http_response(200, value_body),
    ])
    .await;

    let client = client_for(addr);
    let query_body = serde_json::json!({"semester_id":"semester-1"});
    let import_body = serde_json::json!({"records":[{"user_id":"ou-1"}]});

    client
        .performance_v2()
        .metric_detail
        .import_by_query(
            &ImportMetricDetailV2Query::new(&import_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .performance_v2()
        .metric_detail
        .query_by_query(
            &QueryMetricDetailV2Query::new(&query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .performance_v2()
        .metric_field
        .query_by_query(
            &QueryMetricFieldV2Query::new(&query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .performance_v2()
        .metric_lib
        .query_by_query(
            &QueryMetricLibV2Query::new(&query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .performance_v2()
        .metric_template
        .query_by_query(
            &QueryMetricTemplateV2Query::new(&query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .performance_v2()
        .question
        .query_by_query(
            &QueryQuestionV2Query::new(&query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    for path in [
        "POST /open-apis/performance/v2/metric_details/import ",
        "POST /open-apis/performance/v2/metric_details/query ",
        "POST /open-apis/performance/v2/metric_fields/query ",
        "POST /open-apis/performance/v2/metric_libs/query ",
        "POST /open-apis/performance/v2/metric_templates/query ",
        "POST /open-apis/performance/v2/questions/query ",
    ] {
        assert!(request.contains(path));
    }
    assert!(request.contains(r#""semester_id":"semester-1""#));
    assert!(request.contains(r#""user_id":"ou-1""#));
}
