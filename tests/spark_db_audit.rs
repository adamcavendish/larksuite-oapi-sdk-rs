mod common;

use common::{http_response, http_response_with_headers, mock_server_with_requests};
use larksuite_oapi_sdk_rs::service::spark::v1::{
    DbAuditEnv, DbAuditListQuery, DbAuditPrecondition, DbAuditRetention, DbAuditSetRequest,
    DbAuditSetting, DbAuditStatusQuery,
};
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};
use serde_json::{Value, json};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use url::Url;

fn client(addr: SocketAddr) -> LarkClient {
    LarkClient::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .max_retries(3)
        .build()
        .unwrap()
}

fn user() -> RequestOption {
    RequestOption {
        user_access_token: Some("audit-user".into()),
        ..Default::default()
    }
}

fn request_url(request: &str, expected_method: &str) -> Url {
    let mut request_line = request.lines().next().unwrap().split_whitespace();
    assert_eq!(request_line.next(), Some(expected_method));
    assert!(
        request
            .to_ascii_lowercase()
            .contains("authorization: bearer audit-user\r\n")
    );
    Url::parse(&format!("http://mock{}", request_line.next().unwrap())).unwrap()
}

#[tokio::test]
async fn audit_status_preserves_the_server_list_and_environment() {
    let (addr, handle, requests) = mock_server_with_requests(vec![
        http_response(
            200,
            r#"{"code":0,"data":{"items":[{"table":"orders","enabled":true,"retention":"30d","enabled_at":"2026-11-27T10:00:00Z"},{"table":"users","enabled":false}]}}"#,
        ),
        http_response(200, r#"{"code":0,"data":{"items":[]}}"#),
        http_response(200, r#"{"code":0,"data":{"items":[]}}"#),
    ])
    .await;
    let client = client(addr);
    let audit = client.spark().db_audit;
    let option = user();
    let query = DbAuditStatusQuery::new("app/a");

    let response = audit.get_status(&query, &option).await.unwrap();
    assert!(response.success());
    let items = response.data.unwrap().items;
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].table, "orders");
    assert!(items[0].enabled);
    assert_eq!(items[0].retention.as_deref(), Some("30d"));
    assert_eq!(items[0].enabled_at.as_deref(), Some("2026-11-27T10:00:00Z"));
    assert!(!items[1].enabled);
    assert!(items[1].retention.is_none());

    for env in [DbAuditEnv::Dev, DbAuditEnv::Online] {
        let response = audit
            .get_status(&query.env(env).table("orders & returns"), &option)
            .await
            .unwrap();
        assert!(response.data.unwrap().items.is_empty());
    }

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    for (request, env) in requests.iter().zip([None, Some("dev"), Some("online")]) {
        let url = request_url(request, "GET");
        assert_eq!(
            url.path(),
            "/open-apis/spark/v1/apps/app%2Fa/db/audit_status"
        );
        assert_eq!(
            url.query_pairs()
                .find(|(key, _)| key == "env")
                .as_ref()
                .map(|(_, value)| value.as_ref()),
            env
        );
        assert_eq!(
            url.query_pairs()
                .find(|(key, _)| key == "table")
                .as_ref()
                .map(|(_, value)| value.as_ref()),
            env.map(|_| "orders & returns")
        );
    }
    handle.abort();
}

