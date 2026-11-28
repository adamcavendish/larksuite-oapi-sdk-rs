mod common;

use std::{collections::BTreeMap, net::SocketAddr};

use common::{http_response, http_response_with_headers, mock_server, mock_server_with_requests};
use larksuite_oapi_sdk_rs::service::spark::v1::{CreateAppReleaseRequest, ListAppReleaseQuery};
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};
use serde_json::{Value, json};

fn client(addr: SocketAddr) -> LarkClient {
    LarkClient::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .build()
        .unwrap()
}

fn user_option() -> RequestOption {
    RequestOption {
        user_access_token: Some("release-user".into()),
        ..Default::default()
    }
}

#[tokio::test]
async fn release_methods_use_versioned_routes_and_user_tokens() {
    let (addr, handle, requests) =
        mock_server_with_requests(vec![http_response(200, r#"{"code":0,"data":{}}"#)]).await;
    let client = client(addr);
    let spark = client.spark();
    let app = spark.app;
    let option = user_option();

    let body = CreateAppReleaseRequest::new()
        .branch("sprint/default")
        .apply_reason("Fix the release");
    app.create_release("app_a/b", &body, &option).await.unwrap();
    app.get_release("app_a/b", "7712345678901234567", &option)
        .await
        .unwrap();
    app.list_releases(
        "app_a/b",
        &ListAppReleaseQuery::new()
            .status(" finished ")
            .page_size(30)
            .page_token("cursor/+=& ?"),
        &option,
    )
    .await
    .unwrap();
    app.list_releases("app_a/b", &ListAppReleaseQuery::new().status(" "), &option)
        .await
        .unwrap();

    let tenant_option = RequestOption {
        tenant_access_token: Some("tenant".into()),
        ..Default::default()
    };
    assert!(matches!(
        app.get_release("app_a/b", "7712345678901234567", &tenant_option)
            .await,
        Err(LarkError::IllegalParam(_))
    ));

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 4);
    assert_eq!(
        requests[0].lines().next(),
        Some("POST /open-apis/spark/v1/apps/app_a%2Fb/releases HTTP/1.1")
    );
    assert_eq!(
        requests[1].lines().next(),
        Some("GET /open-apis/spark/v1/apps/app_a%2Fb/releases/7712345678901234567 HTTP/1.1")
    );
    assert_eq!(
        requests[3].lines().next(),
        Some("GET /open-apis/spark/v1/apps/app_a%2Fb/releases HTTP/1.1")
    );
    let body = requests[0].split_once("\r\n\r\n").unwrap().1;
    assert_eq!(
        serde_json::from_str::<Value>(body).unwrap(),
        json!({"branch":"sprint/default","apply_reason":"Fix the release"})
    );

    let mut request_line = requests[2].split_whitespace();
    assert_eq!(request_line.next(), Some("GET"));
    let target = request_line.next().unwrap();
    let url = url::Url::parse(&format!("http://localhost{target}")).unwrap();
    assert_eq!(url.path(), "/open-apis/spark/v1/apps/app_a%2Fb/releases");
    assert_eq!(
        url.query_pairs().into_owned().collect::<BTreeMap<_, _>>(),
        BTreeMap::from([
            ("page_size".into(), "30".into()),
            ("page_token".into(), "cursor/+=& ?".into()),
            ("status".into(), "finished".into()),
        ])
    );
    for request in requests.iter() {
        assert!(
            request
                .to_ascii_lowercase()
                .contains("authorization: bearer release-user\r\n")
        );
    }
    handle.abort();
}

#[tokio::test]
async fn create_release_sends_the_write_once_on_timeout_or_redirect() {
    for (status, outcome) in [
        (504, http_response(504, "{}")),
        (
            307,
            http_response_with_headers(
                307,
                "Location: /open-apis/spark/v1/apps/app/releases\r\n",
                "",
            ),
        ),
    ] {
        let (addr, handle, requests) = mock_server_with_requests(vec![
            outcome,
            http_response(
                200,
                r#"{"code":0,"data":{"release_id":"unexpected-replay"}}"#,
            ),
        ])
        .await;
        let error = client(addr)
            .spark()
            .app
            .create_release("app", &CreateAppReleaseRequest::new(), &user_option())
            .await
            .unwrap_err();

        match status {
            504 => assert!(matches!(error, LarkError::ServerTimeout(_))),
            _ => assert!(
                matches!(error, LarkError::Http(aioduct::Error::Status(code)) if code == status)
            ),
        }

        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        assert!(requests[0].starts_with("POST /open-apis/spark/v1/apps/app/releases HTTP/1.1"));
        assert_eq!(requests[0].split_once("\r\n\r\n").unwrap().1, "{}");
        handle.abort();
    }
}

#[tokio::test]
async fn nested_detail_prefers_outer_metadata_and_valid_approval_aliases() {
    let wire_body = json!({
        "code": 0,
        "data": {
            "release": {
                "release_id": 7712345678901234567_i64,
                "status": "pending",
                "created_at": 1788264000,
                "updated_at": "1788264060",
                "commit_id": "abc123",
                "online_url": "https://example.invalid/app",
                "current_node_info": {"current_status": "INNER"},
                "error_logs": [{"step":"inner"}],
                "future_field": true
            },
            "current_node_info": {
                "currentNode": "camel-node",
                "current_node": "review",
                "current_status": 7,
                "currentStatus": "PENDING",
                "created_at": "",
                "createdAt": 1788264060,
                "result": {"approval_url":"", "approvalURL":"https://example.invalid/approval"},
                "submitted_by": "invalid",
                "submittedBy": {"username":"Submitter", "email":"submitter@example.invalid", "openID":"ou_submitter"}
            },
            "error_logs": [{"step":"build", "error_log":"compile failed"}]
        }
    })
    .to_string();
    let (addr, handle) = mock_server(vec![http_response(200, &wire_body)]).await;
    let response = client(addr)
        .spark()
        .app
        .get_release("app", "7712345678901234567", &user_option())
        .await
        .unwrap();

    assert!(response.success());
    assert_eq!(response.api_resp.raw_body, wire_body.as_bytes());
    let detail = response.data.unwrap();
    assert!(detail.is_waiting_for_approval());
    assert_eq!(detail.release_id.as_deref(), Some("7712345678901234567"));
    assert_eq!(detail.created_at.unwrap().as_i64(), Some(1788264000));
    assert_eq!(detail.updated_at.unwrap().as_str(), Some("1788264060"));
    assert_eq!(detail.commit_id.as_deref(), Some("abc123"));
    assert_eq!(
        detail.online_url.as_deref(),
        Some("https://example.invalid/app")
    );
    let node = detail.current_node_info.unwrap();
    assert_eq!(node.current_node.as_deref(), Some("review"));
    assert_eq!(node.current_status.as_deref(), Some("PENDING"));
    assert_eq!(node.created_at.unwrap().as_i64(), Some(1788264060));
    assert_eq!(
        node.approval_url.as_deref(),
        Some("https://example.invalid/approval")
    );
    let submitter = node.submitted_by.unwrap();
    assert_eq!(submitter.username.as_deref(), Some("Submitter"));
    assert_eq!(submitter.open_id.as_deref(), Some("ou_submitter"));
    let logs = detail.error_logs.unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].step.as_deref(), Some("build"));
    assert_eq!(logs[0].error_log.as_deref(), Some("compile failed"));
    handle.abort();
}

