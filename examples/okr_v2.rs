use larksuite_oapi_sdk_rs::service::okr::v2::OkrPageQuery;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client =
        LarkClient::builder(std::env::var("APP_ID")?, std::env::var("APP_SECRET")?).build()?;
    let query = OkrPageQuery::new()
        .page_size(Some(20))
        .user_id_type(Some("open_id"));
    let response = client
        .okr_v2()
        .okr_category
        .list(&query, &RequestOption::default())
        .await?;
    for category in response.data.into_iter().flat_map(|data| data.items) {
        println!("{}", category.category_id.as_deref().unwrap_or_default());
    }
    Ok(())
}
