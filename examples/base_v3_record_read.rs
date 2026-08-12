use larksuite_oapi_sdk_rs::service::base::v3::ListRecordQuery;
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
    let base_token = std::env::var("BASE_V3_BASE_TOKEN")?;
    let table_id = std::env::var("BASE_V3_TABLE_ID")?;

    let records = client
        .base_v3()
        .record
        .list(
            &ListRecordQuery::new(&base_token, &table_id)
                .offset(0)
                .limit(10),
            &option,
        )
        .await?;
    println!("{:#?}", records.data);
    Ok(())
}
