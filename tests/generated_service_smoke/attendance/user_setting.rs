use super::prelude::*;

// ── Attendance ──

#[tokio::test]
async fn attendance_user_setting_batch_get_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"user_settings":[{"user_id":"u-1","face_key":"face-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = BatchGetUserSettingReqBody {
        user_ids: Some(vec!["u-1".to_string()]),
    };
    let resp = client
        .attendance()
        .user_setting
        .batch_get_by_query(
            &BatchGetUserSettingQuery::new(&body, "employee_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.user_settings.first())
            .and_then(|setting| setting.face_key.as_deref()),
        Some("face-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/attendance/v1/user_settings/query?"));
    assert!(request.contains("employee_type=employee_id"));
    assert!(request.contains(r#""user_ids":["u-1"]"#));
}
