mod common;
use common::{http_response, mock_server_with_requests};
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};
const KEY: &str = "b3d93913-e777-4968-a00c-afd0a1c7ce44";
#[tokio::test]
async fn feature_contract_and_no_replay() {
    let (addr, handle, requests) = mock_server_with_requests(vec![
        http_response(
            200,
            r#"{"code":0,"data":{"operation":{"state":"future"},"features":[]}}"#,
        ),
        http_response(504, "{}"),
        http_response(200, r#"{"code":0}"#),
    ])
    .await;
    let client = LarkClient::builder("a", "s")
        .base_url(format!("http://{addr}"))
        .max_retries(3)
        .build()
        .unwrap();
    let user = RequestOption {
        user_access_token: Some("user".into()),
        ..Default::default()
    };
    let spark = client.spark();
    let response = spark.app.features("app/a", Some(KEY), &user).await.unwrap();
    assert_eq!(
        response.data.unwrap().as_value()["operation"]["state"],
        "future"
    );
    assert!(matches!(
        spark
            .app
            .enable_feature("app/a", "data/base", KEY, &user)
            .await,
        Err(LarkError::ServerTimeout(_))
    ));
    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert!(requests[0].starts_with(&format!(
        "GET /open-apis/spark/v1/apps/app%2Fa/features?operation_ref={KEY} HTTP/1.1"
    )));
    assert!(
        requests[1].starts_with(
            "POST /open-apis/spark/v1/apps/app%2Fa/features/data%2Fbase/enable HTTP/1.1"
        )
    );
    for r in requests.iter() {
        assert!(r.to_lowercase().contains("authorization: bearer user"));
    }
    let body: serde_json::Value =
        serde_json::from_str(requests[1].split_once("\r\n\r\n").unwrap().1).unwrap();
    assert_eq!(body, serde_json::json!({"idempotent_key":KEY}));
    handle.abort();
}
#[tokio::test]
async fn rejects_invalid_keys_and_missing_user() {
    let client = LarkClient::builder("a", "s")
        .base_url("http://127.0.0.1:1")
        .build()
        .unwrap();
    let user = RequestOption {
        user_access_token: Some("u".into()),
        ..Default::default()
    };
    let spark = client.spark();
    assert!(matches!(
        spark.app.features("a", Some("bad"), &user).await,
        Err(LarkError::IllegalParam(_))
    ));
    assert!(matches!(
        spark
            .app
            .enable_feature("a", "database", KEY, &RequestOption::default())
            .await,
        Err(LarkError::IllegalParam(_))
    ));
}
