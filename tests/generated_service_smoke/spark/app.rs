use super::prelude::*;

fn user_option() -> RequestOption {
    RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    }
}

// ── Spark v1 ──

#[tokio::test]
async fn spark_app_methods_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"app_id":"app-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let option = user_option();
    let app_body = json_value!({"name":"Demo"});
    client.spark().app.create(&app_body, &option).await.unwrap();
    client
        .spark()
        .app
        .get_app_visibility("app-1", &option)
        .await
        .unwrap();
    client
        .spark()
        .app
        .list(
            &SparkPageQuery::new().page_size(20).page_token("next"),
            &option,
        )
        .await
        .unwrap();
    client
        .spark()
        .app
        .patch("app-1", &app_body, &option)
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/spark/v1/apps "));
    assert!(request.contains(r#""name":"Demo""#));
    assert!(request.contains("GET /open-apis/spark/v1/apps/app-1/access-scope "));
    assert!(request.contains("GET /open-apis/spark/v1/apps?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next"));
    assert!(request.contains("PATCH /open-apis/spark/v1/apps/app-1 "));
}

#[tokio::test]
async fn spark_app_upload_and_command_methods_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let option = user_option();
    let command_body = json_value!({"sql":"select 1"});
    let scope_body = json_value!({"visibility":"all"});
    let icon_body = FormDataBuilder::new()
        .file(
            "file",
            "icon.png",
            b"icon-bytes".to_vec(),
            Some("image/png"),
        )
        .build();
    let html_body = FormDataBuilder::new()
        .file(
            "file",
            "index.html",
            b"<html></html>".to_vec(),
            Some("text/html"),
        )
        .build();

    client.spark().app.icon(icon_body, &option).await.unwrap();
    client
        .spark()
        .app
        .sql_commands("app-1", &command_body, &option)
        .await
        .unwrap();
    client
        .spark()
        .app
        .update_app_visibility("app-1", &scope_body, &option)
        .await
        .unwrap();
    client
        .spark()
        .app
        .upload_html_code_and_release("app-1", html_body, &option)
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/spark/v1/icon "));
    assert!(request.contains("filename=\"icon.png\""));
    assert!(request.contains("POST /open-apis/spark/v1/apps/app-1/sql_commands "));
    assert!(request.contains(r#""sql":"select 1""#));
    assert!(request.contains("PUT /open-apis/spark/v1/apps/app-1/access-scope "));
    assert!(request.contains("POST /open-apis/spark/v1/apps/app-1/upload_and_release_html_code "));
    assert!(request.contains("filename=\"index.html\""));
}
