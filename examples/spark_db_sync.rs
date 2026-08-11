use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LarkClient::builder(std::env::var("APP_ID")?, std::env::var("APP_SECRET")?)
        .disable_token_cache()
        .build()?;
    let option = RequestOption {
        user_access_token: Some(std::env::var("USER_ACCESS_TOKEN")?),
        ..RequestOption::default()
    };
    let app_id = std::env::var("SPARK_APP_ID")?;

    // Supply the server-owned sync configuration as JSON. Preview is safe: it
    // resolves the configuration without creating a sync task.
    let config: serde_json::Value = serde_json::from_str(&std::env::var("SPARK_DB_SYNC_CONFIG")?)?;
    let preview = serde_json::json!({
        "config": config,
        "preview": true,
        "env": "dev",
    });
    let response = client
        .spark()
        .db_sync
        .create(&app_id, &preview, &option)
        .await?;
    println!("{:#?}", response.data);
    Ok(())
}
