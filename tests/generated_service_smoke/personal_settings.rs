use super::*;

// ── Personal Settings ──

#[tokio::test]
async fn personal_settings_system_status_by_query_smoke() {
    let status_body = r#"{"code":0,"msg":"ok","data":{"system_status":{"system_status_id":"status-1","title":"Focus"}}}"#;
    let list_body = r#"{"code":0,"msg":"ok","data":{"items":[{"system_status_id":"status-1","title":"Focus"}],"has_more":false}}"#;
    let batch_body =
        r#"{"code":0,"msg":"ok","data":{"result_list":[{"user_id":"ou-1","success":true}]}}"#;
    let ok_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, status_body),
        http_response(200, list_body),
        http_response(200, batch_body),
        http_response(200, batch_body),
        http_response(200, status_body),
        http_response(200, ok_body),
    ])
    .await;

    let client = client_for(addr);
    let status = SystemStatus {
        title: Some("Focus".into()),
        priority: Some(1),
        ..Default::default()
    };
    let create_body = CreateSystemStatusReqBody {
        system_status: Some(status.clone()),
    };
    let patch_body = CreateSystemStatusReqBody {
        system_status: Some(SystemStatus {
            title: Some("Deep Work".into()),
            color: Some("blue".into()),
            ..Default::default()
        }),
    };
    let user_body = BatchOpenSystemStatusReqBody {
        user_list: Some(vec![serde_json::json!({"user_id":"ou-1"})]),
    };

    let create = client
        .personal_settings()
        .system_status
        .create_by_query(
            &CreateSystemStatusQuery::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .personal_settings()
        .system_status
        .list_by_query(
            &ListSystemStatusQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .personal_settings()
        .system_status
        .batch_open_by_query(
            &BatchOpenSystemStatusQuery::new("status-1", &user_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .personal_settings()
        .system_status
        .batch_close_by_query(
            &BatchCloseSystemStatusQuery::new("status-1", &user_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .personal_settings()
        .system_status
        .patch_by_query(
            &PatchSystemStatusQuery::new("status-1", &patch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete = client
        .personal_settings()
        .system_status
        .delete_by_query(
            &DeleteSystemStatusQuery::new("status-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create.success());
    assert!(delete.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/personal_settings/v1/system_statuses "));
    assert!(request.contains("GET /open-apis/personal_settings/v1/system_statuses?"));
    assert!(
        request
            .contains("POST /open-apis/personal_settings/v1/system_statuses/status-1/batch_open?")
    );
    assert!(
        request
            .contains("POST /open-apis/personal_settings/v1/system_statuses/status-1/batch_close?")
    );
    assert!(request.contains("PATCH /open-apis/personal_settings/v1/system_statuses/status-1 "));
    assert!(request.contains("DELETE /open-apis/personal_settings/v1/system_statuses/status-1 "));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""title":"Focus""#));
    assert!(request.contains(r#""title":"Deep Work""#));
    assert!(request.contains(r#""user_id":"ou-1""#));
}
