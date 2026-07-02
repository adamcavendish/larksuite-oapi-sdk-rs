use super::prelude::*;

// ── Performance ──

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
