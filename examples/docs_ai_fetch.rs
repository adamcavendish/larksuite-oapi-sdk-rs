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
    let document_id = std::env::var("DOCS_AI_DOCUMENT_ID")?;

    let response = client
        .docs_ai()
        .document
        .fetch(
            &document_id,
            serde_json::json!({
                "format": "xml",
                "extra_param": r#"{"enable_user_cite_reference_map":true,"include_comments":true,"return_html5_block_data":true}"#,
            }),
            &option,
        )
        .await?;

    println!("Docs AI content: {:#?}", response.data);
    Ok(())
}
