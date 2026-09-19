mod common;

use common::{http_response, mock_server_with_requests};
use larksuite_oapi_sdk_rs::service::spark::v1::{GetFileAppStorageQuery, ListFilesAppStorageQuery};
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};

fn client(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .max_retries(1)
        .build()
        .unwrap()
}

fn user() -> RequestOption {
    RequestOption {
        user_access_token: Some("storage-user".into()),
        ..Default::default()
    }
}

#[tokio::test]
async fn storage_management_uses_user_auth_and_exact_protocol_contracts() {
    let success = http_response(200, r#"{"code":0,"data":{}}"#);
    let partial_batch_result = serde_json::json!({
        "results": [
            {"status":"ok","file":{"path":"/folder/report.pdf"}},
            {"status":"failed","error_code":"400000008"}
        ]
    });
    let mut responses = vec![success; 7];
    responses.push(http_response(
        200,
        &format!(r#"{{"code":0,"data":{partial_batch_result}}}"#),
    ));
    let (addr, handle, requests) = mock_server_with_requests(responses).await;
    let client = client(addr);
    let storage = client.spark().app_storage;
    let option = user();

    storage
        .list_files(
            &ListFilesAppStorageQuery::new("cli_a/b")
                .page_size(20)
                .page_token("next token")
                .name("report.pdf")
                .path("/folder/report.pdf")
                .content_type("application/pdf")
                .size_gt(10)
                .size_lt(100)
                .uploaded_since("2026-09-01T00:00:00Z")
                .uploaded_until("2026-09-02T00:00:00Z"),
            &option,
        )
        .await
        .unwrap();
    storage
        .get_file(
            &GetFileAppStorageQuery::new("cli_a/b", "/folder/report.pdf"),
            &option,
        )
        .await
        .unwrap();
    storage
        .sign_file(
            "cli_a/b",
            &serde_json::json!({"path":"/folder/report.pdf","expires_in":60}),
            &option,
        )
        .await
        .unwrap();
    storage
        .sign_file(
            "cli_a/b",
            &serde_json::json!({"path":"/folder/report.pdf"}),
            &option,
        )
        .await
        .unwrap();
    storage.get_file_quota("cli_a/b", &option).await.unwrap();
    storage
        .pre_upload_file(
            "cli_a/b",
            &serde_json::json!({"file_name":"report.pdf","file_size":42,"content_type":"application/pdf"}),
            &option,
        )
        .await
        .unwrap();
    storage
        .upload_file_callback(
            "cli_a/b",
            &serde_json::json!({"upload_id":"upload-1","etag":"\"etag-1\""}),
            &option,
        )
        .await
        .unwrap();
    let batch_remove = storage
        .batch_remove_files(
            "cli_a/b",
            &serde_json::json!({"paths":["/folder/report.pdf","/missing.pdf"]}),
            &option,
        )
        .await
        .unwrap();
    assert_eq!(batch_remove.data.unwrap().as_value(), &partial_batch_result);

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 8);
    let request_lines = requests
        .iter()
        .map(|request| request.lines().next().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        request_lines,
        [
            "GET /open-apis/spark/v1/apps/cli_a%2Fb/storage/file_list?name=report.pdf&page_size=20&page_token=next+token&path=%2Ffolder%2Freport.pdf&size_gt=10&size_lt=100&type=application%2Fpdf&uploaded_since=2026-09-01T00%3A00%3A00Z&uploaded_until=2026-09-02T00%3A00%3A00Z HTTP/1.1",
            "GET /open-apis/spark/v1/apps/cli_a%2Fb/storage/file?path=%2Ffolder%2Freport.pdf HTTP/1.1",
            "POST /open-apis/spark/v1/apps/cli_a%2Fb/storage/file_sign HTTP/1.1",
            "POST /open-apis/spark/v1/apps/cli_a%2Fb/storage/file_sign HTTP/1.1",
            "GET /open-apis/spark/v1/apps/cli_a%2Fb/storage/file_quota HTTP/1.1",
            "POST /open-apis/spark/v1/apps/cli_a%2Fb/storage/file_pre_upload HTTP/1.1",
            "POST /open-apis/spark/v1/apps/cli_a%2Fb/storage/file_upload_callback HTTP/1.1",
            "POST /open-apis/spark/v1/apps/cli_a%2Fb/storage/file_batch_remove HTTP/1.1",
        ]
    );
    for request in requests.iter() {
        assert!(
            request
                .to_ascii_lowercase()
                .contains("authorization: bearer storage-user")
        );
    }
    for (request, expected_body) in requests.iter().skip(2).zip([
        Some(serde_json::json!({"path":"/folder/report.pdf","expires_in":60})),
        Some(serde_json::json!({"path":"/folder/report.pdf"})),
        None,
        Some(serde_json::json!({"file_name":"report.pdf","file_size":42,"content_type":"application/pdf"})),
        Some(serde_json::json!({"upload_id":"upload-1","etag":"\"etag-1\""})),
        Some(serde_json::json!({"paths":["/folder/report.pdf","/missing.pdf"]})),
    ]) {
        let body = request.split_once("\r\n\r\n").map(|(_, body)| body).unwrap_or("");
        match expected_body {
            Some(expected_body) => assert_eq!(serde_json::from_str::<serde_json::Value>(body).unwrap(), expected_body),
            None => assert!(body.is_empty()),
        }
    }
    handle.abort();
}

#[tokio::test]
async fn storage_management_preserves_api_errors_and_requires_user_auth() {
    let (addr, handle, requests) = mock_server_with_requests(vec![http_response(
        200,
        r#"{"code":99991672,"msg":"permission denied"}"#,
    )])
    .await;

    let error = client(addr)
        .spark()
        .app_storage
        .get_file_quota(
            "cli_a",
            &RequestOption {
                user_access_token: Some("storage-user".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        LarkError::Api(ref error) if error.code == 99991672
    ));

    let missing_user_token = client(addr)
        .spark()
        .app_storage
        .get_file_quota("cli_a", &RequestOption::default())
        .await
        .unwrap_err();
    assert!(!missing_user_token.to_string().is_empty());
    assert_eq!(requests.lock().unwrap().len(), 1);
    handle.abort();
}
