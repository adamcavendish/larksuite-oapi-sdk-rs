//! List a shared creative app source directory and stream one source file.
//! Requires a user token with spark:app:read.
use larksuite_oapi_sdk_rs::service::spark::v1::PageSourceListQuery;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client =
        LarkClient::builder(std::env::var("APP_ID")?, std::env::var("APP_SECRET")?).build()?;
    let option = RequestOption {
        user_access_token: Some(std::env::var("USER_ACCESS_TOKEN")?),
        ..Default::default()
    };
    let meta_token = std::env::var("SPARK_META_TOKEN")?;
    let source_path = std::env::var("SPARK_SOURCE_PATH")?;
    let page = client
        .spark()
        .app
        .list_page_source(&PageSourceListQuery::new(&meta_token), &option)
        .await?;
    if let Some(data) = page.data {
        for item in data.items {
            println!("{} {} {}", item.r#type, item.path, item.size);
        }
    }

    let mut source = client
        .spark()
        .app
        .get_page_source(&meta_token, &source_path, &option)
        .await?;
    let mut stdout = std::io::stdout();
    while let Some(chunk) = source.body.next_chunk().await? {
        stdout.write_all(&chunk)?;
    }
    stdout.flush()?;
    Ok(())
}