#[tokio::test]
async fn audit_events_follow_the_cursor_and_decode_both_json_forms() {
    let (addr, handle, requests) = mock_server_with_requests(vec![
        http_response(
            200,
            r#"{"code":0,"data":{"items":[{"event_id":"01525","event_time":"2026-11-27T10:30:00Z","target_table":"orders","type":"UPDATE","operator":"{\"id\":\"7311\",\"name\":\"alice\"}","summary":"UPDATE 1 field","before":"{\"amount\":100}","after":{"amount":999}}],"has_more":true,"page_token":"cursor +/&"}}"#,
        ),
        http_response(
            200,
            r#"{"code":0,"data":{"items":[{"event_id":"01526","event_time":"2026-11-27T10:31:00Z","target_table":"users & guests","type":"DELETE","operator":{"id":"7312"},"before":"unchanged {","after":null},{"event_id":"01527","event_time":"2026-11-27T10:32:00Z","target_table":"orders","type":"INSERT","after":"[1,2]"}],"has_more":false}}"#,
        ),
    ])
    .await;
    let client = client(addr);
    let audit = client.spark().db_audit;
    let option = user();
    let query = DbAuditListQuery::new("app_x", &["orders", "users & guests"])
        .env(DbAuditEnv::Online)
        .since("2026-11-27T10:00:00Z")
        .until("2026-11-27T18:45:00+08:00");

    let first = audit
        .list_events(&query, &option)
        .await
        .unwrap()
        .data
        .unwrap();
    assert!(first.has_more);
    let event = &first.items[0];
    assert_eq!(event.event_id, "01525");
    assert_eq!(event.target_table, "orders");
    assert_eq!(event.event_type, "UPDATE");
    assert_eq!(event.operator.as_ref().unwrap().as_value()["id"], "7311");
    assert_eq!(
        event.before.as_ref().unwrap().as_value(),
        &json!({"amount":100})
    );
    assert_eq!(
        event.after.as_ref().unwrap().as_value(),
        &json!({"amount":999})
    );

    let second = audit
        .list_events(&query.page_token(first.page_token.as_deref()), &option)
        .await
        .unwrap()
        .data
        .unwrap();
    assert!(!second.has_more);
    assert!(second.page_token.is_none());
    assert_eq!(
        second.items[0].operator.as_ref().unwrap().as_value()["id"],
        "7312"
    );
    assert_eq!(
        second.items[0].before.as_ref().unwrap().as_str(),
        Some("unchanged {")
    );
    assert!(second.items[0].after.is_none());
    assert!(second.items[1].before.is_none());
    assert_eq!(
        second.items[1].after.as_ref().unwrap().as_value(),
        &json!([1, 2])
    );

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    for (request, cursor) in requests.iter().zip([None, Some("cursor +/&")]) {
        let url = request_url(request, "GET");
        assert_eq!(url.path(), "/open-apis/spark/v1/apps/app_x/db/audit_list");
        let pairs: std::collections::BTreeMap<_, _> = url.query_pairs().collect();
        assert_eq!(pairs["env"], "online");
        assert_eq!(pairs["tables"], "orders,users & guests");
        assert_eq!(pairs["since"], "2026-11-27T10:00:00Z");
        assert_eq!(pairs["until"], "2026-11-27T18:45:00+08:00");
        assert_eq!(pairs["page_size"], "20");
        assert_eq!(pairs.get("page_token").map(AsRef::as_ref), cursor);
    }
    handle.abort();
}

