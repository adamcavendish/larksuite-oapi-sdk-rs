mod common;

use std::collections::BTreeMap;

use common::{http_response, mock_server_with_requests};
use larksuite_oapi_sdk_rs::service::spark::v1::{ListAppQuery, SparkPageQuery};
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};
use serde_json::json;

fn client(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("app", "secret")
        .base_url(format!("http://{addr}"))
        .max_retries(3)
        .build()
        .unwrap()
}

fn user_option() -> RequestOption {
    RequestOption {
        user_access_token: Some("apps-user".into()),
        ..Default::default()
    }
}

#[tokio::test]
async fn delete_preserves_recovery_states_and_encodes_the_target() {
    let data = [
        Some(json!({"in_trash": true, "future": {"state": "retained"}})),
        Some(json!({"in_trash": false})),
        Some(json!({"future": "no recovery indication"})),
        None,
    ];
    let (addr, handle, requests) = mock_server_with_requests(
        data.iter()
            .map(|data| match data {
                Some(data) => http_response(200, &json!({"code": 0, "data": data}).to_string()),
                None => http_response(200, r#"{"code":0}"#),
            })
            .collect(),
    )
    .await;
    let client = client(addr);

    for expected in &data {
        let response = client
            .spark()
            .app
            .delete(" app_a/b?x#y ", &user_option())
            .await
            .unwrap();
        assert_eq!(response.api_resp.status_code, 200);
        assert_eq!(
            response.data.as_ref().map(|data| data.as_value()),
            expected.as_ref()
        );
    }

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), data.len());
    for request in requests.iter() {
        assert_eq!(
            request.lines().next().unwrap(),
            "DELETE /open-apis/spark/v1/apps/app_a%2Fb%3Fx%23y HTTP/1.1"
        );
        assert!(
            request
                .to_ascii_lowercase()
                .contains("authorization: bearer apps-user\r\n")
        );
        assert!(request.split_once("\r\n\r\n").unwrap().1.is_empty());
    }
    handle.abort();
}

#[tokio::test]
async fn delete_does_not_replay_an_ambiguous_timeout() {
    let (addr, handle, requests) = mock_server_with_requests(vec![
        http_response(504, "{}"),
        http_response(200, r#"{"code":0,"data":{"in_trash":true}}"#),
    ])
    .await;
    let error = client(addr)
        .spark()
        .app
        .delete("app_selected", &user_option())
        .await
        .unwrap_err();
    assert!(matches!(error, LarkError::ServerTimeout(_)));
    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert!(requests[0].starts_with("DELETE /open-apis/spark/v1/apps/app_selected HTTP/1.1"));
    handle.abort();
}

#[tokio::test]
async fn delete_preserves_business_errors_including_not_found() {
    let codes = [40300, 40400, 40002, 3340300, 3340400, 3340002];
    let (addr, handle, requests) = mock_server_with_requests(
        codes
            .iter()
            .map(|code| {
                http_response(
                    200,
                    &json!({
                        "code": code,
                        "msg": "original platform message",
                        "error": {"log_id": "platform-log"}
                    })
                    .to_string(),
                )
            })
            .collect(),
    )
    .await;
    let client = client(addr);
    for code in codes {
        match client
            .spark()
            .app
            .delete("app_selected", &user_option())
            .await
            .unwrap_err()
        {
            LarkError::Api(error) => {
                assert_eq!(error.code, code);
                assert_eq!(error.msg, "original platform message");
                assert_eq!(error.error.unwrap().log_id.as_deref(), Some("platform-log"));
            }
            error => panic!("unexpected error: {error:?}"),
        }
    }
    assert_eq!(requests.lock().unwrap().len(), codes.len());
    handle.abort();
}

#[tokio::test]
async fn list_combines_server_filters_with_pagination_and_keeps_get_retries() {
    let data = json!({
        "items": [{
            "app_id": "app_owned",
            "owner_id": "ou_owner",
            "icon_url": "https://example.invalid/icon.png",
            "created_at": "2026-11-05T00:00:00Z",
            "future": {"enabled": true}
        }],
        "page_token": "next",
        "has_more": true,
        "future": "preserved"
    });
    let (addr, handle, requests) = mock_server_with_requests(vec![
        http_response(504, "{}"),
        http_response(200, &json!({"code": 0, "data": data}).to_string()),
    ])
    .await;
    let response = client(addr)
        .spark()
        .app
        .list_with_query(
            &ListAppQuery::new()
                .owner_id(" \t ou_owner \n")
                .keyword("审批 & +/?#")
                .ownership("all")
                .app_type("full_stack")
                .page_size(50)
                .page_token("cursor/+=& ?"),
            &user_option(),
        )
        .await
        .unwrap();
    assert_eq!(response.data.unwrap().as_value(), &data);

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].lines().next(), requests[1].lines().next());
    for request in requests.iter() {
        let target = request
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap();
        let url = url::Url::parse(&format!("http://localhost{target}")).unwrap();
        assert_eq!(url.path(), "/open-apis/spark/v1/apps");
        assert_eq!(
            url.query_pairs().into_owned().collect::<BTreeMap<_, _>>(),
            BTreeMap::from([
                ("owner_id".into(), "ou_owner".into()),
                ("keyword".into(), "审批 & +/?#".into()),
                ("ownership".into(), "all".into()),
                ("app_type".into(), "full_stack".into()),
                ("page_size".into(), "50".into()),
                ("page_token".into(), "cursor/+=& ?".into()),
            ])
        );
        assert!(
            request
                .to_ascii_lowercase()
                .contains("authorization: bearer apps-user\r\n")
        );
        assert!(request.split_once("\r\n\r\n").unwrap().1.is_empty());
    }
    handle.abort();
}

