use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

#[tokio::test]
#[ignore = "requires a disposable Feishu Spark app, user token, and dev-only db-sync configuration"]
async fn feishu_spark_db_sync_live_lifecycle() {
    assert_eq!(
        required_env("FEISHU_SPARK_DB_SYNC_LIVE"),
        "1",
        "set FEISHU_SPARK_DB_SYNC_LIVE=1 only for the dedicated disposable target"
    );

    let client = LarkClient::builder(
        required_env("FEISHU_APP_ID"),
        required_env("FEISHU_APP_SECRET"),
    )
    .disable_token_cache()
    .build()
    .unwrap();
    let app_id = required_env("FEISHU_SPARK_DB_SYNC_APP_ID");
    let option = RequestOption {
        user_access_token: Some(required_env("FEISHU_SPARK_USER_ACCESS_TOKEN")),
        ..RequestOption::default()
    };

    let mut create_body = required_json_env("FEISHU_SPARK_DB_SYNC_CREATE_BODY");
    create_body["preview"] = serde_json::Value::Bool(true);
    let preview = client
        .spark()
        .db_sync
        .create(&app_id, &create_body, &option)
        .await
        .unwrap();
    assert!(
        preview.success(),
        "preview failed: {:?}",
        preview.code_error
    );

    create_body["preview"] = serde_json::Value::Bool(false);
    let created = client
        .spark()
        .db_sync
        .create(&app_id, &create_body, &option)
        .await
        .unwrap();
    assert!(created.success(), "create failed: {:?}", created.code_error);
    let task_id = created
        .data
        .as_ref()
        .and_then(|data| data.get("task_id"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .expect("create response must contain task_id for cleanup");

    let lifecycle = async {
        ensure_success(
            "get",
            client
                .spark()
                .db_sync
                .get(&app_id, &task_id, &option)
                .await?,
        )?;
        ensure_success(
            "list",
            client
                .spark()
                .db_sync
                .list(
                    &larksuite_oapi_sdk_rs::service::spark::v1::ListDbSyncQuery::new(&app_id)
                        .mode("streaming"),
                    &option,
                )
                .await?,
        )?;
        ensure_success(
            "enable",
            client
                .spark()
                .db_sync
                .enable(&app_id, &task_id, &option)
                .await?,
        )?;
        ensure_success(
            "disable",
            client
                .spark()
                .db_sync
                .disable(&app_id, &task_id, &option)
                .await?,
        )?;

        let mut update_body = required_json_env("FEISHU_SPARK_DB_SYNC_UPDATE_BODY");
        update_body["task_id"] = serde_json::Value::String(task_id.clone());
        ensure_success(
            "update",
            client
                .spark()
                .db_sync
                .update(&app_id, &update_body, &option)
                .await?,
        )
    }
    .await;

    let cleanup = client
        .spark()
        .db_sync
        .delete(&app_id, &task_id, &option)
        .await;
    match cleanup {
        Ok(response) if response.success() => {}
        Ok(response) => panic!("cleanup delete failed: {:?}", response.code_error),
        Err(error) => panic!("cleanup delete request failed: {error}"),
    }
    lifecycle.unwrap_or_else(|error| panic!("db-sync lifecycle failed: {error}"));
}

fn ensure_success(
    operation: &str,
    response: larksuite_oapi_sdk_rs::service::common::JsonResp,
) -> Result<(), larksuite_oapi_sdk_rs::LarkError> {
    if response.success() {
        Ok(())
    } else {
        Err(larksuite_oapi_sdk_rs::LarkError::IllegalParam(format!(
            "{operation} returned API error: {:?}",
            response.code_error
        )))
    }
}

fn required_env(name: &str) -> String {
    std::env::var(name)
        .unwrap_or_else(|_| panic!("{name} must be set for the live Spark db-sync test"))
}

fn required_json_env(name: &str) -> serde_json::Value {
    serde_json::from_str(&required_env(name))
        .unwrap_or_else(|error| panic!("{name} must contain a JSON object: {error}"))
}
