use super::prelude::*;

// ── OKR ──

#[tokio::test]
async fn okr_period_image_and_batch_by_query_smoke() {
    let period_list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"period-1","zh_name":"Q1"}]}}"#;
    let json_body = r#"{"code":0,"msg":"ok","data":{"id":"period-1"}}"#;
    let image_body = r#"{"code":0,"msg":"ok","data":{"image_key":"img-1"}}"#;
    let batch_body = r#"{"code":0,"msg":"ok","data":{"items":[{"okr_id":"okr-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, period_list_body),
        http_response(200, json_body),
        http_response(200, json_body),
        http_response(200, json_body),
        http_response(200, image_body),
        http_response(200, batch_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreatePeriodReqBody {
        period_rule_id: Some("rule-1".into()),
        start_month: Some("2026-01".into()),
    };
    let patch_body = PatchPeriodReqBody { status: Some(1) };
    let image_body = FormDataBuilder::new()
        .file("data", "okr.png", b"okr-image".to_vec(), Some("image/png"))
        .field("target_id", "okr-1")
        .field("target_type", "1")
        .build();
    let okr_ids = ["okr-1", "okr-2"];

    let period_list_resp = client
        .okr()
        .period
        .list_by_query(
            &ListOkrPeriodQuery::new()
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let create_resp = client
        .okr()
        .period
        .create_by_query(
            &CreateOkrPeriodQuery::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let patch_resp = client
        .okr()
        .period
        .patch_by_query(
            &PatchOkrPeriodQuery::new("period-1", &patch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let rule_resp = client
        .okr()
        .period_rule
        .list_by_query(&ListPeriodRuleQuery::new(), &RequestOption::default())
        .await
        .unwrap();
    let image_resp = client
        .okr()
        .image
        .upload_by_query(
            &UploadOkrImageQuery::new(&image_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let batch_resp = client
        .okr()
        .okr
        .batch_get_by_query(
            &BatchGetOkrQuery::new(&okr_ids)
                .user_id_type("open_id")
                .lang("zh_cn"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(period_list_resp.success());
    assert!(create_resp.success());
    assert!(patch_resp.success());
    assert!(rule_resp.success());
    assert!(image_resp.success());
    assert!(batch_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/okr/v1/periods?"));
    assert!(request.contains("POST /open-apis/okr/v1/periods"));
    assert!(request.contains("PATCH /open-apis/okr/v1/periods/period-1"));
    assert!(request.contains("GET /open-apis/okr/v1/period_rules"));
    assert!(request.contains("POST /open-apis/okr/v1/images/upload"));
    assert!(request.contains("GET /open-apis/okr/v1/okrs/batch_get?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("okr_ids=okr-1"));
    assert!(request.contains("okr_ids=okr-2"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("lang=zh_cn"));
    assert!(request.contains(r#""period_rule_id":"rule-1""#));
    assert!(request.contains(r#""start_month":"2026-01""#));
    assert!(request.contains(r#""status":1"#));
    assert!(request.contains("name=\"data\""));
    assert!(request.contains("okr-image"));
}

#[tokio::test]
async fn okr_progress_record_by_query_smoke() {
    let json_body = r#"{"code":0,"msg":"ok","data":{"progress_id":"progress-1"}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, json_body),
        http_response(200, empty_body),
        http_response(200, json_body),
        http_response(200, json_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateProgressRecordReqBody {
        target_id: Some("okr-1".into()),
        target_type: Some(1),
        progress_rate: Some(ProgressRateNew {
            percent: Some(50.0),
            ..Default::default()
        }),
        ..Default::default()
    };
    let update_body = UpdateProgressRecordReqBody {
        progress_rate: Some(ProgressRateNew {
            percent: Some(80.0),
            ..Default::default()
        }),
        ..Default::default()
    };
    let create_resp = client
        .okr()
        .progress_record
        .create_by_query(
            &CreateProgressRecordQuery::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete_resp = client
        .okr()
        .progress_record
        .delete_by_query(
            &DeleteProgressRecordQuery::new("progress-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let get_resp = client
        .okr()
        .progress_record
        .get_by_query(
            &GetProgressRecordQuery::new("progress-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_resp = client
        .okr()
        .progress_record
        .update_by_query(
            &UpdateProgressRecordQuery::new("progress-1", &update_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(delete_resp.success());
    assert!(get_resp.success());
    assert!(update_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/okr/v1/progress_records?"));
    assert!(request.contains("DELETE /open-apis/okr/v1/progress_records/progress-1"));
    assert!(request.contains("GET /open-apis/okr/v1/progress_records/progress-1?"));
    assert!(request.contains("PUT /open-apis/okr/v1/progress_records/progress-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""target_id":"okr-1""#));
    assert!(request.contains(r#""percent":50.0"#));
    assert!(request.contains(r#""percent":80.0"#));
}

#[tokio::test]
async fn okr_review_and_user_okr_by_query_smoke() {
    let review_body = r#"{"code":0,"msg":"ok","data":{"items":[{"review_id":"review-1"}]}}"#;
    let user_okr_body =
        r#"{"code":0,"msg":"ok","data":{"okr_list":[{"id":"okr-1","name":"Alice"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, review_body),
        http_response(200, user_okr_body),
    ])
    .await;

    let client = client_for(addr);
    let user_ids = ["ou-1", "ou-2"];
    let period_ids = ["period-1", "period-2"];
    let review_resp = client
        .okr()
        .review
        .query_by_query(
            &QueryReviewQuery::new(&user_ids)
                .period_ids(Some(&period_ids[..]))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let user_okr_resp = client
        .okr()
        .user_okr
        .get_by_query(
            &GetUserOkrQuery::new("ou-1")
                .user_id_type("open_id")
                .period_ids(vec!["period-1", "period-2"])
                .offset("0")
                .limit("20")
                .lang("zh_cn"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(review_resp.success());
    assert!(user_okr_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/okr/v1/reviews/query?"));
    assert!(request.contains("GET /open-apis/okr/v1/users/ou-1/okrs?"));
    assert!(request.contains("user_ids=ou-1"));
    assert!(request.contains("user_ids=ou-2"));
    assert!(request.contains("period_ids=period-1"));
    assert!(request.contains("period_ids=period-2"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("offset=0"));
    assert!(request.contains("limit=20"));
    assert!(request.contains("lang=zh_cn"));
}