#[tokio::test]
async fn list_omits_unset_filters_and_preserves_the_original_list_call() {
    let (addr, handle, requests) = mock_server_with_requests(vec![http_response(
        200,
        r#"{"code":0,"data":{"items":[],"has_more":false}}"#,
    )])
    .await;
    let client = client(addr);
    let user = user_option();
    let spark = client.spark();
    let response = spark
        .app
        .list_with_query(&ListAppQuery::new(), &user)
        .await
        .unwrap();
    assert_eq!(response.data.unwrap().as_value()["items"], json!([]));
    spark
        .app
        .list_with_query(
            &ListAppQuery::new()
                .owner_id(None)
                .page_size(10)
                .page_token("next"),
            &user,
        )
        .await
        .unwrap();
    spark
        .app
        .list(
            &SparkPageQuery::new().page_size(10).page_token("next"),
            &user,
        )
        .await
        .unwrap();
    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    assert_eq!(
        requests[0].lines().next().unwrap(),
        "GET /open-apis/spark/v1/apps HTTP/1.1"
    );
    assert_eq!(requests[1].lines().next(), requests[2].lines().next());
    assert!(requests[1].contains("page_size=10"));
    assert!(requests[1].contains("page_token=next"));
    assert!(requests.iter().all(|request| !request.contains("owner_id")));
    handle.abort();
}

#[tokio::test]
async fn app_operations_require_an_explicit_nonempty_user_token_without_io() {
    let (addr, handle, requests) =
        mock_server_with_requests(vec![http_response(200, r#"{"code":0}"#)]).await;
    let client = client(addr);
    for token in [None, Some(""), Some(" \t ")] {
        let option = RequestOption {
            user_access_token: token.map(Into::into),
            app_access_token: Some("must-not-use-app-token".into()),
            tenant_access_token: Some("must-not-use-tenant-token".into()),
            ..Default::default()
        };
        let spark = client.spark();
        assert!(matches!(
            spark.app.delete("app_selected", &option).await,
            Err(LarkError::IllegalParam(_))
        ));
        assert!(matches!(
            spark
                .app
                .list_with_query(&ListAppQuery::new(), &option)
                .await,
            Err(LarkError::IllegalParam(_))
        ));
    }
    assert!(requests.lock().unwrap().is_empty());
    handle.abort();
}

#[tokio::test]
async fn explicit_invalid_owners_and_app_ids_are_rejected_without_io() {
    let (addr, handle, requests) =
        mock_server_with_requests(vec![http_response(200, r#"{"code":0}"#)]).await;
    let client = client(addr);
    let user = user_option();
    let spark = client.spark();
    for owner in ["", " \t ", "12345", "on_owner"] {
        assert!(matches!(
            spark
                .app
                .list_with_query(&ListAppQuery::new().owner_id(owner), &user)
                .await,
            Err(LarkError::IllegalParam(_))
        ));
    }
    for app_id in ["", " \t ", "meta_token", "app", "APP_selected"] {
        assert!(matches!(
            spark.app.delete(app_id, &user).await,
            Err(LarkError::IllegalParam(_))
        ));
    }
    assert!(requests.lock().unwrap().is_empty());
    handle.abort();
}