#[tokio::test]
async fn flat_detail_distinguishes_active_approval_from_terminal_and_unknown_statuses() {
    let statuses = [
        ("publishing", true),
        ("finished", false),
        ("failed", false),
        ("future", false),
    ];
    let (addr, handle) = mock_server(
        statuses
            .iter()
            .map(|(status, _)| {
                http_response(
                    200,
                    &json!({
                        "code": 0,
                        "data": {
                            "release_id": "7",
                            "status": status,
                            "current_node_info": {
                                "current_status": "PENDING",
                                "currentStatus": "FINISHED",
                                "result": {"approval_url": "https://example.invalid/approval"},
                                "submitted_by": {"open_id": "ou_submitter"}
                            },
                            "error_logs": []
                        }
                    })
                    .to_string(),
                )
            })
            .collect(),
    )
    .await;
    let client = client(addr);
    for (status, pending) in statuses {
        let response = client
            .spark()
            .app
            .get_release("app", "7", &user_option())
            .await
            .unwrap();
        let detail = response.data.unwrap();
        assert_eq!(detail.status.as_deref(), Some(status));
        assert_eq!(detail.release_id.as_deref(), Some("7"));
        assert_eq!(detail.is_waiting_for_approval(), pending);
        assert!(detail.error_logs.unwrap().is_empty());
        assert_eq!(
            detail.current_node_info.unwrap().approval_url.as_deref(),
            Some("https://example.invalid/approval")
        );
    }
    handle.abort();
}

#[tokio::test]
async fn nested_detail_uses_inner_metadata_unless_outer_fields_are_present() {
    let release = json!({
        "status": "pending",
        "current_node_info": {"current_status":"PENDING"},
        "error_logs": [{"step":"build"}]
    });
    let responses = [
        json!({"release": release}),
        json!({"release": release, "current_node_info": null, "error_logs": null}),
        json!({"release": release, "current_node_info": "invalid", "error_logs": "invalid"}),
    ];
    let (addr, handle) = mock_server(
        responses
            .iter()
            .map(|data| http_response(200, &json!({"code": 0, "data": data}).to_string()))
            .collect(),
    )
    .await;
    let client = client(addr);

    for has_metadata in [true, false, false] {
        let response = client
            .spark()
            .app
            .get_release("app", "7", &user_option())
            .await
            .unwrap();
        let detail = response.data.unwrap();
        assert_eq!(detail.is_waiting_for_approval(), has_metadata);
        assert_eq!(detail.current_node_info.is_some(), has_metadata);
        assert_eq!(detail.error_logs.is_some(), has_metadata);
    }
    handle.abort();
}

#[tokio::test]
async fn detail_preserves_platform_errors_even_with_unexpected_data() {
    let (addr, handle) = mock_server(vec![
        http_response(200, r#"{"code":0,"data":null}"#),
        http_response(200, r#"{"code":0,"data":"unexpected shape"}"#),
        http_response(200, r#"{"code":9499,"msg":"Invalid release ID","error":{"message":"Use a valid release ID","log_id":"release-log"},"data":[]}"#),
    ]).await;
    let client = client(addr);
    for _ in 0..2 {
        let response = client
            .spark()
            .app
            .get_release("app", "7", &user_option())
            .await
            .unwrap();
        assert!(response.success());
        assert!(response.data.is_none());
    }
    let error = client
        .spark()
        .app
        .get_release("app", "7", &user_option())
        .await
        .unwrap_err();
    let LarkError::Api(error) = error else {
        panic!("expected platform error, got {error:?}");
    };
    assert_eq!(error.code, 9499);
    assert_eq!(error.effective_message(), "Use a valid release ID");
    assert_eq!(error.error.unwrap().log_id.as_deref(), Some("release-log"));
    handle.abort();
}
