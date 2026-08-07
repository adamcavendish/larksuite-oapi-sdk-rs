use larksuite_oapi_sdk_rs::service::bot::v4::SearchBotReqBody;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client =
        LarkClient::builder(std::env::var("APP_ID")?, std::env::var("APP_SECRET")?).build()?;
    let option = RequestOption {
        user_access_token: Some(std::env::var("USER_ACCESS_TOKEN")?),
        ..Default::default()
    };
    let body =
        SearchBotReqBody::new(std::env::var("BOT_QUERY").unwrap_or_else(|_| "calendar".into()));

    let response = client
        .bot_v4()
        .bot
        .search(&body, Some(20), None, Some("open_id"), &option)
        .await?;
    for bot in response.data.into_iter().flat_map(|data| data.items) {
        println!(
            "{}\t{}",
            bot.id.as_deref().unwrap_or_default(),
            bot.display_info.as_deref().unwrap_or_default()
        );
    }
    Ok(())
}
