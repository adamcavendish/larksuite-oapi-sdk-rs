use larksuite_oapi_sdk_rs::card::template::TemplateMessage;
use larksuite_oapi_sdk_rs::service::im::v1::CreateMessageReqBody;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};
use serde::Serialize;

#[derive(Serialize)]
struct TemplateVariables {
    title: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let chat_id = std::env::var("CHAT_ID").expect("CHAT_ID env var required");
    let template_id = std::env::var("CARD_TEMPLATE_ID").expect("CARD_TEMPLATE_ID env var required");

    let template = TemplateMessage::with_variables(
        template_id,
        TemplateVariables {
            title: std::env::var("CARD_TEMPLATE_TITLE").unwrap_or_else(|_| "Release health".into()),
        },
    )?;
    let body = CreateMessageReqBody::interactive_card(chat_id, &template)?;

    let client = LarkClient::builder(app_id, app_secret).build()?;
    let response = client
        .im()
        .message
        .create("chat_id", &body, &RequestOption::default())
        .await?;

    println!(
        "sent: message_id={}",
        response
            .data
            .as_ref()
            .and_then(|data| data.message_id.as_deref())
            .unwrap_or("")
    );
    Ok(())
}
