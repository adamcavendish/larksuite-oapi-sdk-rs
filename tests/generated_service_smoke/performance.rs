use super::*;

// ── Performance ──

#[tokio::test]
async fn performance_activity_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"activity_id":"act-1","name":"Review"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .performance()
        .activity
        .list(Some(20), Some("next-page"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/performance/v1/activities?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn performance_activity_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"activity_id":"act-1","name":"Review"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListPerformanceActivityQuery::new()
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .performance()
        .activity
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].activity_id.as_deref(), Some("act-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/performance/v1/activities?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn performance_metric_tag_v2_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"tag_id":"tag-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .performance_v2()
        .metric_tag
        .list(Some(20), Some("next-page"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/performance/v2/metric_tags?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn performance_metric_tag_v2_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"tag_id":"tag-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let tag_ids = ["tag-1", "tag-2"];
    let query = ListPerformanceMetricTagV2Query::new()
        .tag_ids(tag_ids.as_slice())
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .performance_v2()
        .metric_tag
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0]["tag_id"].as_str(), Some("tag-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/performance/v2/metric_tags?"));
    assert!(request.contains("tag_ids=tag-1"));
    assert!(request.contains("tag_ids=tag-2"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

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
    let query_body = serde_json::json!({"semester_id":"semester-1"});
    let import_body = serde_json::json!({"records":[{"user_id":"ou-1"}]});
    let delete_body = serde_json::json!({"additional_information_ids":["info-1"]});

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

#[tokio::test]
async fn performance_v2_review_user_by_query_smoke() {
    let value_body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, value_body),
        http_response(200, value_body),
        http_response(200, value_body),
        http_response(200, value_body),
        http_response(200, value_body),
    ])
    .await;

    let client = client_for(addr);
    let query_body = serde_json::json!({"semester_id":"semester-1"});
    let write_body = serde_json::json!({"user_group_id":"group-1","user_ids":["ou-1"]});

    client
        .performance_v2()
        .review_data
        .query_by_query(
            &QueryReviewDataV2Query::new(&query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .performance_v2()
        .review_template
        .query_by_query(
            &QueryReviewTemplateV2Query::new(&query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .performance_v2()
        .reviewee
        .query_by_query(
            &QueryRevieweeV2Query::new(&query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .performance_v2()
        .user_group_user_rel
        .write_by_query(
            &WriteUserGroupUserRelV2Query::new(&write_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .performance_v2()
        .user_info
        .query_by_query(
            &QueryUserInfoV2Query::new(&query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    for path in [
        "POST /open-apis/performance/v2/review_datas/query ",
        "POST /open-apis/performance/v2/review_templates/query ",
        "POST /open-apis/performance/v2/reviewees/query ",
        "POST /open-apis/performance/v2/user_group_user_rels/write ",
        "POST /open-apis/performance/v2/user_info/query ",
    ] {
        assert!(request.contains(path));
    }
    assert!(request.contains(r#""semester_id":"semester-1""#));
    assert!(request.contains(r#""user_group_id":"group-1""#));
}
