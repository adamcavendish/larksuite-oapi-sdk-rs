use larksuite_oapi_sdk_rs::service::base::v3::ListRecordQuery;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

#[tokio::test]
#[ignore = "requires a Base table and user token with read access"]
async fn feishu_base_v3_record_live_read() {
    assert_eq!(
        required_env("FEISHU_BASE_V3_RECORD_LIVE"),
        "1",
        "set FEISHU_BASE_V3_RECORD_LIVE=1 only for a read-only Base target"
    );

    let client = LarkClient::builder(
        required_env("FEISHU_APP_ID"),
        required_env("FEISHU_APP_SECRET"),
    )
    .disable_token_cache()
    .build()
    .unwrap();
    let option = RequestOption {
        user_access_token: Some(required_env("FEISHU_BASE_V3_USER_ACCESS_TOKEN")),
        ..RequestOption::default()
    };
    let base_token = required_env("FEISHU_BASE_V3_BASE_TOKEN");
    let table_id = required_env("FEISHU_BASE_V3_TABLE_ID");

    let list = client
        .base_v3()
        .record
        .list(
            &ListRecordQuery::new(&base_token, &table_id)
                .offset(0)
                .limit(1),
            &option,
        )
        .await
        .unwrap();
    assert!(list.success(), "list failed: {:?}", list.code_error);

    let search_body = std::env::var("FEISHU_BASE_V3_SEARCH_BODY")
        .map(|body| serde_json::from_str(&body).expect("search body must be valid JSON"))
        .unwrap_or_else(|_| serde_json::json!({"offset": 0, "limit": 1}));
    let search = client
        .base_v3()
        .record
        .search(&base_token, &table_id, search_body, &option)
        .await
        .unwrap();
    assert!(search.success(), "search failed: {:?}", search.code_error);
}

fn required_env(name: &str) -> String {
    std::env::var(name)
        .unwrap_or_else(|_| panic!("{name} must be set for the live Base v3 record test"))
}