#[tokio::test]
async fn audit_set_sends_enable_and_disable_and_preserves_partial_status() {
    let (addr, handle, requests) = mock_server_with_requests(vec![
        http_response(
            200,
            r#"{"code":0,"data":{"status":{"table":"orders","enabled":true,"retention":"180d"}}}"#,
        ),
        http_response(200, r#"{"code":0,"data":{"status":{"enabled":false}}}"#),
        http_response(200, r#"{"code":0,"data":{}}"#),
    ])
    .await;
    let client = client(addr);
    let audit = client.spark().db_audit;
    let option = user();
    let enable = DbAuditSetRequest::new(
        "app_x",
        "orders",
        DbAuditSetting::Enable {
            retention: DbAuditRetention::Days180,
        },
    )
    .env(DbAuditEnv::Dev);
    let disable = DbAuditSetRequest::new("app_x", "orders", DbAuditSetting::Disable);

    let enabled = audit
        .set_status(&enable, &option)
        .await
        .unwrap()
        .data
        .unwrap()
        .status
        .unwrap();
    assert_eq!(enabled.table.as_deref(), Some("orders"));
    assert_eq!(enabled.enabled, Some(true));
    assert_eq!(enabled.retention.as_deref(), Some("180d"));
    let disabled = audit
        .set_status(&disable.env(DbAuditEnv::Online), &option)
        .await
        .unwrap()
        .data
        .unwrap()
        .status
        .unwrap();
    assert_eq!(disabled.enabled, Some(false));
    assert!(disabled.table.is_none());
    let absent = audit
        .set_status(&disable, &option)
        .await
        .unwrap()
        .data
        .unwrap();
    assert!(absent.status.is_none());

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    for (request, (query, body)) in requests.iter().zip([
        (
            Some("env=dev"),
            json!({"table":"orders","enabled":true,"retention":"180d"}),
        ),
        (
            Some("env=online"),
            json!({"table":"orders","enabled":false}),
        ),
        (None, json!({"table":"orders","enabled":false})),
    ]) {
        let url = request_url(request, "POST");
        assert_eq!(url.path(), "/open-apis/spark/v1/apps/app_x/db/audit_set");
        assert_eq!(url.query(), query);
        let actual: Value =
            serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(actual, body);
    }
    handle.abort();
}

#[tokio::test]
async fn audit_set_does_not_retry_an_ambiguous_timeout() {
    let (addr, handle, requests) = mock_server_with_requests(vec![
        http_response(504, r#"{"code":0}"#),
        http_response(200, r#"{"code":0,"data":{}}"#),
    ])
    .await;
    let error = client(addr)
        .spark()
        .db_audit
        .set_status(
            &DbAuditSetRequest::new("app_x", "orders", DbAuditSetting::Disable),
            &user(),
        )
        .await
        .unwrap_err();
    assert!(matches!(error, LarkError::ServerTimeout(_)));
    assert_eq!(requests.lock().unwrap().len(), 1);
    handle.abort();
}

#[tokio::test]
async fn audit_set_rejects_redirects_without_resending_the_write() {
    for status in [307, 308] {
        let (addr, handle, requests) = mock_server_with_requests(vec![
            http_response_with_headers(
                status,
                "Location: /other\r\nContent-Type: application/json\r\n",
                r#"{"code":0}"#,
            ),
            http_response(200, r#"{"code":0,"data":{}}"#),
        ])
        .await;
        let error = client(addr)
            .spark()
            .db_audit
            .set_status(
                &DbAuditSetRequest::new("app_x", "orders", DbAuditSetting::Disable),
                &user(),
            )
            .await
            .unwrap_err();

        assert!(matches!(error, LarkError::Http(aioduct::Error::Status(code)) if code == status));
        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        assert_eq!(
            request_url(&requests[0], "POST").path(),
            "/open-apis/spark/v1/apps/app_x/db/audit_set"
        );
        handle.abort();
    }
}

async fn read_request(connection: &mut BufReader<TcpStream>) -> String {
    let mut request = String::new();
    loop {
        let previous_length = request.len();
        assert_ne!(connection.read_line(&mut request).await.unwrap(), 0);
        if &request[previous_length..] == "\r\n" {
            break;
        }
    }
    let content_length = request
        .lines()
        .filter_map(|line| line.split_once(':'))
        .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
        .map_or(0, |(_, value)| value.trim().parse::<usize>().unwrap());
    let mut body = vec![0; content_length];
    connection.read_exact(&mut body).await.unwrap();
    request.push_str(std::str::from_utf8(&body).unwrap());
    request
}

#[tokio::test]
async fn audit_set_uses_a_fresh_connection_and_does_not_replay_after_disconnect() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (write_finished, finished) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut warm = BufReader::new(socket);
        let request = read_request(&mut warm).await;
        assert_eq!(
            request_url(&request, "GET").path(),
            "/open-apis/spark/v1/apps/app_x/db/audit_status"
        );
        warm.get_mut()
            .write_all(http_response(200, r#"{"code":0,"data":{"items":[]}}"#).as_bytes())
            .await
            .unwrap();

        // Keep the GET connection open while waiting for the audit write.
        let (socket, _) = listener.accept().await.unwrap();
        let mut write = BufReader::new(socket);
        let request = read_request(&mut write).await;
        assert_eq!(
            request_url(&request, "POST").path(),
            "/open-apis/spark/v1/apps/app_x/db/audit_set"
        );
        let body: Value = serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(body, json!({"table":"orders","enabled":false}));
        // The server saw the whole write, but never acknowledges it.
        drop(write);

        let replayed = tokio::select! {
            _ = finished => false,
            connection = listener.accept() => {
                let (mut socket, _) = connection.unwrap();
                socket.write_all(http_response(200, r#"{"code":0}"#).as_bytes()).await.unwrap();
                true
            }
        };
        drop(warm);
        replayed
    });

    let client = client(addr);
    let audit = client.spark().db_audit;
    let option = user();
    audit
        .get_status(&DbAuditStatusQuery::new("app_x"), &option)
        .await
        .unwrap();
    let request = DbAuditSetRequest::new("app_x", "orders", DbAuditSetting::Disable);
    let result = tokio::time::timeout(Duration::from_secs(5), audit.set_status(&request, &option))
        .await
        .expect("the audit write must open a fresh connection and finish");
    let _ = write_finished.send(());
    let replayed = tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .expect("the server must receive and record the audit write")
        .unwrap();
    assert!(!replayed);
    assert!(result.is_err());
}

#[tokio::test]
async fn audit_preconditions_match_the_gateway_code_and_message_prefix() {
    let cases = [
        (
            400_002_476,
            "k_dl_4000028：audit is disabled",
            None,
            Some(DbAuditPrecondition::AuditDisabled),
        ),
        (
            400_002_476,
            "k_dl_1600000: dev branch is missing",
            Some(""),
            Some(DbAuditPrecondition::BranchNotInitialized),
        ),
        (
            400_002_476,
            "database operation failed",
            Some("k_dl_4000028：audit is disabled"),
            Some(DbAuditPrecondition::AuditDisabled),
        ),
        (400_002_476, "k_dl_9999999: unfamiliar", None, None),
        (400_002_476, "table k_dl_4000028: missing", None, None),
        (400_002_476, "k_dl_4000028 without separator", None, None),
        (500_002_776, "k_dl_4000028: other envelope", None, None),
    ];
    let responses = cases
        .iter()
        .map(|(code, msg, rendered, _)| {
            let body =
                json!({"code":code,"msg":msg,"error":{"log_id":"audit-log-id","message":rendered}});
            http_response(200, &body.to_string())
        })
        .collect();
    let (addr, handle, requests) = mock_server_with_requests(responses).await;
    let client = client(addr);
    let audit = client.spark().db_audit;
    let query = DbAuditListQuery::new("app_x", &["orders"]).page_size(7);
    for (code, msg, rendered, expected) in cases {
        let error = audit.list_events(&query, &user()).await.unwrap_err();
        assert_eq!(DbAuditPrecondition::from_error(&error), expected);
        let LarkError::Api(error) = error else {
            panic!("expected a platform error");
        };
        assert_eq!(error.code, code);
        assert_eq!(error.msg, msg);
        let diagnostics = error.error.unwrap();
        assert_eq!(diagnostics.log_id.as_deref(), Some("audit-log-id"));
        assert_eq!(diagnostics.message.as_deref(), rendered);
    }
    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), cases.len());
    assert_eq!(
        request_url(&requests[0], "GET").query().unwrap(),
        "page_size=7&tables=orders"
    );
    handle.abort();
}

#[tokio::test]
async fn audit_rejects_missing_user_credentials_and_invalid_queries_without_io() {
    let (addr, handle, requests) =
        mock_server_with_requests(vec![http_response(200, r#"{"code":0}"#)]).await;
    let client = client(addr);
    let audit = client.spark().db_audit;
    let status = DbAuditStatusQuery::new("app_x");
    let list = DbAuditListQuery::new("app_x", &["orders"]);
    let set = DbAuditSetRequest::new("app_x", "orders", DbAuditSetting::Disable);
    for token in [None, Some(" \t ")] {
        let option = RequestOption {
            user_access_token: token.map(Into::into),
            tenant_access_token: Some("tenant-token".into()),
            ..Default::default()
        };
        assert!(matches!(
            audit.get_status(&status, &option).await,
            Err(LarkError::IllegalParam(_))
        ));
        assert!(matches!(
            audit.list_events(&list, &option).await,
            Err(LarkError::IllegalParam(_))
        ));
        assert!(matches!(
            audit.set_status(&set, &option).await,
            Err(LarkError::IllegalParam(_))
        ));
    }
    for query in [
        DbAuditListQuery::new("app_x", &[]),
        DbAuditListQuery::new("app_x", &["  "]),
        DbAuditListQuery::new("app_x", &["unencodable,name"]),
        list.page_size(0),
    ] {
        assert!(matches!(
            audit.list_events(&query, &user()).await,
            Err(LarkError::IllegalParam(_))
        ));
    }
    assert!(requests.lock().unwrap().is_empty());
    handle.abort();
}
