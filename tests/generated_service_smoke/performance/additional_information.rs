use super::prelude::*;

// ── Performance ──

#[tokio::test]
async fn performance_v2_activity_additional_by_query_smoke() {
    let value_body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let ok_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, value_body),
        http_response(200, value_body),
        http_response(200, value_body),
        http_response(200, ok_body),
        http_response(200, value_body),
    ])
    .await;

    let client = client_for(addr);
    let query_body = json_value!({"semester_id":"semester-1"});
    let import_body = json_value!({"records":[{"user_id":"ou-1"}]});
    let delete_body = json_value!({"additional_information_ids":["info-1"]});

    client
        .performance_v2()
        .activity
        .query_by_query(
            &QueryActivityV2Query::new(&query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .performance_v2()
        .additional_information
        .import_by_query(
            &ImportAdditionalInformationV2Query::new(&import_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .performance_v2()
        .additional_information
        .query_by_query(
            &QueryAdditionalInformationV2Query::new(&query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .performance_v2()
        .additional_informations_batch
        .delete_by_query(
            &DeleteAdditionalInformationsBatchV2Query::new(&delete_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .performance_v2()
        .indicator
        .query_by_query(
            &QueryIndicatorV2Query::new(&query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    for path in [
        "POST /open-apis/performance/v2/activity/query ",
        "POST /open-apis/performance/v2/additional_informations/import ",
        "POST /open-apis/performance/v2/additional_informations/query ",
        "DELETE /open-apis/performance/v2/additional_informations/batch ",
        "POST /open-apis/performance/v2/indicators/query ",
    ] {
        assert!(request.contains(path));
    }
    assert!(request.contains(r#""semester_id":"semester-1""#));
    assert!(request.contains(r#""user_id":"ou-1""#));
    assert!(request.contains(r#""additional_information_ids":["info-1"]"#));
}
